use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
    web::Data,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tracing::warn;
use uuid::Uuid;

use crate::domain::model::models::Utilisateur;
use crate::domain::error::MyError;
use crate::ports::sortie::utilisateurs::UtilisateurSortie;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub role: String, // User role
    pub exp: usize,  // Expiration
}

pub fn auth_middleware<S>(
    repository: Arc<Box<dyn UtilisateurSortie>>,
) -> impl Transform<
    S,
    ServiceRequest,
    Response = ServiceResponse,
    Error = Error,
    InitError = (),
> where
    S: Service<
            ServiceRequest,
            Response = ServiceResponse,
            Error = Error,
        > + 'static + std::clone::Clone,
    S::Future: 'static,
{
    AuthMiddleware {
        repository: Data::new(repository),
        _phantom: std::marker::PhantomData,
    }
}

pub struct AuthMiddleware<S> {
    repository: Data<Arc<Box<dyn UtilisateurSortie>>>,
    _phantom: std::marker::PhantomData<S>,
}

impl<S> Transform<S, ServiceRequest> for AuthMiddleware<S>
where
    S: Service<
            ServiceRequest,
            Response = ServiceResponse,
            Error = Error,
        > + 'static + std::clone::Clone,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service,
            repository: self.repository.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    repository: Data<Arc<Box<dyn UtilisateurSortie>>>,
}

impl<S> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<
            ServiceRequest,
            Response = ServiceResponse,
            Error = Error,
        > + 'static + Clone, // Added Clone bound
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let repository = self.repository.clone();
        let service = self.service.clone(); // Clone service to own it
        let auth_result = async move {
            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|header| header.to_str().ok())
                .ok_or_else(|| MyError::Unauthorized("Missing Authorization header".to_string()))?;

            let token = auth_header
                .strip_prefix("Bearer ")
                .ok_or_else(|| MyError::Unauthorized("Invalid Authorization header format".to_string()))?;

            let jwt_secret = env::var("JWT_SECRET").map_err(|_| MyError::Custom("JWT_SECRET not set".to_string()))?;

            let token_data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(jwt_secret.as_ref()),
                &Validation::new(jsonwebtoken::Algorithm::HS256),
            )
            .map_err(|e| MyError::Unauthorized(format!("Invalid token: {}", e)))?;

            if !["utilisateur", "gerant"].contains(&token_data.claims.role.as_str()) {
                return Err(MyError::Unauthorized("Invalid role".to_string()));
            }

            let user_id = Uuid::parse_str(&token_data.claims.sub)
                .map_err(|_| MyError::Unauthorized("Invalid user ID".to_string()))?;

            let utilisateur = repository
                .obtenir_par_id(user_id)
                .await?
                .ok_or_else(|| MyError::Unauthorized("User not found".to_string()))?;

            if utilisateur.role != token_data.claims.role {
                return Err(MyError::Unauthorized("Role mismatch".to_string()));
            }

            let path = req.path();
            let restricted_paths = ["/utilisateurs", "/utilisateurs/{id}/role"];
            if restricted_paths.iter().any(|p| path.starts_with(p)) && token_data.claims.role != "gerant" {
                warn!("Accès non autorisé à {} par utilisateur avec rôle {}", path, token_data.claims.role);
                return Err(MyError::Unauthorized("Seul un gérant peut accéder à ce service".to_string()));
            }

            req.extensions_mut().insert(utilisateur);
            req.extensions_mut().insert(token_data.claims.role);

            Ok(req)
        };

        Box::pin(async move {
            let req = auth_result.await.map_err(|e| actix_web::Error::from(e))?;
            service.call(req).await // Use cloned service
        })
    }
}

pub async fn generate_jwt(user: &Utilisateur) -> Result<String, MyError> {
    let jwt_secret = env::var("JWT_SECRET").map_err(|_| MyError::Custom("JWT_SECRET not set".to_string()))?;

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        role: user.role.clone(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .map_err(|_| MyError::Custom("Failed to generate token".to_string()))?;

    Ok(token)
}
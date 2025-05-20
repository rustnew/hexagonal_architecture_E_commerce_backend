use axum::{
    extract::{Extension, Path, Json},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Router,
};
use crate::adaptateurs::entrer::auth::{auth_middleware, generate_jwt};
use crate::domain::model::models::Utilisateur;
use crate::domain::error::MyError;
use crate::ports::entrer::utilisateurs::UtilisateurEntree;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct UtilisateurRequest {
    #[validate(email(message = "L'email doit être valide"))]
    pub email: String,
    #[validate(length(min = 8, message = "Le mot de passe doit avoir au moins 8 caractères"))]
    pub mot_de_passe: String,
    #[validate(length(min = 1, message = "Le prénom ne peut pas être vide"))]
    pub prenom: String,
    #[validate(length(min = 1, message = "Le nom ne peut pas être vide"))]
    pub nom: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub mot_de_passe: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateRoleRequest {
    #[validate(length(min = 1, message = "Le rôle ne peut pas être vide"))]
    pub role: String,
}
pub fn init_routes(service: Arc<Box<dyn UtilisateurEntree>>) -> Router {
    Router::new()
        .route("/utilisateurs", post(create_utilisateur))
        .route("/login", post(login))
        .route("/utilisateurs", get(get_all_utilisateurs))
        .route("/utilisateurs/:id", get(get_utilisateur))
        .route("/utilisateurs/:id", put(update_utilisateur))
        .route("/utilisateurs/:id/role", put(update_user_role))
        .route("/utilisateurs/:id", delete(delete_utilisateur))
        .route_layer(middleware::from_fn_with_state(service.clone(), auth_middleware))
        .layer(Extension(service))
}

async fn create_utilisateur(
    Extension(service): Extension<Arc<Box<dyn UtilisateurEntree>>>,
    Json(req): Json<UtilisateurRequest>,
) -> Result<impl IntoResponse, MyError> {
    req.validate().map_err(|e| MyError::Validation(e.to_string()))?;
    let utilisateur = Utilisateur {
        id: Uuid::new_v4(),
        email: req.email,
        mot_de_passe: req.mot_de_passe,
        prenom: req.prenom,
        nom: req.nom,
        date_creation: chrono::Utc::now(),
        role: "utilisateur".to_string(),
    };
    let utilisateur = service.creer_utilisateur(utilisateur).await?;
    Ok((StatusCode::CREATED, Json(utilisateur)))
}


async fn login(
    Extension(service): Extension<Arc<Box<dyn UtilisateurEntree>>>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, MyError> {
    let utilisateur = service.authentifier(&req.email, &req.mot_de_passe).await?;
    let token = generate_jwt(&utilisateur).await?;
    Ok((StatusCode::OK, Json(serde_json::json!({ "token": token }))))
}


async fn get_all_utilisateurs(
    Extension(service): Extension<Arc<Box<dyn UtilisateurEntree>>>,
    Extension(role): Extension<String>,
) -> Result<impl IntoResponse, MyError> {
    let utilisateurs = service.lister_utilisateurs(&role).await?;
    Ok((StatusCode::OK, Json(utilisateurs)))
}

async fn get_utilisateur(
    Path(id): Path<Uuid>,
    Extension(service): Extension<Arc<Box<dyn UtilisateurEntree>>>,
    Extension(current_user): Extension<Utilisateur>,
    Extension(role): Extension<String>,
) -> Result<impl IntoResponse, MyError> {
    let utilisateur = service
        .obtenir_utilisateur(id, &role, current_user.id)
        .await?
        .ok_or_else(|| MyError::NotFound("Utilisateur non trouvé".to_string()))?;
    Ok((StatusCode::OK, Json(utilisateur)))
}


async fn update_utilisateur(
    Path(id): Path<Uuid>,
    Json(req): Json<UtilisateurRequest>,
    Extension(service): Extension<Arc<Box<dyn UtilisateurEntree>>>,
    Extension(current_user): Extension<Utilisateur>,
    Extension(role): Extension<String>,
) -> Result<impl IntoResponse, MyError> {
    req.validate().map_err(|e| MyError::Validation(e.to_string()))?;
    let utilisateur = Utilisateur {
        id,
        email: req.email,
        mot_de_passe: req.mot_de_passe,
        prenom: req.prenom,
        nom: req.nom,
        date_creation: current_user.date_creation,
        role: current_user.role,
    };
    let utilisateur = service.mettre_a_jour_utilisateur(utilisateur, &role, current_user.id).await?;
    Ok((StatusCode::OK, Json(utilisateur)))
}

async fn update_user_role(
    Path(id): Path<Uuid>,
    Extension(service): Extension<Arc<Box<dyn UtilisateurEntree>>>,
    Extension(role): Extension<String>,
    Json(req): Json<UpdateRoleRequest>,
) -> Result<impl IntoResponse, MyError> {
    req.validate().map_err(|e| MyError::Validation(e.to_string()))?;
    let utilisateur = service.changer_role(id, &req.role, &role).await?;
    Ok((StatusCode::OK, Json(utilisateur)))
}


async fn delete_utilisateur(
    Path(id): Path<Uuid>,
    Extension(service): Extension<Arc<Box<dyn UtilisateurEntree>>>,
    Extension(role): Extension<String>,
) -> Result<impl IntoResponse, MyError> {
    service.supprimer_utilisateur(id, &role).await?;
    Ok(StatusCode::NO_CONTENT)
}
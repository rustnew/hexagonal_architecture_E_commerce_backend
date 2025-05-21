use actix_web::{
    get, post, put, delete,
    web::{self, Data, Json, Path},
    HttpResponse, Responder, Scope,
};
use crate::adaptateurs::entrer::auth::{auth_middleware, generate_jwt};
use crate::domain::model::models::Utilisateur;
use crate::domain::error::MyError;
use crate::ports::entrer::utilisateurs::UtilisateurEntree;
use crate::ports::sortie::utilisateurs::UtilisateurSortie;
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


#[post("/utilisateurs")]
pub async fn create_utilisateur(
    service: Data<Arc<Box<dyn UtilisateurEntree>>>,
    req: Json<UtilisateurRequest>,
) -> Result<impl Responder, MyError> {
    req.validate().map_err(|e| MyError::Validation(e.to_string()))?;
    let utilisateur = Utilisateur {
        id: Uuid::new_v4(),
        email: req.email.clone(),
        mot_de_passe: req.mot_de_passe.clone(),
        prenom: req.prenom.clone(),
        nom: req.nom.clone(),
        date_creation: chrono::Utc::now(),
        role: "utilisateur".to_string(),
    };
    let utilisateur = service.creer_utilisateur(utilisateur).await?;
    Ok(HttpResponse::Created().json(utilisateur))
}

#[post("/login")]
pub async fn login(
    service: Data<Arc<Box<dyn UtilisateurEntree>>>,
    req: Json<LoginRequest>,
) -> Result<impl Responder, MyError> {
    let utilisateur = service.authentifier(&req.email, &req.mot_de_passe).await?;
    let token = generate_jwt(&utilisateur).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "token": token })))
}

#[get("/utilisateurs")]
pub async fn get_all_utilisateurs(
    service: Data<Arc<Box<dyn UtilisateurEntree>>>,
    role: web::ReqData<String>,
) -> Result<impl Responder, MyError> {
    let utilisateurs = service.lister_utilisateurs(&role).await?;
    Ok(HttpResponse::Ok().json(utilisateurs))
}

#[get("/utilisateurs/{id}")]
pub async fn get_utilisateur(
    id: Path<Uuid>,
    service: Data<Arc<Box<dyn UtilisateurEntree>>>,
    current_user: web::ReqData<Utilisateur>,
    role: web::ReqData<String>,
) -> Result<impl Responder, MyError> {
    let utilisateur = service
        .obtenir_utilisateur(*id, &role, current_user.id)
        .await?
        .ok_or_else(|| MyError::NotFound("Utilisateur non trouvé".to_string()))?;
    Ok(HttpResponse::Ok().json(utilisateur))
}

#[put("/utilisateurs/{id}")]
pub async fn update_utilisateur(
    id: Path<Uuid>,
    service: Data<Arc<Box<dyn UtilisateurEntree>>>,
    current_user: web::ReqData<Utilisateur>,
    role: web::ReqData<String>,
    req: Json<UtilisateurRequest>,
) -> Result<impl Responder, MyError> {
    req.validate().map_err(|e| MyError::Validation(e.to_string()))?;
    let utilisateur = Utilisateur {
        id: *id,
        email: req.email.clone(),
        mot_de_passe: req.mot_de_passe.clone(),
        prenom: req.prenom.clone(),
        nom: req.nom.clone(),
        date_creation: current_user.date_creation,
        role: current_user.role.clone(),
    };
    let utilisateur = service.mettre_a_jour_utilisateur(utilisateur, &role, current_user.id).await?;
    Ok(HttpResponse::Ok().json(utilisateur))
}

#[put("/utilisateurs/{id}/role")]
pub async fn update_user_role(
    id: Path<Uuid>,
    service: Data<Arc<Box<dyn UtilisateurEntree>>>,
    role: web::ReqData<String>,
    req: Json<UpdateRoleRequest>,
) -> Result<impl Responder, MyError> {
    req.validate().map_err(|e| MyError::Validation(e.to_string()))?;
    let utilisateur = service.changer_role(*id, &req.role, &role).await?;
    Ok(HttpResponse::Ok().json(utilisateur))
}

#[delete("/utilisateurs/{id}")]
pub async fn delete_utilisateur(
    id: Path<Uuid>,
    service: Data<Arc<Box<dyn UtilisateurEntree>>>,
    role: web::ReqData<String>,
) -> Result<impl Responder, MyError> {
    service.supprimer_utilisateur(*id, &role).await?;
    Ok(HttpResponse::NoContent().finish())
}
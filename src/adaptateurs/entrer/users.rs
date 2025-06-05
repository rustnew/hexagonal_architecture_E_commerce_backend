use actix_web::{web, HttpResponse, Responder};
use actix_web::ResponseError;

use uuid::Uuid;
use chrono::Utc;
use crate::ports::users::UtilisateurEntree;
use crate::domain::user::{Utilisateur, CreateUser, UpdateUser};
use crate::domain::error::MyError;




pub async fn obtenir_par_id(
    path: web::Path<Uuid>,
    repo: web::Data<dyn UtilisateurEntree>,
) -> impl Responder {
    match repo.obtenir_par_id(path.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(MyError::NotFound("Utilisateur non trouvé".to_string())),
        Err(e) => HttpResponse::build(e.status_code()).json(e),
    }
}

pub async fn creer(
    repo: web::Data<dyn UtilisateurEntree>,
    user: web::Json<CreateUser>,
) -> impl Responder {
    let create_user = user.into_inner();
    let new_user = Utilisateur::new(create_user);
    match repo.creer(&new_user).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => HttpResponse::build(e.status_code()).json(e),
    }
}

pub async fn obtenir_par_nom(
    path: web::Path<String>,
    repo: web::Data<dyn UtilisateurEntree>,
) -> impl Responder {
    match repo.obtenir_par_nom(&path.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(MyError::NotFound("Utilisateur non trouvé".to_string())),
        Err(e) => HttpResponse::build(e.status_code()).json(e),
    }
}

pub async fn obtenir_par_email(
    path: web::Path<String>,
    repo: web::Data<dyn UtilisateurEntree>,
) -> impl Responder {
    match repo.obtenir_par_email(&path.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(MyError::NotFound("Utilisateur non trouvé".to_string())),
        Err(e) => HttpResponse::build(e.status_code()).json(e),
    }
}

pub async fn obtenir_tous(
    repo: web::Data<dyn UtilisateurEntree>,
) -> impl Responder {
    match repo.obtenir_tous().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::build(e.status_code()).json(e),
    }
}

pub async fn mettre_a_jour(
    path: web::Path<Uuid>,
    repo: web::Data<dyn UtilisateurEntree>,
    update_user: web::Json<UpdateUser>,
) -> impl Responder {
    let id = path.into_inner();
    match repo.obtenir_par_id(id).await {
        Ok(Some(mut existing_user)) => {
            // Mettre à jour les champs non nuls
            if let Some(email) = &update_user.email {
                existing_user.email = email.clone();
            }
            if let Some(mot_de_passe) = &update_user.mot_de_passe {
                existing_user.mot_de_passe = mot_de_passe.clone();
            }
            if let Some(prenom) = &update_user.prenom {
                existing_user.prenom = prenom.clone();
            }
            if let Some(nom) = &update_user.nom {
                existing_user.nom = nom.clone();
            }
            existing_user.date_update = Utc::now();
            // Le rôle n'est pas modifié, il reste celui défini par défaut

            match repo.mettre_a_jour(&existing_user).await {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(e) => HttpResponse::build(e.status_code()).json(e),
            }
        }
        Ok(None) => HttpResponse::NotFound().json(MyError::NotFound("Utilisateur non trouvé".to_string())),
        Err(e) => HttpResponse::build(e.status_code()).json(e),
    }
}

pub async fn supprimer(
    path: web::Path<Uuid>,
    repo: web::Data<dyn UtilisateurEntree>,
) -> impl Responder {
    match repo.supprimer(path.into_inner()).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::build(e.status_code()).json(e),
    }
}



pub fn configurer_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/{id}", web::get().to(obtenir_par_id))
            .route("", web::post().to(creer))
            .route("/nom/{nom}", web::get().to(obtenir_par_nom))
            .route("/email/{email}", web::get().to(obtenir_par_email))
            .route("", web::get().to(obtenir_tous))
            .route("/{id}", web::put().to(mettre_a_jour))
            .route("/{id}", web::delete().to(supprimer))
    );
}
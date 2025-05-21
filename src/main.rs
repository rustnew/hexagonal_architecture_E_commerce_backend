use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use tracing_subscriber;
mod domain;
mod ports;
mod adaptateurs;

use crate::adaptateurs::entrer::utilisateurs::create_utilisateur;
use crate::adaptateurs::sortie::utilisateurs::SqlxUtilisateurRepository;
use crate::domain::services::utilisateurs::UtilisateurService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::PgPool::connect(&database_url).await.expect("Failed to connect to database");

    let utilisateur_repository = SqlxUtilisateurRepository::new(pool);
    
    tracing::info!("Starting server on 0.0.0.0:8080");
    HttpServer::new(move || {
        App::new()
            .service(create_utilisateur)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

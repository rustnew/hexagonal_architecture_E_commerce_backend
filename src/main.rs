use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use tracing_subscriber;

mod domain;
mod ports;
mod adaptateurs;

use adaptateurs::entrer::users::configurer_routes;
use adaptateurs::sortie::users::PostgreSql;
use ports::users::UtilisateurEntree;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Charger les variables d'environnement
    dotenv().ok();

    // Initialiser le tracing avec un niveau de log configurable
    tracing_subscriber::fmt()
        .with_env_filter("trace")
        .init();

    // Récupérer l'URL de la base de données
    let database_url = env::var("DATABASE_URL")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("DATABASE_URL must be set: {}", e)))?;

    // Connexion à la base de données
    let pool = sqlx::postgres::PgPool::connect(&database_url)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to connect to database: {}", e)))?;

    // Initialisation du repository
    let repo = PostgreSql::new(pool);
    // Encapsuler le repository dans un Arc<web::Data<dyn UtilisateurEntree>>
    let repo_data = Arc::new(web::Data::new(Box::new(repo) as Box<dyn UtilisateurEntree>));

    println!("Le serveur est disponible sur http://127.0.0.1:8080");
    tracing::info!("Starting server on 0.0.0.0:8080");

    // Lancement du serveur HTTP
    HttpServer::new(move || {
        App::new()
            .app_data(repo_data.clone()) // Partage du repository avec les handlers
            .configure(configurer_routes) // Configuration des routes
    })
    .bind(("127.0.0.1", 8080))? // Lancer le serveur sur le port 8080
    .run()
    .await
}
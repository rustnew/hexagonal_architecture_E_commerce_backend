use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;
mod adaptateurs;
mod domain;
mod ports;

use adaptateurs::{
        entrer::utilisateurs::init_routes as init_utilisateurs_routes,
        sortie::utilisateurs::SqlxUtilisateurRepository,
};
use domain::services::utilisateurs::UtilisateurService;
use ports::entrer::utilisateurs::UtilisateurEntree;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    sqlx::migrate!().run(&pool).await?;

    let utilisateur_repository = SqlxUtilisateurRepository::new(pool);
    let utilisateur_service = Arc::new(Box::new(UtilisateurService::new(utilisateur_repository)) as Box<dyn UtilisateurEntree>);

    let app = init_utilisateurs_routes(utilisateur_service);

    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
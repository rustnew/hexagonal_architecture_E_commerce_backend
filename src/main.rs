use sqlx::postgres::PgPoolOptions;
use axum::{routing::get, Router};
use std::{env, net::SocketAddr};
use dotenv::dotenv;

mod domain;
mod adaptateurs;
mod ports;
mod config;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL doit être définie dans le fichier .env");
    println!("URL de la base de données : {}", database_url);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Serveur démarré sur http://{}", addr);

}

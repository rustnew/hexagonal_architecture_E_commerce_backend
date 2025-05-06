// src/interfaces/api/routes/utilisateurs.rs
use axum::{
    extract::{Path, State},http::StatusCode,
    response::IntoResponse, Json,
    routing::{get, post, put, delete}, Router,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::model::models::Utilisateur;
use crate::ports::entrer::produit::UtilisateurEntree;
use crate::domain::model::implementation::*;

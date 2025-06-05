use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;


// Table: utilisateurs
#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq, Eq)]
pub struct Utilisateur {
    pub id: Uuid, 
    pub email: String, 
    pub mot_de_passe: String, 
    pub prenom: String, 
    pub nom: String, 
    pub role : String,
    pub date_creation: DateTime<Utc>,
    pub date_update : DateTime<Utc>
}

// src/domaine/modeles/utilisateur.rs (ajouté au même fichier)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUser {
    pub email: String,
    pub mot_de_passe: String,
    pub prenom: String,
    pub nom: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub mot_de_passe: Option<String>,
    pub prenom: Option<String>,
    pub nom: Option<String>,
}

impl  Utilisateur {
    pub fn new(create_user: CreateUser) -> Self {
        let now = Utc::now();
        Utilisateur{
            id: Uuid::new_v4(), // Génère un nouvel UUID
            email: create_user.email,
            mot_de_passe: create_user.mot_de_passe, // À hacher dans une application réelle
            prenom: create_user.prenom,
            nom: create_user.nom,
            role: "User".to_string(),
            date_creation: now,
            date_update: now,
        }
    }
}
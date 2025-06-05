use async_trait::async_trait;
use sqlx::{PgPool, Error as SqlxError};
use uuid::Uuid;

use crate::ports::users::UtilisateurEntree;
use crate::domain::user::{Utilisateur, CreateUser, UpdateUser};
use crate::domain::error::MyError;


pub struct PostgreSql {
    pool: PgPool,
}

impl PostgreSql {

    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}



#[async_trait]
impl UtilisateurEntree for PostgreSql {
    async fn creer(&self, utilisateur: &Utilisateur) -> Result<Utilisateur, MyError> {
        let user = sqlx::query_as::<_, Utilisateur>(
            r#"
            INSERT INTO utilisateur (id, email, mot_de_passe, prenom, nom, role, date_creation, date_update)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, email, mot_de_passe, prenom, nom, role, date_creation, date_update
            "#,
        )
        .bind(utilisateur.id)
        .bind(&utilisateur.email)
        .bind(&utilisateur.mot_de_passe)
        .bind(&utilisateur.prenom)
        .bind(&utilisateur.nom)
        .bind(&utilisateur.role)
        .bind(utilisateur.date_creation)
        .bind(utilisateur.date_update)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            SqlxError::Database(db_err) if db_err.constraint().is_some() => {
                MyError::BadRequest("Email déjà utilisé".to_string())
            }
            _ => MyError::Database(e.to_string()),
        })?;

        Ok(user)
    }

    async fn obtenir_par_id(&self, id: Uuid) -> Result<Option<Utilisateur>, MyError> {
        let user = sqlx::query_as::<_, Utilisateur>(
            "SELECT id, email, mot_de_passe, prenom, nom, role, date_creation, date_update 
             FROM utilisateur WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| MyError::Database(e.to_string()))?;

        Ok(user)
    }

    async fn obtenir_par_nom(&self, nom: &str) -> Result<Option<Utilisateur>, MyError> {
        let user = sqlx::query_as::<_, Utilisateur>(
            "SELECT id, email, mot_de_passe, prenom, nom, role, date_creation, date_update 
             FROM utilisateur WHERE nom = $1"
        )
        .bind(nom)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| MyError::Database(e.to_string()))?;

        Ok(user)
    }

    async fn obtenir_par_email(&self, email: &str) -> Result<Option<Utilisateur>, MyError> {
        let user = sqlx::query_as::<_, Utilisateur>(
            "SELECT id, email, mot_de_passe, prenom, nom, role, date_creation, date_update 
             FROM utilisateur WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| MyError::Database(e.to_string()))?;

        Ok(user)
    }

    async fn obtenir_tous(&self) -> Result<Vec<Utilisateur>, MyError> {
        let users = sqlx::query_as::<_, Utilisateur>(
            "SELECT id, email, mot_de_passe, prenom, nom, role, date_creation, date_update 
             FROM utilisateur"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| MyError::Database(e.to_string()))?;

        Ok(users)
    }

    async fn mettre_a_jour(&self, utilisateur: &Utilisateur) -> Result<Utilisateur, MyError> {
        let user = sqlx::query_as::<_, Utilisateur>(
            r#"
            UPDATE utilisateur
            SET email = $2, mot_de_passe = $3, prenom = $4, nom = $5, date_update = $6
            WHERE id = $1
            RETURNING id, email, mot_de_passe, prenom, nom, role, date_creation, date_update
            "#,
        )
        .bind(utilisateur.id)
        .bind(&utilisateur.email)
        .bind(&utilisateur.mot_de_passe)
        .bind(&utilisateur.prenom)
        .bind(&utilisateur.nom)
        .bind(utilisateur.date_update)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            SqlxError::RowNotFound => MyError::NotFound("Utilisateur non trouvé".to_string()),
            _ => MyError::Database(e.to_string()),
        })?;

        Ok(user)
    }

    async fn supprimer(&self, id: Uuid) -> Result<(), MyError> {
        let result = sqlx::query("DELETE FROM utilisateur WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| MyError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(MyError::NotFound("Utilisateur non trouvé".to_string()));
        }

        Ok(())
    }
}
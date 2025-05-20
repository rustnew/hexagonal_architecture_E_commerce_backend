use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::model::models::Utilisateur;
use crate::domain::error::{MyError, not_found_error};
use crate::ports::sortie::utilisateurs::UtilisateurSortie;

pub struct SqlxUtilisateurRepository {
    pub pool: PgPool,
}

impl SqlxUtilisateurRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UtilisateurSortie for SqlxUtilisateurRepository {
    async fn sauvegarder(&self, utilisateur: &Utilisateur) -> Result<Utilisateur, MyError> {
        let utilisateur = sqlx::query_as::<_, Utilisateur>(
            "INSERT INTO utilisateurs (id, email, mot_de_passe, prenom, nom, date_creation, role)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING *",
        )
        .bind(utilisateur.id)
        .bind(&utilisateur.email)
        .bind(&utilisateur.mot_de_passe)
        .bind(&utilisateur.prenom)
        .bind(&utilisateur.nom)
        .bind(utilisateur.date_creation)
        .bind(&utilisateur.role)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| MyError::Database(e))?;
        Ok(utilisateur)
    }

    async fn obtenir_par_id(&self, id: Uuid) -> Result<Option<Utilisateur>, MyError> {
        let utilisateur = sqlx::query_as::<_, Utilisateur>("SELECT * FROM utilisateurs WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| MyError::Database(e))?;
        Ok(utilisateur)
    }

    async fn obtenir_par_email(&self, email: &str) -> Result<Option<Utilisateur>, MyError> {
        let utilisateur = sqlx::query_as::<_, Utilisateur>("SELECT * FROM utilisateurs WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| MyError::Database(e))?;
        Ok(utilisateur)
    }

    async fn obtenir_tous(&self) -> Result<Vec<Utilisateur>, MyError> {
        let utilisateurs = sqlx::query_as::<_, Utilisateur>("SELECT * FROM utilisateurs")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| MyError::Database(e))?;
        Ok(utilisateurs)
    }

    async fn mettre_a_jour(&self, utilisateur: &Utilisateur) -> Result<Utilisateur, MyError> {
        let utilisateur = sqlx::query_as::<_, Utilisateur>(
            "UPDATE utilisateurs
             SET email = $2, mot_de_passe = $3, prenom = $4, nom = $5, role = $6
             WHERE id = $1
             RETURNING *",
        )
        .bind(utilisateur.id)
        .bind(&utilisateur.email)
        .bind(&utilisateur.mot_de_passe)
        .bind(&utilisateur.prenom)
        .bind(&utilisateur.nom)
        .bind(&utilisateur.role)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| MyError::Database(e))?;
        Ok(utilisateur)
    }

    async fn supprimer(&self, id: Uuid) -> Result<(), MyError> {
        let result = sqlx::query("DELETE FROM utilisateurs WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| MyError::Database(e))?;
        if result.rows_affected() == 0 {
            return Err(not_found_error("Utilisateur non trouvé"));
        }
        Ok(())
    }
}
// src/infrastructure/repositories/utilisateur_repository.rs
use crate::domain::model::models::Utilisateur;
use crate::ports::sortie::produit::UtilisateurSortie;
use async_trait::async_trait;
use sqlx::postgres::PgPool;
use uuid::Uuid;

pub struct UtilisateurRepository {
    pool: PgPool,
}

impl UtilisateurRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UtilisateurSortie for UtilisateurRepository {
    async fn sauvegarder(&self, utilisateur: &Utilisateur) -> Result<Utilisateur, String> {
        sqlx::query_as!(
            Utilisateur,
            r#"
            INSERT INTO utilisateurs 
            (id, email, mot_de_passe_hash, prenom, nom, telephone, date_creation, date_mise_a_jour, est_actif, email_verifie)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
            utilisateur.id,
            utilisateur.email,
            utilisateur.mot_de_passe_hash,
            utilisateur.prenom,
            utilisateur.nom,
            utilisateur.telephone,
            utilisateur.date_creation,
            utilisateur.date_mise_a_jour,
            utilisateur.est_actif,
            utilisateur.email_verifie
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn obtenir_par_id(&self, id: Uuid) -> Result<Option<Utilisateur>, String> {
        sqlx::query_as!(
            Utilisateur,
            "SELECT * FROM utilisateurs WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn mettre_a_jour(&self, utilisateur: &Utilisateur) -> Result<Utilisateur, String> {
        sqlx::query_as!(
            Utilisateur,
            r#"
            UPDATE utilisateurs 
            SET email = $2, mot_de_passe_hash = $3, prenom = $4, nom = $5, telephone = $6,
                date_mise_a_jour = $7, est_actif = $8, email_verifie = $9
            WHERE id = $1
            RETURNING *
            "#,
            utilisateur.id,
            utilisateur.email,
            utilisateur.mot_de_passe_hash,
            utilisateur.prenom,
            utilisateur.nom,
            utilisateur.telephone,
            utilisateur.date_mise_a_jour,
            utilisateur.est_actif,
            utilisateur.email_verifie
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn supprimer(&self, id: Uuid) -> Result<(), String> {
        sqlx::query!(
            "DELETE FROM utilisateurs WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
    }

    async fn obtenir_tous(&self) -> Result<Vec<Utilisateur>, String> {
        sqlx::query_as!(
            Utilisateur,
            "SELECT * FROM utilisateurs ORDER BY date_creation DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
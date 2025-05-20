use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::model::models::Utilisateur;
use crate::domain::error::MyError;

#[async_trait]
pub trait UtilisateurEntree: Send + Sync {
    async fn creer_utilisateur(&self, utilisateur: Utilisateur) -> Result<Utilisateur, MyError>;
    async fn obtenir_utilisateur(&self, id: Uuid, current_role: &str, current_user_id: Uuid) -> Result<Option<Utilisateur>, MyError>;
    async fn mettre_a_jour_utilisateur(&self, utilisateur: Utilisateur, current_role: &str, current_user_id: Uuid) -> Result<Utilisateur, MyError>;
    async fn supprimer_utilisateur(&self, id: Uuid, current_role: &str) -> Result<(), MyError>;
    async fn lister_utilisateurs(&self, current_role: &str) -> Result<Vec<Utilisateur>, MyError>;
    async fn authentifier(&self, email: &str, mot_de_passe: &str) -> Result<Utilisateur, MyError>;
    async fn changer_role(&self, user_id: Uuid, new_role: &str, current_role: &str) -> Result<Utilisateur, MyError>;
}
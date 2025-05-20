use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::model::models::Utilisateur;
use crate::domain::error::MyError;

#[async_trait]
pub trait UtilisateurSortie: Send + Sync {
    async fn sauvegarder(&self, utilisateur: &Utilisateur) -> Result<Utilisateur, MyError>;
    async fn obtenir_par_id(&self, id: Uuid) -> Result<Option<Utilisateur>, MyError>;
    async fn obtenir_par_email(&self, email: &str) -> Result<Option<Utilisateur>, MyError>;
    async fn obtenir_tous(&self) -> Result<Vec<Utilisateur>, MyError>;
    async fn mettre_a_jour(&self, utilisateur: &Utilisateur) -> Result<Utilisateur, MyError>;
    async fn supprimer(&self, id: Uuid) -> Result<(), MyError>;
}
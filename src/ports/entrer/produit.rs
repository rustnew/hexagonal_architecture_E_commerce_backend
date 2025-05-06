use  crate::domain::model::models::Utilisateur;
use async_trait::async_trait;
use  uuid::Uuid;

// Port d'entrée (trait pour les cas d'utilisation)
#[async_trait]
pub trait UtilisateurEntree {
     async fn creer_utilisateur(&self, utilisateur: Utilisateur) -> Result<Utilisateur, String>;
     async fn obtenir_utilisateur(&self, id: Uuid) -> Result<Option<Utilisateur>, String>;
     async fn mettre_a_jour_utilisateur(&self, utilisateur: Utilisateur) -> Result<Utilisateur, String>;
     async fn supprimer_utilisateur(&self, id: Uuid) -> Result<(), String>;
    async fn lister_utilisateurs(&self) -> Result<Vec<Utilisateur>, String>;
}
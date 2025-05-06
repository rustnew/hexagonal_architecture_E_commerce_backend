use crate::domain::model::models::Utilisateur;
use async_trait::async_trait;
use uuid::Uuid;


#[async_trait]
pub trait UtilisateurSortie {
     async fn sauvegarder(&self, utilisateur: &Utilisateur) -> Result<Utilisateur, String>;
     async fn obtenir_par_id(&self, id: Uuid) -> Result<Option<Utilisateur>, String>;
     async fn mettre_a_jour(&self, utilisateur: &Utilisateur) -> Result<Utilisateur, String>;
     async fn supprimer(&self, id: Uuid) -> Result<(), String>;
     async fn obtenir_tous(&self) -> Result<Vec<Utilisateur>, String>;
}
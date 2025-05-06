use  async_trait::async_trait;
use crate::domain::model::models::Utilisateur;
use crate::ports::entrer::produit::UtilisateurEntree;
use crate::ports::sortie::produit::UtilisateurSortie;
use uuid::Uuid;

pub struct UtilisateurService<S: UtilisateurSortie + Send + Sync> {
    repository: S,
}

impl<S: UtilisateurSortie + Send + Sync> UtilisateurService<S> {
    pub fn new(repository: S) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<S: UtilisateurSortie + Send + Sync> UtilisateurEntree for UtilisateurService<S> {
    async fn creer_utilisateur(&self, utilisateur: Utilisateur) -> Result<Utilisateur, String> {
        // Validation supplémentaire peut être ajoutée ici avant la création
        self.repository.sauvegarder(&utilisateur).await
    }

    async fn obtenir_utilisateur(&self, id: Uuid) -> Result<Option<Utilisateur>, String> {
        self.repository.obtenir_par_id(id).await
    }

    async fn mettre_a_jour_utilisateur(&self, utilisateur: Utilisateur) -> Result<Utilisateur, String> {
        // Vérifier d'abord si l'utilisateur existe
        match self.repository.obtenir_par_id(utilisateur.id).await? {
            Some(_) => self.repository.mettre_a_jour(&utilisateur).await,
            None => Err("Utilisateur non trouvé".to_string()),
        }
    }

    async fn supprimer_utilisateur(&self, id: Uuid) -> Result<(), String> {
        self.repository.supprimer(id).await
    }

    async fn lister_utilisateurs(&self) -> Result<Vec<Utilisateur>, String> {
        self.repository.obtenir_tous().await
    }
}
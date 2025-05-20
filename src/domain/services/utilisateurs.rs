use crate::domain::model::models::Utilisateur;
use crate::domain::error::{MyError, validation_error};
use crate::ports::entrer::utilisateurs::UtilisateurEntree;
use crate::ports::sortie::utilisateurs::UtilisateurSortie;
use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

pub struct UtilisateurService<R: UtilisateurSortie> {
    repository: R,
}

impl<R: UtilisateurSortie> UtilisateurService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: UtilisateurSortie + Send + Sync> UtilisateurEntree for UtilisateurService<R> {
    async fn creer_utilisateur(&self, mut utilisateur: Utilisateur) -> Result<Utilisateur, MyError> {
        if utilisateur.email.is_empty() {
            return Err(validation_error("L'email ne peut pas être vide"));
        }
        if self.repository.obtenir_par_email(&utilisateur.email).await?.is_some() {
            return Err(validation_error("L'email est déjà utilisé"));
        }
        if utilisateur.mot_de_passe.len() < 8 {
            return Err(validation_error("Le mot de passe doit avoir au moins 8 caractères"));
        }
        if utilisateur.prenom.is_empty() {
            return Err(validation_error("Le prénom ne peut pas être vide"));
        }
        if utilisateur.nom.is_empty() {
            return Err(validation_error("Le nom ne peut pas être vide"));
        }
        if utilisateur.role.is_empty() {
            utilisateur.role = "utilisateur".to_string();
        } else if !["utilisateur", "gerant"].contains(&utilisateur.role.as_str()) {
            return Err(validation_error("Rôle invalide"));
        }
        utilisateur.mot_de_passe = hash(&utilisateur.mot_de_passe, DEFAULT_COST)
            .map_err(|e| MyError::Custom(format!("Erreur de hachage: {}", e)))?;
        let utilisateur = self.repository.sauvegarder(&utilisateur).await?;
        Ok(utilisateur)
    }

    async fn obtenir_utilisateur(&self, id: Uuid, current_role: &str, current_user_id: Uuid) -> Result<Option<Utilisateur>, MyError> {
        if current_role != "gerant" && current_user_id != id {
            return Err(MyError::Unauthorized("Vous ne pouvez accéder qu'à votre propre profil".to_string()));
        }
        let utilisateur = self.repository.obtenir_par_id(id).await?;
        Ok(utilisateur)
    }

    async fn mettre_a_jour_utilisateur(&self, mut utilisateur: Utilisateur, current_role: &str, current_user_id: Uuid) -> Result<Utilisateur, MyError> {
        if current_role != "gerant" && current_user_id != utilisateur.id {
            return Err(MyError::Unauthorized("Vous ne pouvez modifier que votre propre profil".to_string()));
        }
        if utilisateur.email.is_empty() {
            return Err(validation_error("L'email ne peut pas être vide"));
        }
        if let Some(existing) = self.repository.obtenir_par_email(&utilisateur.email).await? {
            if existing.id != utilisateur.id {
                return Err(validation_error("L'email est déjà utilisé"));
            }
        }
        if utilisateur.mot_de_passe.len() < 8 {
            return Err(validation_error("Le mot de passe doit avoir au moins 8 caractères"));
        }
        if utilisateur.prenom.is_empty() {
            return Err(validation_error("Le prénom ne peut pas être vide"));
        }
        if utilisateur.nom.is_empty() {
            return Err(validation_error("Le nom ne peut pas être vide"));
        }
        utilisateur.mot_de_passe = hash(&utilisateur.mot_de_passe, DEFAULT_COST)
            .map_err(|e| MyError::Custom(format!("Erreur de hachage: {}", e)))?;
        if let Some(existing) = self.repository.obtenir_par_id(utilisateur.id).await? {
            utilisateur.role = existing.role;
        } else {
            return Err(MyError::NotFound("Utilisateur non trouvé".to_string()));
        }
        let utilisateur = self.repository.mettre_a_jour(&utilisateur).await?;
        Ok(utilisateur)
    }

    async fn supprimer_utilisateur(&self, id: Uuid, current_role: &str) -> Result<(), MyError> {
        if current_role != "gerant" {
            return Err(MyError::Unauthorized("Seul un gérant peut supprimer un utilisateur".to_string()));
        }
        self.repository.supprimer(id).await?;
        Ok(())
    }

    async fn lister_utilisateurs(&self, current_role: &str) -> Result<Vec<Utilisateur>, MyError> {
        if current_role != "gerant" {
            return Err(MyError::Unauthorized("Seul un gérant peut lister les utilisateurs".to_string()));
        }
        let utilisateurs = self.repository.obtenir_tous().await?;
        Ok(utilisateurs)
    }

    async fn authentifier(&self, email: &str, mot_de_passe: &str) -> Result<Utilisateur, MyError> {
        let utilisateur = self
            .repository
            .obtenir_par_email(email)
            .await?
            .ok_or_else(|| MyError::Unauthorized("Email ou mot de passe incorrect".to_string()))?;

        if !verify(mot_de_passe, &utilisateur.mot_de_passe)
            .map_err(|_| MyError::Custom("Erreur de vérification du mot de passe".to_string()))?
        {
            return Err(MyError::Unauthorized("Email ou mot de passe incorrect".to_string()));
        }

        Ok(utilisateur)
    }

    async fn changer_role(&self, user_id: Uuid, new_role: &str, current_role: &str) -> Result<Utilisateur, MyError> {
        if current_role != "gerant" {
            return Err(MyError::Unauthorized("Seul un gérant peut modifier un rôle".to_string()));
        }
        if !["utilisateur", "gerant"].contains(&new_role) {
            return Err(validation_error("Rôle invalide"));
        }
        let mut utilisateur = self
            .repository
            .obtenir_par_id(user_id)
            .await?
            .ok_or_else(|| MyError::NotFound("Utilisateur non trouvé".to_string()))?;
        utilisateur.role = new_role.to_string();
        let utilisateur = self.repository.mettre_a_jour(&utilisateur).await?;
        Ok(utilisateur)
    }
}
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Utilisateur {
    pub id: Uuid,
    pub email: String,
    pub mot_de_passe_hash: String,
    pub prenom: String,
    pub nom: String,
    pub telephone: Option<String>,
    pub date_creation: DateTime<Utc>,
    pub date_mise_a_jour: DateTime<Utc>,
    pub est_actif: bool,
    pub email_verifie: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Produit {
    pub id: Uuid,
    pub nom: String,
    pub description: String,
    pub description_courte: Option<String>,
    pub reference: String,
    pub prix: f64,
    pub prix_comparatif: Option<f64>,
    pub prix_revient: Option<f64>,
    pub quantite: i32,
    pub categorie_id: Uuid,
    pub est_publie: bool,
    pub est_en_vedette: bool,
    pub date_creation: DateTime<Utc>,
    pub date_mise_a_jour: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Categorie {
    pub id: Uuid,
    pub nom: String,
    pub description: Option<String>,
    pub categorie_parente_id: Option<Uuid>,
    pub url_image: Option<String>,
    pub est_active: bool,
    pub date_creation: DateTime<Utc>,
    pub date_mise_a_jour: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageProduit {
    pub id: Uuid,
    pub produit_id: Uuid,
    pub url: String,
    pub texte_alternatif: Option<String>,
    pub est_principale: bool,
    pub ordre_tri: i32,
    pub date_creation: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarianteProduit {
    pub id: Uuid,
    pub produit_id: Uuid,
    pub nom: String,
    pub valeur: String,
    pub reference: Option<String>,
    pub ajustement_prix: f64,
    pub quantite: Option<i32>,
    pub date_creation: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeNotification {
    Commande,
    Livraison,
    Promotion,
    Compte,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub utilisateur_id: Uuid,
    pub type_notification: TypeNotification,
    pub contenu: String,
    pub est_lue: bool,
    pub date_creation: DateTime<Utc>,
    pub date_lue: Option<DateTime<Utc>>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adresse {
    pub id: Uuid,
    pub utilisateur_id: Uuid,
    pub prenom: String,
    pub nom: String,
    pub ligne1: String,
    pub ville: String,
    pub region: String,
    pub lieu_proche_connus: String,
    pub quartier: String,
    pub est_principale: bool,
    pub date_creation: DateTime<Utc>,
    pub date_mise_a_jour: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panier {
    pub id: Uuid,
    pub utilisateur_id: Option<Uuid>,
    pub date_creation: DateTime<Utc>,
    pub date_mise_a_jour: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticlePanier {
    pub id: Uuid,
    pub panier_id: Uuid,
    pub produit_id: Uuid,
    pub variante_id: Option<Uuid>,
    pub quantite: i32,
    pub date_creation: DateTime<Utc>,
    pub date_mise_a_jour: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StatutCommande {
    EnAttente,
    EnTraitement,
    EnLivraison,
    Livree,
    Annulee,
    Retournee,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commande {
    pub id: Uuid,
    pub utilisateur_id: Option<Uuid>,
    pub numero_commande: String,
    pub statut: StatutCommande,
    pub sous_total: f64,
    pub montant_taxes: f64,
    pub frais_livraison: f64,
    pub montant_remise: f64,
    pub montant_total: f64,
    pub adresse_livraison_id: Uuid,
    pub note: Option<String>,
    pub date_creation: DateTime<Utc>,
    pub date_mise_a_jour: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleCommande {
    pub id: Uuid,
    pub commande_id: Uuid,
    pub produit_id: Uuid,
    pub variante_id: Option<Uuid>,
    pub nom_produit: String,
    pub description_variante: Option<String>,
    pub prix: f64,
    pub quantite: i32,
    pub montant_remise: f64,
    pub prix_total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StatutPaiement {
    EnAttente,
    Complete,
    Echoue,
    Remboursee,
    PartiellementRemboursee,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paiement {
    pub id: Uuid,
    pub commande_id: Uuid,
    pub montant: f64,
    pub methode_paiement: String,
    pub id_transaction: Option<String>,
    pub statut: StatutPaiement,
    pub devise: String,
    pub details_paiement: Option<Value>,
    pub date_creation: DateTime<Utc>,
    pub date_mise_a_jour: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StatutLivraison {
    EnPreparation,
    Expediee,
    EnTransit,
    EnLivraison,
    Livree,
    Retournee,
    Annulee,
    Probleme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Livraison {
    pub id: Uuid,
    pub commande_id: Uuid,
    pub methode_livraison: String,
    pub numero_suivi: Option<String>,
    pub transporteur: Option<String>,
    pub statut: StatutLivraison,
    pub date_livraison_estimee: Option<DateTime<Utc>>,
    pub date_livraison_reelle: Option<DateTime<Utc>>,
    pub cout_livraison: f64,
    pub date_creation: DateTime<Utc>,
    pub date_mise_a_jour: DateTime<Utc>,
}

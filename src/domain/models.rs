use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;


// Table: utilisateurs
#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq, Eq)]
pub struct User {
    pub id: Uuid, // PRIMARY KEY, DEFAULT uuid_generate_v4()
    pub email: String, // VARCHAR(255), NOT NULL, UNIQUE
    pub mot_de_passe: String, // VARCHAR(255), NOT NULL
    pub prenom: String, // VARCHAR(100), NOT NULL
    pub nom: String, // VARCHAR(100), NOT NULL
    pub date_creation: DateTime<Utc>,
    pub role :  String,
}

// Table: sessions
#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq, Eq)]
pub struct Session {
    pub id: Uuid, // PRIMARY KEY, DEFAULT uuid_generate_v4()
    pub token: String, // VARCHAR(255), NOT NULL, UNIQUE
    pub utilisateur_id: Option<Uuid>, // UUID, REFERENCES utilisateurs(id)
    pub date_expiration: DateTime<Utc>, // TIMESTAMPTZ, NOT NULL
    pub date_creation: DateTime<Utc>, // TIMESTAMPTZ, NOT NULL, DEFAULT CURRENT_TIMESTAMP
}

// Table: categories
#[derive(Debug, Serialize, Deserialize, Clone,  FromRow)]
pub struct Categorie {
    pub id: Uuid, // PRIMARY KEY, DEFAULT uuid_generate_v4()
    pub nom: String, // VARCHAR(50), NOT NULL
    pub description: Option<String>, // TEXT
    pub date_creation: DateTime<Utc>, // TIMESTAMPTZ, NOT NULL, DEFAULT CURRENT_TIMESTAMP
}

// Table: produits
#[derive(Debug, Clone , Serialize, Deserialize, FromRow)]
pub struct Produit {
    pub id: Uuid, // PRIMARY KEY, DEFAULT uuid_generate_v4()
    pub nom: String, // VARCHAR(100), NOT NULL
    pub description: String, // TEXT, NOT NULL
    pub reference: String, // VARCHAR(50), UNIQUE, NOT NULL
    pub prix: String, // DECIMAL(12, 2), NOT NULL, CHECK (prix > 0)
    pub quantite: i32, // INTEGER, NOT NULL, DEFAULT 0, CHECK (quantite >= 0)
    pub categorie_id: Option<Uuid>, // UUID, REFERENCES categories(id), ON DELETE SET NULL
    pub image_principale_url: Option<String>, // VARCHAR(255)
    pub est_publie: bool, // BOOLEAN, NOT NULL, DEFAULT TRUE
    pub date_creation: DateTime<Utc>, // TIMESTAMPTZ, NOT NULL, DEFAULT CURRENT_TIMESTAMP
}

// Table: variantes_produit
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct VarianteProduit {
    pub id: Uuid, // PRIMARY KEY, DEFAULT uuid_generate_v4()
    pub produit_id: Uuid, // UUID, NOT NULL, REFERENCES produits(id), ON DELETE CASCADE
    pub nom: String, // VARCHAR(50), NOT NULL
    pub valeur: String, // VARCHAR(50), NOT NULL
    pub prix_ajuste: String, // DECIMAL(10, 2), DEFAULT 0.00
    pub quantite: i32, // INTEGER, NOT NULL, DEFAULT 0, CHECK (quantite >= 0)
    pub date_creation: DateTime<Utc>, // TIMESTAMPTZ, NOT NULL, DEFAULT CURRENT_TIMESTAMP
    // UNIQUE(produit_id, nom, valeur)
}

// Table: reviews
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Review {
    pub id: Uuid, // PRIMARY KEY, DEFAULT uuid_generate_v4()
    pub produit_id: Uuid, // UUID, NOT NULL, REFERENCES produits(id), ON DELETE CASCADE
    pub utilisateur_id: Uuid, // UUID, NOT NULL, REFERENCES utilisateurs(id)
    pub note: i32, // INTEGER, NOT NULL, CHECK (note >= 1 AND note <= 5)
    pub commentaire: Option<String>, // TEXT
    pub date_creation: DateTime<Utc>, // TIMESTAMPTZ, NOT NULL, DEFAULT CURRENT_TIMESTAMP
    // UNIQUE(produit_id, utilisateur_id)
}

// Table: wisrhlist
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Wishlist {
    pub id: Uuid, // PRIMARY KEY, DEFAULT uuid_generate_v4()
    pub utilisateur_id: Uuid, // UUID, NOT NULL, REFERENCES utilisateurs(id)
    pub produit_id: Uuid, // UUID, NOT NULL, REFERENCES produits(id), ON DELETE CASCADE
    pub date_ajout: DateTime<Utc>, // TIMESTAMPTZ, NOT NULL, DEFAULT CURRENT_TIMESTAMP
    // UNIQUE(utilisateur_id, produit_id)
}

// Table: promotions
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Promotion {
    pub id: Uuid, // PRIMARY KEY, DEFAULT uuid_generate_v4()
    pub code: String, // VARCHAR(20), NOT NULL, UNIQUE
    pub description: Option<String>, // TEXT
    pub pourcentage_remise: Option<String>, // DECIMAL(5, 2), CHECK (pourcentage_remise >= 0 AND pourcentage_remise <= 100)
    pub montant_remise: Option<String>, // DECIMAL(12, 2), CHECK (montant_remise >= 0)
    pub date_debut: DateTime<Utc>, // TIMESTAMPTZ, NOT NULL
    pub date_fin: DateTime<Utc>, // TIMESTAMPTZ, NOT NULL
    pub date_creation: DateTime<Utc>, // TIMESTAMPTZ, NOT NULL, DEFAULT CURRENT_TIMESTAMP
    // CHECK (pourcentage_remise IS NOT NULL OR montant_remise IS NOT NULL)
}

// Table: methodes_paiement
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MethodePaiement {
    pub id: Uuid, // PRIMARY KEY, DEFAULT uuid_generate_v4()
    pub nom: String, // VARCHAR(50), NOT NULL
    pub description: Option<String>, // TEXT
    pub date_creation: DateTime<Utc>, // TIMESTAMPTZ, NOT NULL, DEFAULT CURRENT_TIMESTAMP
}

// Table: methodes_livraison
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MethodeLivraison {
    pub id: Uuid, // PRIMARY KEY, DEFAULT uuid_generate_v4()
    pub nom: String, // VARCHAR(50), NOT NULL
    pub description: Option<String>, // TEXT
    pub cout: String, // DECIMAL(10, 2), NOT NULL, DEFAULT 0.00
    pub delai_estime: Option<String>, // VARCHAR(50)
    pub date_creation: DateTime<Utc>, // TIMESTAMPTZ, NOT NULL, DEFAULT CURRENT_TIMESTAMP
}

// Table: adresses
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Adresse {
    pub id: Uuid, // PRIMARY KEY, DEFAULT uuid_generate_v4()
    pub utilisateur_id: Uuid, // UUID, NOT NULL, REFERENCES utilisateurs(id)
    pub prenom: String, // VARCHAR(100), NOT NULL
    pub nom: String, // VARCHAR(100), NOT NULL
    pub ligne1: String, // VARCHAR(255), NOT NULL
    pub ville: String, // VARCHAR(100), NOT NULL
    pub region: String, // VARCHAR(100), NOT NULL
    pub code_postal: Option<String>, // VARCHAR(20)
    pub pays: String, // VARCHAR(100), NOT NULL, DEFAULT 'France'
    pub est_principale: bool, // BOOLEAN, DEFAULT FALSE
    pub date_creation: DateTime<Utc>, // TIMESTAMPTZ, NOT NULL, DEFAULT CURRENT_TIMESTAMP
}

// Table: paniers_articles
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PanierArticle {
    pub id: Uuid, // PRIMARY KEY, DEFAULT uuid_generate_v4()
    pub utilisateur_id: Option<Uuid>, // UUID, REFERENCES utilisateurs(id)
    pub session_id: Option<Uuid>, // UUID, REFERENCES sessions(id)
    pub produit_id: Uuid, // UUID, NOT NULL, REFERENCES produits(id)
    pub variante_id: Option<Uuid>, // UUID, REFERENCES variantes_produit(id)
    pub quantite: i32, // INTEGER, NOT NULL, CHECK (quantite > 0)
    pub date_creation: DateTime<Utc>, // TIMESTAMPTZ, NOT NULL, DEFAULT CURRENT_TIMESTAMP
    // UNIQUE(utilisateur_id, session_id, produit_id, variante_id)
    // CHECK (utilisateur_id IS NOT NULL OR session_id IS NOT NULL)
}

// Table: commandes
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Commande {
    pub id: Uuid, // PRIMARY KEY, DEFAULT uuid_generate_v4()
    pub utilisateur_id: Option<Uuid>, // UUID, REFERENCES utilisateurs(id)
    pub numero_commande: String, // VARCHAR(20), UNIQUE, NOT NULL
    pub statut: CommandeStatut, // VARCHAR(20), NOT NULL, DEFAULT 'en_attente'
    pub montant_total: String, // DECIMAL(12, 2), NOT NULL, CHECK (montant_total >= 0)
    pub promotion_id: Option<Uuid>, // UUID, REFERENCES promotions(id)
    pub lieu_publique_proche: Uuid, // UUID, NOT NULL, REFERENCES adresses(id)
    pub methode_paiement_id: Option<Uuid>, // UUID, REFERENCES methodes_paiement(id)
    pub statut_paiement: Option<PaiementStatut>, // VARCHAR(20), DEFAULT 'en_attente'
    pub id_transaction: Option<String>, // VARCHAR(100)
    pub methode_livraison_id: Option<Uuid>, // UUID, REFERENCES methodes_livraison(id)
    pub statut_livraison: Option<LivraisonStatut>, // VARCHAR(20), DEFAULT 'en_preparation'
    pub numero_suivi: Option<String>, // VARCHAR(100)
    pub devise: String, // VARCHAR(3), NOT NULL, DEFAULT 'EUR'
    pub date_creation: DateTime<Utc>, // TIMESTAMPTZ, NOT NULL, DEFAULT CURRENT_TIMESTAMP
}

// Enum for commandes.statut
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "snake_case")]
pub enum CommandeStatut {
    EnAttente,
    EnTraitement,
    Expediee,
    Livree,
    Annulee,
    Remboursee,
}

// Enum for commandes.statut_paiement
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "snake_case")]
pub enum PaiementStatut {
    EnAttente,
    Complete,
    Echoue,
    Remboursee,
}

// Enum for commandes.statut_livraison
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "snake_case")]
pub enum LivraisonStatut {
    EnPreparation,
    Expediee,
    EnTransit,
    Livree,
    Echouee,
}

// Table: articles_commande
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ArticleCommande {
    pub id: Uuid, // PRIMARY KEY, DEFAULT uuid_generate_v4()
    pub commande_id: Uuid, // UUID, NOT NULL, REFERENCES commandes(id), ON DELETE CASCADE
    pub produit_id: Uuid, // UUID, NOT NULL, REFERENCES produits(id)
    pub variante_id: Option<Uuid>, // UUID, REFERENCES variantes_produit(id)
    pub nom_produit: String, // VARCHAR(255), NOT NULL
    pub prix_unitaire: String, // DECIMAL(12, 2), NOT NULL, CHECK (prix_unitaire >= 0)
    pub quantite: i32, // INTEGER, NOT NULL, CHECK (quantite > 0)
    pub prix_total: String, // DECIMAL(12, 2), NOT NULL, CHECK (prix_total >= 0)
}

// Table: notifications
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Notification {
    pub id: Uuid, // PRIMARY KEY, DEFAULT uuid_generate_v4()
    pub utilisateur_id: Uuid, // UUID, NOT NULL, REFERENCES utilisateurs(id)
    pub type_notification: String, // VARCHAR(50), NOT NULL
    pub contenu: String, // TEXT, NOT NULL
    pub est_lue: bool, // BOOLEAN, NOT NULL, DEFAULT FALSE
    pub date_creation: DateTime<Utc>, // TIMESTAMPTZ, NOT NULL, DEFAULT CURRENT_TIMESTAMP
    pub metadata: Option<serde_json::Value>, // JSONB
}
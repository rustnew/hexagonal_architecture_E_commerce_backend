use crate::domain::model::models::*;
use chrono::Utc;
use uuid::Uuid;
use serde_json::Value;

impl Utilisateur {
    pub fn new(
        email: String,
        mot_de_passe_hash: String,
        prenom: String,
        nom: String,
        telephone: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Utilisateur {
            id: Uuid::new_v4(),
            email,
            mot_de_passe_hash,
            prenom,
            nom,
            telephone,
            date_creation: now,
            date_mise_a_jour: now,
            est_actif: true,
            email_verifie: false,
        }
    }
}

impl Produit {
    pub fn nouveau(
        nom: String,
        description: String,
        description_courte: Option<String>,
        reference: String,
        prix: f64,
        prix_comparatif: Option<f64>,
        prix_revient: Option<f64>,
        quantite: i32,
        categorie_id: Uuid,
        est_publie: bool,
        est_en_vedette: bool,
    ) -> Self {
        let maintenant = Utc::now();
        Produit {
            id: Uuid::new_v4(),
            nom,
            description,
            description_courte,
            reference,
            prix,
            prix_comparatif,
            prix_revient,
            quantite,
            categorie_id,
            est_publie,
            est_en_vedette,
            date_creation: maintenant,
            date_mise_a_jour: maintenant,
        }
    }
}

impl Categorie {
    pub fn nouvelle(
        nom: String,
        description: Option<String>,
        categorie_parente_id: Option<Uuid>,
        url_image: Option<String>,
    ) -> Self {
        let maintenant = Utc::now();
        Categorie {
            id: Uuid::new_v4(),
            nom,
            description,
            categorie_parente_id,
            url_image,
            est_active: true,
            date_creation: maintenant,
            date_mise_a_jour: maintenant,
        }
    }
}

impl ImageProduit {
    pub fn nouvelle(
        produit_id: Uuid,
        url: String,
        texte_alternatif: Option<String>,
        est_principale: bool,
        ordre_tri: i32,
    ) -> Self {
        ImageProduit {
            id: Uuid::new_v4(),
            produit_id,
            url,
            texte_alternatif,
            est_principale,
            ordre_tri,
            date_creation: Utc::now(),
        }
    }
}

impl VarianteProduit {
    pub fn nouvelle(
        produit_id: Uuid,
        nom: String,
        valeur: String,
        reference: Option<String>,
        ajustement_prix: f64,
        quantite: Option<i32>,
    ) -> Self {
        VarianteProduit {
            id: Uuid::new_v4(),
            produit_id,
            nom,
            valeur,
            reference,
            ajustement_prix,
            quantite,
            date_creation: Utc::now(),
        }
    }
}

impl Notification {
    pub fn new(
        utilisateur_id: Uuid,
        type_notification: TypeNotification,
        contenu: String,
        metadata: Option<serde_json::Value>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            utilisateur_id,
            type_notification,
            contenu,
            est_lue: false,
            date_creation: Utc::now(),
            date_lue: None,
            metadata,
        }
    }
}

impl Adresse {
    pub fn nouvelle(
        utilisateur_id: Uuid,
        prenom: String,
        nom: String,
        ligne1: String,
        ville: String,
        region: String,
        lieu_proche_connus: String,
        quartier: String,
        est_principale: bool,
    ) -> Self {
        Adresse {
            id: Uuid::new_v4(),
            utilisateur_id,
            prenom,
            nom,
            ligne1,
            ville,
            region,
            lieu_proche_connus,
            quartier,
            est_principale,
            date_creation: Utc::now(),
            date_mise_a_jour: Utc::now(),
        }
    }

    pub fn mettre_a_jour(&mut self, nouvelle_adresse: Adresse) {
        self.prenom = nouvelle_adresse.prenom;
        self.nom = nouvelle_adresse.nom;
        self.ligne1 = nouvelle_adresse.ligne1;
        self.ville = nouvelle_adresse.ville;
        self.region = nouvelle_adresse.region;
        self.lieu_proche_connus = nouvelle_adresse.lieu_proche_connus;
        self.quartier = nouvelle_adresse.quartier;
        self.est_principale = nouvelle_adresse.est_principale;
        self.date_mise_a_jour = Utc::now();
    }
}

impl Panier {
    pub fn nouveau(utilisateur_id: Option<Uuid>) -> Self {
        Panier {
            id: Uuid::new_v4(),
            utilisateur_id,
            date_creation: Utc::now(),
            date_mise_a_jour: Utc::now(),
        }
    }
}

impl ArticlePanier {
    pub fn nouveau(
        panier_id: Uuid,
        produit_id: Uuid,
        variante_id: Option<Uuid>,
        quantite: i32,
    ) -> Self {
        ArticlePanier {
            id: Uuid::new_v4(),
            panier_id,
            produit_id,
            variante_id,
            quantite,
            date_creation: Utc::now(),
            date_mise_a_jour: Utc::now(),
        }
    }
}

impl Commande {
    pub fn nouvelle(
        utilisateur_id: Option<Uuid>,
        sous_total: f64,
        montant_taxes: f64,
        frais_livraison: f64,
        montant_remise: f64,
        adresse_livraison_id: Uuid,
        note: Option<String>,
    ) -> Self {
        let id = Uuid::new_v4();
        let numero_commande = format!("CMD-{}", id.simple());
        let date_actuelle = Utc::now();
        let montant_total = sous_total + montant_taxes + frais_livraison - montant_remise;

        Commande {
            id,
            utilisateur_id,
            numero_commande,
            statut: StatutCommande::EnAttente,
            sous_total,
            montant_taxes,
            frais_livraison,
            montant_remise,
            montant_total,
            adresse_livraison_id,
            note,
            date_creation: date_actuelle,
            date_mise_a_jour: date_actuelle,
        }
    }
}

impl ArticleCommande {
    pub fn nouveau(
        commande_id: Uuid,
        produit_id: Uuid,
        variante_id: Option<Uuid>,
        nom_produit: String,
        description_variante: Option<String>,
        prix: f64,
        quantite: i32,
        montant_remise: f64,
    ) -> Self {
        let prix_total = (prix * quantite as f64) - montant_remise;

        ArticleCommande {
            id: Uuid::new_v4(),
            commande_id,
            produit_id,
            variante_id,
            nom_produit,
            description_variante,
            prix,
            quantite,
            montant_remise,
            prix_total,
        }
    }
}

impl Paiement {
    pub fn nouveau(
        commande_id: Uuid,
        montant: f64,
        methode_paiement: String,
        devise: String,
        id_transaction: Option<String>,
        details_paiement: Option<Value>,
    ) -> Self {
        let date_actuelle = Utc::now();

        Paiement {
            id: Uuid::new_v4(),
            commande_id,
            montant,
            methode_paiement,
            id_transaction,
            statut: StatutPaiement::EnAttente,
            devise,
            details_paiement,
            date_creation: date_actuelle,
            date_mise_a_jour: date_actuelle,
        }
    }
}

impl Livraison {
    pub fn new(
        commande_id: Uuid,
        methode_livraison: String,
        cout_livraison: f64,
    ) -> Self {
        let now = Utc::now();
        let status = String::new();
        Self {
            id: Uuid::new_v4(),
            commande_id,
            methode_livraison,
            numero_suivi: None,
            transporteur: None,
            statut:  StatutLivraison::EnTransit,
            date_livraison_estimee: None,
            date_livraison_reelle: None,
            cout_livraison,
            date_creation: now,
            date_mise_a_jour: now,
        }
    }
}


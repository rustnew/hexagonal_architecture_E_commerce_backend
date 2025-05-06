-- Activer l'extension UUID
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Table: Utilisateurs
CREATE TABLE utilisateurs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL UNIQUE,
    mot_de_passe_hash VARCHAR(255) NOT NULL,
    prenom VARCHAR(100) NOT NULL,
    nom VARCHAR(100) NOT NULL,
    telephone VARCHAR(20),
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    date_mise_a_jour TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    est_actif BOOLEAN NOT NULL DEFAULT TRUE,
    email_verifie BOOLEAN NOT NULL DEFAULT FALSE
);

-- Table: Catégories
CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom VARCHAR(50) NOT NULL,
    description TEXT,
    categorie_parente_id UUID REFERENCES categories(id),
    url_image VARCHAR(255),
    est_active BOOLEAN NOT NULL DEFAULT TRUE,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    date_mise_a_jour TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Produits
CREATE TABLE produits (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom VARCHAR(100) NOT NULL,
    description TEXT NOT NULL,
    description_courte VARCHAR(160),
    reference VARCHAR(50) UNIQUE NOT NULL,
    prix DECIMAL(12, 2) NOT NULL CHECK (prix > 0),
    prix_comparatif DECIMAL(12, 2),
    prix_revient DECIMAL(12, 2),
    quantite INTEGER NOT NULL DEFAULT 0 CHECK (quantite >= 0),
    categorie_id UUID REFERENCES categories(id),
    est_publie BOOLEAN NOT NULL DEFAULT TRUE,
    est_en_vedette BOOLEAN NOT NULL DEFAULT FALSE,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    date_mise_a_jour TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Images Produit
CREATE TABLE images_produit (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    produit_id UUID NOT NULL REFERENCES produits(id) ON DELETE CASCADE,
    url VARCHAR(255) NOT NULL,
    texte_alternatif VARCHAR(125),
    est_principale BOOLEAN NOT NULL DEFAULT FALSE,
    ordre_tri INTEGER NOT NULL DEFAULT 0,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Variantes Produit
CREATE TABLE variantes_produit (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    produit_id UUID NOT NULL REFERENCES produits(id) ON DELETE CASCADE,
    nom VARCHAR(50) NOT NULL, -- ex: "Couleur", "Taille"
    valeur VARCHAR(50) NOT NULL, -- ex: "Rouge", "XL"
    reference VARCHAR(50),
    ajustement_prix DECIMAL(10, 2) DEFAULT 0.00,
    quantite INTEGER DEFAULT 0,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(produit_id, nom, valeur)
);

-- Table: Adresses
CREATE TABLE adresses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id UUID NOT NULL REFERENCES utilisateurs(id),
    prenom VARCHAR(100) NOT NULL,
    nom VARCHAR(100) NOT NULL,
    ligne1 VARCHAR(255) NOT NULL,
    ville VARCHAR(100) NOT NULL,
    region VARCHAR(100) NOT NULL,
    lieu_proche_connus VARCHAR(20) NOT NULL,
    quartier VARCHAR(100) NOT NULL,
    est_principale BOOLEAN DEFAULT FALSE,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    date_mise_a_jour TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Paniers
CREATE TABLE paniers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id UUID REFERENCES utilisateurs(id),
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    date_mise_a_jour TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Articles Panier
CREATE TABLE articles_panier (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    panier_id UUID NOT NULL REFERENCES paniers(id) ON DELETE CASCADE,
    produit_id UUID NOT NULL REFERENCES produits(id),
    variante_id UUID REFERENCES variantes_produit(id),
    quantite INTEGER NOT NULL CHECK (quantite > 0),
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    date_mise_a_jour TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Commandes
CREATE TABLE commandes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id UUID REFERENCES utilisateurs(id),
    numero_commande VARCHAR(20) UNIQUE NOT NULL,
    statut VARCHAR(20) NOT NULL DEFAULT 'en_attente'
        CHECK (statut IN ('en_attente', 'en_traitement', 'expediee', 'livree', 'annulee', 'remboursee')),
    sous_total DECIMAL(12, 2) NOT NULL,
    montant_taxes DECIMAL(12, 2) NOT NULL DEFAULT 0.00,
    frais_livraison DECIMAL(12, 2) NOT NULL DEFAULT 0.00,
    montant_remise DECIMAL(12, 2) NOT NULL DEFAULT 0.00,
    montant_total DECIMAL(12, 2) NOT NULL,
    adresse_livraison_id UUID NOT NULL REFERENCES adresses(id),
    note TEXT,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    date_mise_a_jour TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Articles Commande
CREATE TABLE articles_commande (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    commande_id UUID NOT NULL REFERENCES commandes(id) ON DELETE CASCADE,
    produit_id UUID NOT NULL REFERENCES produits(id),
    variante_id UUID REFERENCES variantes_produit(id),
    nom_produit VARCHAR(255) NOT NULL,
    description_variante VARCHAR(255),
    prix DECIMAL(12, 2) NOT NULL,
    quantite INTEGER NOT NULL,
    montant_remise DECIMAL(12, 2) DEFAULT 0.00,
    prix_total DECIMAL(12, 2) NOT NULL
);

-- Table: Paiements
CREATE TABLE paiements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    commande_id UUID NOT NULL REFERENCES commandes(id),
    montant DECIMAL(12, 2) NOT NULL,
    methode_paiement VARCHAR(50) NOT NULL,
    id_transaction VARCHAR(100),
    statut VARCHAR(20) NOT NULL DEFAULT 'en_attente'
        CHECK (statut IN ('en_attente', 'complete', 'echoue', 'remboursee')),
    devise VARCHAR(3) NOT NULL DEFAULT 'EUR',
    details_paiement JSONB,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    date_mise_a_jour TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Livraisons
CREATE TABLE livraisons (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    commande_id UUID NOT NULL REFERENCES commandes(id),
    methode_livraison VARCHAR(50) NOT NULL,
    numero_suivi VARCHAR(100),
    transporteur VARCHAR(50),
    statut VARCHAR(20) NOT NULL DEFAULT 'en_preparation'
        CHECK (statut IN ('en_preparation', 'expediee', 'en_transit', 'livree', 'echouee')),
    date_livraison_estimee TIMESTAMPTZ,
    date_livraison_reelle TIMESTAMPTZ,
    cout_livraison DECIMAL(10, 2) NOT NULL,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    date_mise_a_jour TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Fonction et Déclencheur pour date_mise_a_jour
CREATE OR REPLACE FUNCTION mettre_a_jour_date()
RETURNS TRIGGER AS $$
BEGIN
    NEW.date_mise_a_jour = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Application des déclencheurs aux tables
CREATE TRIGGER mettre_a_jour_utilisateurs
BEFORE UPDATE ON utilisateurs
FOR EACH ROW EXECUTE FUNCTION mettre_a_jour_date();

CREATE TRIGGER mettre_a_jour_categories
BEFORE UPDATE ON categories
FOR EACH ROW EXECUTE FUNCTION mettre_a_jour_date();

CREATE TRIGGER mettre_a_jour_produits
BEFORE UPDATE ON produits
FOR EACH ROW EXECUTE FUNCTION mettre_a_jour_date();

CREATE TRIGGER mettre_a_jour_paniers
BEFORE UPDATE ON paniers
FOR EACH ROW EXECUTE FUNCTION mettre_a_jour_date();

CREATE TRIGGER mettre_a_jour_articles_panier
BEFORE UPDATE ON articles_panier
FOR EACH ROW EXECUTE FUNCTION mettre_a_jour_date();

CREATE TRIGGER mettre_a_jour_commandes
BEFORE UPDATE ON commandes
FOR EACH ROW EXECUTE FUNCTION mettre_a_jour_date();

CREATE TRIGGER mettre_a_jour_paiements
BEFORE UPDATE ON paiements
FOR EACH ROW EXECUTE FUNCTION mettre_a_jour_date();

CREATE TRIGGER mettre_a_jour_livraisons
BEFORE UPDATE ON livraisons
FOR EACH ROW EXECUTE FUNCTION mettre_a_jour_date();

-- Index recommandés
CREATE INDEX idx_produits_categorie ON produits(categorie_id);
CREATE INDEX idx_produits_slug ON produits(slug);
CREATE INDEX idx_produits_publies ON produits(est_publie) WHERE est_publie = TRUE;

CREATE INDEX idx_paniers_utilisateur ON paniers(utilisateur_id);
CREATE INDEX idx_paniers_session ON paniers(id_session) WHERE id_session IS NOT NULL;
CREATE INDEX idx_articles_panier_produit ON articles_panier(produit_id);

CREATE INDEX idx_commandes_utilisateur ON commandes(utilisateur_id);
CREATE INDEX idx_commandes_statut ON commandes(statut);
CREATE INDEX idx_commandes_numero ON commandes(numero_commande);

CREATE INDEX idx_articles_commande_commande ON articles_commande(commande_id);
CREATE INDEX idx_articles_commande_produit ON articles_commande(produit_id);

CREATE INDEX idx_paiements_commande ON paiements(commande_id);
CREATE INDEX idx_paiements_statut ON paiements(statut);

CREATE INDEX idx_livraisons_commande ON livraisons(commande_id);
CREATE INDEX idx_livraisons_suivi ON livraisons(numero_suivi) WHERE numero_suivi IS NOT NULL;

CREATE TYPE type_notification AS ENUM (
    'commande_statut',
    'livraison_statut',
    'promotion'
    "nouveau_produit"
);

CREATE TABLE notifications (
    id UUID PRIMARY KEY,
    utilisateur_id UUID NOT NULL REFERENCES utilisateurs(id),
    type_notification type_notification NOT NULL,
    contenu TEXT NOT NULL,
    est_lue BOOLEAN NOT NULL DEFAULT false,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    date_lue TIMESTAMPTZ,
    metadata JSONB
);

CREATE INDEX idx_notifications_utilisateur ON notifications(utilisateur_id);
CREATE INDEX idx_notifications_lues ON notifications(utilisateur_id) WHERE est_lue = false;

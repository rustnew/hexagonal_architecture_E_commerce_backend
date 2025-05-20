-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Table: Utilisateurs
-- Stores user information
CREATE TABLE utilisateurs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL UNIQUE,
    mot_de_passe VARCHAR(255) NOT NULL,
    prenom VARCHAR(100) NOT NULL,
    nom VARCHAR(100) NOT NULL,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Sessions
-- Manages sessions for authenticated and guest users
CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    token VARCHAR(255) NOT NULL UNIQUE,
    utilisateur_id UUID REFERENCES utilisateurs(id),
    date_expiration TIMESTAMPTZ NOT NULL,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Catégories
-- Stores product categories
CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom VARCHAR(50) NOT NULL,
    description TEXT,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Produits
-- Stores product information
CREATE TABLE produits (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom VARCHAR(100) NOT NULL,
    description TEXT NOT NULL,
    reference VARCHAR(50) UNIQUE NOT NULL,
    prix DECIMAL(12, 2) NOT NULL CHECK (prix > 0),
    quantite INTEGER NOT NULL DEFAULT 0 CHECK (quantite >= 0),
    categorie_id UUID REFERENCES categories(id) ON DELETE SET NULL,
    image_principale_url VARCHAR(255),
    est_publie BOOLEAN NOT NULL DEFAULT TRUE,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Variantes Produit
-- Stores product variants (e.g., color, size)
CREATE TABLE variantes_produit (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    produit_id UUID NOT NULL REFERENCES produits(id) ON DELETE CASCADE,
    nom VARCHAR(50) NOT NULL, -- ex: "Couleur", "Taille"
    valeur VARCHAR(50) NOT NULL, -- ex: "Rouge", "XL"
    prix_ajuste DECIMAL(10, 2) DEFAULT 0.00,
    quantite INTEGER NOT NULL DEFAULT 0 CHECK (quantite >= 0),
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(produit_id, nom, valeur)
);

-- Table: Avis (Reviews)
-- Stores product reviews
CREATE TABLE reviews (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    produit_id UUID NOT NULL REFERENCES produits(id) ON DELETE CASCADE,
    utilisateur_id UUID NOT NULL REFERENCES utilisateurs(id),
    note INTEGER NOT NULL CHECK (note >= 1 AND note <= 5),
    commentaire TEXT,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(produit_id, utilisateur_id)
);

-- Table: Liste de souhaits (Wishlist)
-- Stores user wishlists
CREATE TABLE wishlist (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id UUID NOT NULL REFERENCES utilisateurs(id),
    produit_id UUID NOT NULL REFERENCES produits(id) ON DELETE CASCADE,
    date_ajout TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(utilisateur_id, produit_id)
);

-- Table: Promotions
-- Stores promotional discounts
CREATE TABLE promotions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code VARCHAR(20) NOT NULL UNIQUE,
    description TEXT,
    pourcentage_remise DECIMAL(5, 2) CHECK (pourcentage_remise >= 0 AND pourcentage_remise <= 100),
    montant_remise DECIMAL(12, 2) CHECK (montant_remise >= 0),
    date_debut TIMESTAMPTZ NOT NULL,
    date_fin TIMESTAMPTZ NOT NULL,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK ((pourcentage_remise IS NOT NULL AND montant_remise IS NULL) OR 
           (pourcentage_remise IS NULL AND montant_remise IS NOT NULL))
);

-- Table: Méthodes de paiement
-- Stores payment methods
CREATE TABLE methodes_paiement (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom VARCHAR(50) NOT NULL, -- ex: "Carte bancaire", "PayPal"
    description TEXT,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Méthodes de livraison
-- Stores delivery methods
CREATE TABLE methodes_livraison (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom VARCHAR(50) NOT NULL, -- ex: "Colissimo", "Chronopost"
    description TEXT,
    cout DECIMAL(10, 2) NOT NULL DEFAULT 0.00,
    delai_estime VARCHAR(50), -- ex: "2-3 jours"
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Adresses
-- Stores user addresses
CREATE TABLE adresses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id UUID NOT NULL REFERENCES utilisateurs(id),
    prenom VARCHAR(100) NOT NULL,
    nom VARCHAR(100) NOT NULL,
    ligne1 VARCHAR(255) NOT NULL,
    ville VARCHAR(100) NOT NULL,
    region VARCHAR(100) NOT NULL,
    code_postal VARCHAR(20),
    pays VARCHAR(100) NOT NULL DEFAULT 'France',
    est_principale BOOLEAN DEFAULT FALSE,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Paniers Articles
-- Stores cart items for users or sessions
CREATE TABLE paniers_articles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id UUID REFERENCES utilisateurs(id),
    session_id UUID REFERENCES sessions(id),
    produit_id UUID NOT NULL REFERENCES produits(id),
    variante_id UUID REFERENCES variantes_produit(id),
    quantite INTEGER NOT NULL CHECK (quantite > 0),
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(utilisateur_id, session_id, produit_id, variante_id),
    CHECK (utilisateur_id IS NOT NULL OR session_id IS NOT NULL)
);

-- Table: Commandes
-- Stores order information
CREATE TABLE commandes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id UUID REFERENCES utilisateurs(id),
    numero_commande VARCHAR(20) UNIQUE NOT NULL,
    statut VARCHAR(20) NOT NULL DEFAULT 'en_attente'
    CHECK (statut IN ('en_attente', 'en_traitement', 'expediee', 'livree', 'annulee', 'remboursee')),
    montant_total DECIMAL(12, 2) NOT NULL CHECK (montant_total >= 0),
    promotion_id UUID REFERENCES promotions(id),
    lieu_publique_proche UUID NOT NULL REFERENCES adresses(id),
    methode_paiement_id UUID REFERENCES methodes_paiement(id),
    statut_paiement VARCHAR(20) DEFAULT 'en_attente'
    CHECK (statut_paiement IN ('en_attente', 'complete', 'echoue', 'remboursee')),
    id_transaction VARCHAR(100),
    methode_livraison_id UUID REFERENCES methodes_livraison(id),
    statut_livraison VARCHAR(20) DEFAULT 'en_preparation'
    CHECK (statut_livraison IN ('en_preparation', 'expediee', 'en_transit', 'livree', 'echouee')),
    numero_suivi VARCHAR(100),
    devise VARCHAR(3) NOT NULL DEFAULT 'EUR',
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Articles Commande
-- Stores items in an order
CREATE TABLE articles_commande (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    commande_id UUID NOT NULL REFERENCES commandes(id) ON DELETE CASCADE,
    produit_id UUID NOT NULL REFERENCES produits(id),
    variante_id UUID REFERENCES variantes_produit(id),
    nom_produit VARCHAR(255) NOT NULL,
    prix_unitaire DECIMAL(12, 2) NOT NULL CHECK (prix_unitaire >= 0),
    quantite INTEGER NOT NULL CHECK (quantite > 0),
    prix_total DECIMAL(12, 2) NOT NULL CHECK (prix_total >= 0)
);

-- Table: Notifications
-- Stores user notifications
CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id UUID NOT NULL REFERENCES utilisateurs(id),
    type VARCHAR(50) NOT NULL CHECK (type IN ('commande_statut', 'livraison_statut', 'promotion', 'autre')),
    contenu TEXT NOT NULL,
    est_lue BOOLEAN NOT NULL DEFAULT FALSE,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    metadata JSONB
);

-- Indexes for performance
CREATE INDEX idx_utilisateurs_email ON utilisateurs(email);
CREATE INDEX idx_sessions_utilisateur ON sessions(utilisateur_id);
CREATE INDEX idx_produits_categorie ON produits(categorie_id);
CREATE INDEX idx_produits_reference ON produits(reference);
CREATE INDEX idx_produits_publies ON produits(est_publie) WHERE est_publie = TRUE;
CREATE INDEX idx_variantes_produit ON variantes_produit(produit_id);
CREATE INDEX idx_reviews_produit ON reviews(produit_id);
CREATE INDEX idx_wishlist_utilisateur ON wishlist(utilisateur_id);
CREATE INDEX idx_promotions_code ON promotions(code);
CREATE INDEX idx_adresses_utilisateur ON adresses(utilisateur_id);
CREATE INDEX idx_paniers_articles_utilisateur ON paniers_articles(utilisateur_id);
CREATE INDEX idx_paniers_articles_session ON paniers_articles(session_id);
CREATE INDEX idx_commandes_utilisateur ON commandes(utilisateur_id);
CREATE INDEX idx_commandes_numero ON commandes(numero_commande);
CREATE INDEX idx_commandes_promotion ON commandes(promotion_id);
CREATE INDEX idx_articles_commande_commande ON articles_commande(commande_id);
CREATE INDEX idx_notifications_utilisateur ON notifications(utilisateur_id);
CREATE INDEX idx_notifications_lues ON notifications(utilisateur_id) WHERE est_lue = FALSE;
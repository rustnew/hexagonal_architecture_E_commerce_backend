-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Table: Users
-- Stores user information
CREATE TABLE utilisateur (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL UNIQUE,
    mot_de_passe VARCHAR(255) NOT NULL,
    prenom VARCHAR(100) NOT NULL,
    nom VARCHAR(100) NOT NULL,
    role VARCHAR(20) NOT NULL,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    date_update TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Sessions
CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    token VARCHAR(255) NOT NULL UNIQUE,
    utilisateur_id UUID REFERENCES utilisateur(id),  -- CORRIGÉ: utilisateur au lieu de user
    date_expiration TIMESTAMPTZ NOT NULL,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Categories
-- Stores product categories
CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom VARCHAR(50) NOT NULL,
    description TEXT,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Products
-- Stores product information
CREATE TABLE products (
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

-- Table: Product Variants
-- Stores product variants (e.g., color, size)
CREATE TABLE product_variants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    nom VARCHAR(50) NOT NULL, -- ex: "Couleur", "Taille"
    valeur VARCHAR(50) NOT NULL, -- ex: "Rouge", "XL"
    prix_ajuste DECIMAL(10, 2) DEFAULT 0.00,
    quantite INTEGER NOT NULL DEFAULT 0 CHECK (quantite >= 0),
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(product_id, nom, valeur)
);

-- Table: Reviews
-- Stores product reviews
CREATE TABLE reviews (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    utilisateur_id UUID NOT NULL REFERENCES utilisateur(id), -- CORRIGÉ: utilisateur au lieu de user
    note INTEGER NOT NULL CHECK (note >= 1 AND note <= 5),
    commentaire TEXT,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(product_id, utilisateur_id)
);

-- Table: Wishlist
-- Stores user wishlists
CREATE TABLE wishlist (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id UUID NOT NULL REFERENCES utilisateur(id), -- CORRIGÉ: utilisateur au lieu de user
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    date_ajout TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(utilisateur_id, product_id)
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

-- Table: Payment Methods
-- Stores payment methods
CREATE TABLE payment_methods (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom VARCHAR(50) NOT NULL, -- ex: "Carte bancaire", "PayPal"
    description TEXT,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Delivery Methods
-- Stores delivery methods
CREATE TABLE delivery_methods (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom VARCHAR(50) NOT NULL, -- ex: "Colissimo", "Chronopost"
    description TEXT,
    cout DECIMAL(10, 2) NOT NULL DEFAULT 0.00,
    delai_estime VARCHAR(50), -- ex: "2-3 jours"
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Addresses
-- Stores user addresses
CREATE TABLE addresses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id UUID NOT NULL REFERENCES utilisateur(id), -- CORRIGÉ: utilisateur au lieu de user
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

-- Table: Cart Items
-- Stores cart items for users or sessions
CREATE TABLE cart_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id UUID REFERENCES utilisateur(id), -- CORRIGÉ: utilisateur au lieu de user
    session_id UUID REFERENCES sessions(id),
    product_id UUID NOT NULL REFERENCES products(id),
    variante_id UUID REFERENCES product_variants(id),
    quantite INTEGER NOT NULL CHECK (quantite > 0),
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(utilisateur_id, session_id, product_id, variante_id),
    CHECK (utilisateur_id IS NOT NULL OR session_id IS NOT NULL)
);

-- Table: Orders
-- Stores order information
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id UUID REFERENCES utilisateur(id), -- CORRIGÉ: utilisateur au lieu de user
    numero_commande VARCHAR(20) UNIQUE NOT NULL,
    statut VARCHAR(20) NOT NULL DEFAULT 'en_attente'
        CHECK (statut IN ('en_attente', 'en_traitement', 'expediee', 'livree', 'annulee', 'remboursee')),
    montant_total DECIMAL(12, 2) NOT NULL CHECK (montant_total >= 0),
    promotion_id UUID REFERENCES promotions(id),
    lieu_publique_proche UUID NOT NULL REFERENCES addresses(id),
    methode_paiement_id UUID REFERENCES payment_methods(id),
    statut_paiement VARCHAR(20) DEFAULT 'en_attente'
        CHECK (statut_paiement IN ('en_attente', 'complete', 'echoue', 'remboursee')),
    id_transaction VARCHAR(100),
    methode_livraison_id UUID REFERENCES delivery_methods(id),
    statut_livraison VARCHAR(20) DEFAULT 'en_preparation'
        CHECK (statut_livraison IN ('en_preparation', 'expediee', 'en_transit', 'livree', 'echouee')),
    numero_suivi VARCHAR(100),
    devise VARCHAR(3) NOT NULL DEFAULT 'EUR',
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Table: Order Items
-- Stores items in an order
CREATE TABLE order_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id),
    variante_id UUID REFERENCES product_variants(id),
    nom_produit VARCHAR(255) NOT NULL,
    prix_unitaire DECIMAL(12, 2) NOT NULL CHECK (prix_unitaire >= 0),
    quantite INTEGER NOT NULL CHECK (quantite > 0),
    prix_total DECIMAL(12, 2) NOT NULL CHECK (prix_total >= 0)
);

-- Table: Notifications
-- Stores user notifications
CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id UUID NOT NULL REFERENCES utilisateur(id), -- CORRIGÉ: utilisateur au lieu de user
    type VARCHAR(50) NOT NULL CHECK (type IN ('commande_statut', 'livraison_statut', 'promotion', 'autre')),
    contenu TEXT NOT NULL,
    est_lue BOOLEAN NOT NULL DEFAULT FALSE,
    date_creation TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    metadata JSONB
);
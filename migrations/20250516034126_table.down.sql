-- Add down migration script here
-- Suppression des tables dans l'ordre inverse des dépendances

DROP TABLE IF EXISTS notifications;
DROP TABLE IF EXISTS articles_commande;
DROP TABLE IF EXISTS commandes;
DROP TABLE IF EXISTS paniers_articles;
DROP TABLE IF EXISTS adresses;
DROP TABLE IF EXISTS methodes_livraison;
DROP TABLE IF EXISTS methodes_paiement;
DROP TABLE IF EXISTS promotions;
DROP TABLE IF EXISTS wishlist;
DROP TABLE IF EXISTS reviews;
DROP TABLE IF EXISTS variantes_produit;
DROP TABLE IF EXISTS produits;
DROP TABLE IF EXISTS categories;
DROP TABLE IF EXISTS sessions;
DROP TABLE IF EXISTS utilisateurs;

-- Désactiver l'extension UUID
DROP EXTENSION IF EXISTS "uuid-ossp";
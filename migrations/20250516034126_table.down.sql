-- Suppression des tables dans l'ordre inverse des dépendances
DROP TABLE notifications;
DROP TABLE order_items;
DROP TABLE orders;
DROP TABLE cart_items;
DROP TABLE addresses;
DROP TABLE delivery_methods;
DROP TABLE payment_methods;
DROP TABLE promotions;
DROP TABLE wishlist;
DROP TABLE reviews;
DROP TABLE product_variants;
DROP TABLE products;
DROP TABLE categories;
DROP TABLE sessions;
DROP TABLE utilisateur;

-- Désactiver l'extension UUID
DROP EXTENSION IF EXISTS "uuid-ossp";
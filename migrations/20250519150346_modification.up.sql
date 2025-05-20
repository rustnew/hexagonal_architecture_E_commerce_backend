-- Add up migration script here
ALTER TABLE utilisateurs
ADD COLUMN role VARCHAR(50) NOT NULL DEFAULT 'utilisateurs';
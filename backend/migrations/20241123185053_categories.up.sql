-- Add up migration script here
DROP TABLE IF EXISTS "categories";
CREATE TABLE "categories" (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT
);
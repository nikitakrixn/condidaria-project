-- Add up migration script here
DROP TABLE IF EXISTS "products";
CREATE TABLE "products" (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price DOUBLE PRECISION NOT NULL,
    category_id UUID NOT NULL,
    image_url VARCHAR(255)
);
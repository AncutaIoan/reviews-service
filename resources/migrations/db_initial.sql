CREATE TABLE IF NOT EXISTS reviews (
                         id SERIAL PRIMARY KEY,            -- Auto-incremented ID for the review
                         added_by VARCHAR(255) NOT NULL,    -- User who added the review
                         added_at VARCHAR(255) NOT NULL,      -- Timestamp when the review was added
                         rating INTEGER NOT NULL,          -- Rating value (e.g., 1-5)
                         entity_type VARCHAR(255) NOT NULL, -- Type of entity being reviewed (e.g., 'Product', 'Person', etc.)
                         entity_id VARCHAR(255) NOT NULL   -- ID of the entity being reviewed (e.g., product ID, person ID, etc.)
);


CREATE TABLE IF NOT EXISTS users (
                       id SERIAL PRIMARY KEY,
                       name TEXT NOT NULL,
                       username TEXT UNIQUE NOT NULL,
                       email TEXT UNIQUE NOT NULL,
                       password TEXT NOT NULL,
                       phone_number TEXT UNIQUE NOT NULL,
                       created_at VARCHAR(255),
                       updated_at VARCHAR(255)
);
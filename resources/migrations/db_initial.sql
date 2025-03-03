CREATE TABLE IF NOT EXISTS reviews (
                         id SERIAL PRIMARY KEY,
                         added_by VARCHAR(255) NOT NULL,
                         added_at TIMESTAMP NOT NULL DEFAULT NOW(),
                         rating INTEGER NOT NULL,
                         entity_type VARCHAR(255) NOT NULL,
                         entity_id VARCHAR(255) NOT NULL
);


CREATE TABLE IF NOT EXISTS users (
                       id SERIAL PRIMARY KEY,
                       name TEXT NOT NULL,
                       username TEXT UNIQUE NOT NULL,
                       email TEXT UNIQUE NOT NULL,
                       password TEXT NOT NULL,
                       phone_number TEXT UNIQUE NOT NULL,
                       created_at TIMESTAMP DEFAULT NOW(),
                       updated_at TIMESTAMP DEFAULT NOW()
);
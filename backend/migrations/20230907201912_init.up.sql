-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
        user_id SERIAL PRIMARY KEY,
        username TEXT NOT NULL UNIQUE,
        email TEXT NOT NULL UNIQUE,
        password TEXT NOT NULL,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
    );

CREATE TABLE IF NOT EXISTS engagements (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        rating INTEGER NOT NULL,
        text TEXT NOT NULL UNIQUE,
        user_id INTEGER DEFAULT NULL,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        CONSTRAINT fk_user
            FOREIGN KEY(user_id) 
	            REFERENCES users(user_id)
    );

INSERT INTO users (user_id, username, email, password) 
VALUES 
(1, 'jim_jam', 'jim@jam.com', 'hashthissoon'),
(2, 'aaron', 'aaron@site.com', 'hashthissoon');

INSERT INTO engagements (rating, text, user_id) 
VALUES 
(7, 'It was a seven.', 1),
(3, 'I give it a 3', 2);
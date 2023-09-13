-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
DROP TYPE IF EXISTS consultant_specialty;
DROP TYPE IF EXISTS consultant_territory;
DROP TYPE IF EXISTS state_abbr;
DROP TYPE IF EXISTS state_name;
DROP TYPE IF EXISTS us_territories;

CREATE TYPE consultant_specialty AS ENUM ('Insurance', 'Finance', 'Government');

CREATE TYPE consultant_territory AS ENUM ('Midwest', 'East', 'West', 'North', 'South');

CREATE TYPE state_abbr AS ENUM ('AL','AK','AZ','AR','CA','CO','CT','DE','FL','GA','HI','ID','IL','IN','IA','KS','KY','LA','ME','MD','MA',
        'MI','MN','MS','MO','MT','NE','NV','NH','NJ','NM','NY','NC','ND','OH','OK','OR','PA','RI','SC','SD','TN',
        'TX','UT','VT','VA','WA','WV','WI','WY','AS','GU','MP','PR','VI','DC');

CREATE TYPE state_name AS ENUM ('Alabama','Alaska','Arizona','Arkansas','California','Colorado','Connecticut','Delaware','Florida','Georgia',
        'Hawaii','Idaho','Illinois','Indiana','Iowa','Kansas','Kentucky','Louisiana','Maine','Maryland','Massachusetts',
        'Michigan','Minnesota','Mississippi','Missouri','Montana','Nebraska','Nevada','New_Hampshire','New_Jersey','New_Mexico',
        'New_York','North_Carolina','North_Dakota','Ohio','Oklahoma','Oregon','Pennsylvania','Rhode_Island','South_Carolina',
        'South_Dakota','Tennessee','Texas','Utah','Vermont','Virginia','Washington','West_Virginia','Wisconsin','Wyoming');

CREATE TYPE us_territories AS ENUM ('American_Samoa', 'Guam', 'Northern_Mariana_Islands', 'Puerto_Rico', 'Virgin_Islands', 'District_of_Columbia');

CREATE TABLE IF NOT EXISTS users (
        user_id SERIAL PRIMARY KEY,
        username TEXT NOT NULL UNIQUE,
        email TEXT NOT NULL UNIQUE,
        secret TEXT DEFAULT NULL,
        password TEXT NOT NULL,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
    );

CREATE TABLE IF NOT EXISTS consultants (
        consultant_id SERIAL PRIMARY KEY,
        specialty consultant_specialty NOT NULL,
        territory consultant_territory NULL,
        user_id INTEGER NOT NULL,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        CONSTRAINT fk_user
            FOREIGN KEY(user_id) 
	            REFERENCES users(user_id)
    );

CREATE TABLE IF NOT EXISTS clients (
        client_id SERIAL PRIMARY KEY,
        client_slug TEXT NOT NULL DEFAULT (uuid_generate_v4()),
        client_address_one TEXT NOT NULL,
        client_address_two TEXT NULL,
        client_city TEXT NOT NULL,
        client_state state_abbr NOT NULL,
        client_zip VARCHAR (5) NOT NULL,
        client_home_phone TEXT NULL,
        client_mobile_phone TEXT NULL,
        client_office_phone TEXT NULL,
        client_email TEXT NULL,
        user_id INTEGER NOT NULL,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        CONSTRAINT fk_user
            FOREIGN KEY(user_id) 
	            REFERENCES users(user_id)
    );

CREATE TABLE IF NOT EXISTS engagements (
        engagement_id SERIAL PRIMARY KEY,
        rating INTEGER NOT NULL,
        text TEXT NOT NULL UNIQUE,
        user_id INTEGER DEFAULT NULL,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        CONSTRAINT fk_user
            FOREIGN KEY(user_id) 
	            REFERENCES users(user_id)
    );

CREATE TABLE IF NOT EXISTS messages (
        message_id SERIAL PRIMARY KEY,
        content TEXT NOT NULL,
        subject TEXT NOT NULL,
        sent_to INTEGER NOT NULL,
        sent_from INTEGER NOT NULL,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        sent_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
        read_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
        CONSTRAINT fk_sent_to
            FOREIGN KEY(sent_to) 
	            REFERENCES users(user_id),
        CONSTRAINT fk_sent_from
            FOREIGN KEY(sent_from) 
	            REFERENCES users(user_id)
    );

CREATE TABLE IF NOT EXISTS consults (
        consult_id SERIAL PRIMARY KEY,
        consultant_id INTEGER NOT NULL,
        client_id INTEGER NOT NULL,
        consult_location TEXT NOT NULL,
        consult_start TIMESTAMP WITH TIME ZONE DEFAULT NULL,
        consult_end TIMESTAMP WITH TIME ZONE DEFAULT NULL,
        notes TEXT NULL,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        CONSTRAINT fk_client
            FOREIGN KEY(client_id) 
	            REFERENCES clients(client_id),
        CONSTRAINT fk_consultant
            FOREIGN KEY(consultant_id) 
	            REFERENCES consultants(consultant_id)
    );

INSERT INTO users (user_id, username, email, password) 
VALUES 
(1, 'root', 'root@consultancy.com', 'hashthissoon'),
(2, 'admin', 'admin@consultancy.com', 'hashthissoon'),
-- Users
(3, 'jim_jam', 'jim@jam.com', 'hashthissoon'),
(4, 'aaron', 'aaron@aaron.com', 'hashthissoon'),
-- Clients
(5, 'first_client', 'client_one@client.com', 'hashthissoon'),
(6, 'second_client', 'client_two@client.com', 'hashthissoon'),
-- Consultants
(7, 'first_consultant', 'consultant_one@consultancy.com', 'hashthissoon'),
(8, 'second_consultant', 'consultant_two@consultancy.com', 'hashthissoon');


INSERT INTO clients (client_id, client_address_one, client_city, client_state, client_zip, user_id) 
VALUES 
(1, '1111 Client St.', 'Client City', 'NE', '68114', 5),
(2, '2222 Client St.', 'Client Town', 'MN', '55057', 6);

INSERT INTO consultants (consultant_id, specialty, user_id) 
VALUES 
(1, 'Finance', 7),
(2, 'Insurance', 8);

INSERT INTO engagements (rating, text, user_id) 
VALUES 
(7, 'It was a seven.', 1),
(3, 'I give it a 3', 2);

INSERT INTO consults (consult_id, consultant_id, client_id, consult_location, consult_start, consult_end) 
VALUES 
(1, 1, 1, 'Consult Location #1', '2023-09-11 19:10:25-06', '2023-09-11 19:30:25-06'),
(2, 2, 2, 'Consult Location #2', '2023-09-11 16:00:25-06', '2023-09-11 16:50:11-06');
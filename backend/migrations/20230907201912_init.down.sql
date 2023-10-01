-- Add down migration script here
DROP TABLE IF EXISTS consultant_ties;
DROP TABLE IF EXISTS consult_attachments;
DROP TABLE IF EXISTS engagements;
DROP TABLE IF EXISTS consults;
DROP TABLE IF EXISTS clients;
-- Tables depends on these
DROP TABLE IF EXISTS messages;
DROP TABLE IF EXISTS attachments;
DROP TABLE IF EXISTS locations;
DROP TABLE IF EXISTS contacts;
DROP TABLE IF EXISTS consultants;
DROP TABLE IF EXISTS users;

DROP TABLE IF EXISTS territories;
DROP TABLE IF EXISTS specialties;
-- This needs to be last
DROP TABLE IF EXISTS accounts;

DROP TYPE IF EXISTS consultant_specialty;
DROP TYPE IF EXISTS consultant_territory;
DROP TYPE IF EXISTS state_abbr;
DROP TYPE IF EXISTS state_name;
DROP TYPE IF EXISTS us_territories;
DROP TYPE IF EXISTS attachment_channel;
DROP TYPE IF EXISTS mime_type;
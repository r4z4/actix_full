-- Add down migration script here

DROP TABLE IF EXISTS engagements;
DROP TABLE IF EXISTS consults;
-- Tables depends on these
DROP TABLE IF EXISTS clients;
DROP TABLE IF EXISTS consultants;
-- This needs to be last
DROP TABLE IF EXISTS users;
-- Add migration script her-- SQLite migration: create 'User' table
CREATE TABLE user (
    id TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    email TEXT NOT NULL
);

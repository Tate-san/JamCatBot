-- Add migration script here
CREATE TABLE IF NOT EXISTS watched_anime (
	id          INTEGER PRIMARY KEY NOT NULL,
	name        TEXT    UNIQUE 		NOT NULL
);

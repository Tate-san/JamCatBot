CREATE TABLE IF NOT EXISTS watched_anime_guild (
	id INTEGER PRIMARY KEY NOT NULL,
	watched_anime_id INTEGER NOT NULL,
	guild_id INTEGER NOT NULL,

	unique (watched_anime_id, guild_id),

	CONSTRAINT fk_watched_anime
		FOREIGN KEY (watched_anime_id)
		REFERENCES watched_anime(id)
);

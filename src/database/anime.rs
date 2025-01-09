use super::prelude::*;

#[derive(Debug, Deserialize, FromRow)]
struct AnimeDB {
    pub id: i64,
    pub name: String,
}

impl AnimeDB {
    pub async fn insert_anime(pool: &sqlx::SqlitePool, name: &str) -> sqlx::Result<()> {
        sqlx::query(
            "
            INSERT OR IGNORE INTO watched_anime (name)
                VALUES($1)
            ",
        )
        .bind(name)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn find_anime_by_name(pool: &sqlx::SqlitePool, name: &str) -> sqlx::Result<Self> {
        sqlx::query_as(
            "
            SELECT * FROM watched_anime WHERE name=$1
            ",
        )
        .bind(name)
        .fetch_one(pool)
        .await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedAnime {
    pub id: i64,
    pub watched_anime: String,
    pub guild_id: i64,
}

impl WatchedAnime {
    pub async fn insert_watched_anime(
        pool: &sqlx::SqlitePool,
        anime: &str,
        guild_id: u64,
    ) -> sqlx::Result<()> {
        let anime: AnimeDB = match AnimeDB::find_anime_by_name(pool, anime).await {
            Ok(row) => row,
            Err(_) => {
                AnimeDB::insert_anime(pool, anime).await?;
                AnimeDB::find_anime_by_name(pool, anime).await?
            }
        };

        sqlx::query(
            "
            INSERT INTO watched_anime_guild (watched_anime_id, guild_id)
                VALUES($1, $2)
            ",
        )
        .bind(anime.id)
        .bind(guild_id as i64)
        .execute(pool)
        .await?;

        Ok(())
    }
}

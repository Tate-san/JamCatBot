pub mod guild;
pub mod music;
pub mod queue;

use guild::GuildCacheMap;

pub use jamcat_error::BotError;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Data {
    pub http: reqwest::Client,
    pub songbird: Arc<songbird::Songbird>,
    pub guild_cache: Arc<Mutex<GuildCacheMap>>,
}

pub type Error = BotError;
pub type Context<'a> = poise::Context<'a, Data, Error>;


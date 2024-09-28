pub mod guild;

use guild::GuildCacheMap;

pub use crate::error::BotError;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Data {
    pub http: reqwest_old::Client,
    pub songbird: Arc<songbird::Songbird>,
    pub guild_cache: Arc<Mutex<GuildCacheMap>>,
}

pub type Error = BotError;
pub type Context<'a> = poise::Context<'a, Data, Error>;

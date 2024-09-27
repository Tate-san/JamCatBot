pub use crate::error::BotError;
use std::sync::Arc;

pub struct Data {
    pub http: reqwest_old::Client,
    pub songbird: Arc<songbird::Songbird>,
}

pub type Error = BotError;
pub type Context<'a> = poise::Context<'a, Data, Error>;

use std::sync::Arc;

// User data, which is stored and accessible in all command invocations
pub struct Data {
    pub http: reqwest_old::Client,
    pub songbird: Arc<songbird::Songbird>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

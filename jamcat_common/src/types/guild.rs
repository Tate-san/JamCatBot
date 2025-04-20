use serenity::all::GuildId;
use tokio::sync::Mutex;
use std::{collections::HashMap, sync::Arc};

use super::queue::Queue;

pub struct GuildCache {
    pub queue: Arc<Mutex<Queue>>,
}

impl Default for GuildCache {
    fn default() -> Self {
        Self {
            queue: Arc::new(Mutex::new(Queue::default())),        
        }
    }
}

impl GuildCache {
    pub fn new(http: reqwest::Client) -> Self {
        Self {
            queue: Arc::new(Mutex::new(Queue::new(http))),
        }
    }
}

pub type GuildCacheMap = HashMap<GuildId, GuildCache>;


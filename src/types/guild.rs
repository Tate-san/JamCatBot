use serenity::all::GuildId;
use std::collections::HashMap;

pub struct GuildCache {
    pub volume: f32,
}

impl Default for GuildCache {
    fn default() -> Self {
        Self { volume: 100.0 }
    }
}

pub type GuildCacheMap = HashMap<GuildId, GuildCache>;

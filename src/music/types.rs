use std::time::Duration;

use serenity::prelude::TypeMapKey;

#[derive(Debug, Clone)]
pub enum QueryType {
    Keywords(String),
    KeywordsList(Vec<String>),
    TrackLink(String),
    PlaylistLink(String),
}

#[derive(Debug, Clone)]
pub struct TrackInfo {
    pub url: String,
    pub artist: String,
    pub title: String,
    pub duration: Option<Duration>,
    pub thumbnail: String,
}

impl TrackInfo {
    pub fn full_name(&self) -> String {
        format!("{} - {}", self.artist, self.title)
    }
}

impl TypeMapKey for TrackInfo {
    type Value = TrackInfo;
}

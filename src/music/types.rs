use std::time::Duration;

use serenity::prelude::TypeMapKey;

#[derive(Debug, Clone)]
pub enum QueryType {
    Keywords(String),
    TrackLink(String),
    PlaylistLink(String),
}

#[derive(Debug, Clone)]
pub struct TrackInfo {
    pub url: String,
    pub title: String,
    pub duration: Option<Duration>,
    pub thumbnail: String,
}

impl TypeMapKey for TrackInfo {
    type Value = TrackInfo;
}

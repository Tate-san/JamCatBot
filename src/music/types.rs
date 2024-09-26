use std::time::Duration;

use serenity::prelude::TypeMapKey;

#[derive(Debug, Clone)]
pub enum QueryType {
    Spotify(String),
    SpotifyPlaylist(String),
    Youtube(String),
    YoutubePlaylist(String),
    Other(String),
    Search(String),
}

impl QueryType {
    pub fn from_url(query: String) -> Self {
        if query.starts_with("http") {
            if query.contains("youtube.com") {
                if query.contains("list=") {
                    return Self::YoutubePlaylist(query);
                } else if query.contains("watch?v=") {
                    return Self::Youtube(query);
                }
            }
            return Self::Other(query);
        }
        Self::Search(query)
    }
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

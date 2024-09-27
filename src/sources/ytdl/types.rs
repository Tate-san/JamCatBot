use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaylistQueryItem {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thumbnail {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryItem {
    #[serde(rename = "uploader")]
    pub author: Option<String>,
    #[serde(rename = "uploader_url")]
    pub author_url: Option<String>,
    pub title: Option<String>,
    #[serde(default)]
    pub url: String,
    pub duration: Option<f32>,
    #[serde(default)]
    pub thumbnail: Option<String>,
    #[serde(default)]
    pub thumbnails: Vec<Thumbnail>,
}

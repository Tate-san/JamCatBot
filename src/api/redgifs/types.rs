use crate::api::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct TemporaryAccessResponse {
    pub token: String,
    pub addr: String,
    pub agent: String,
    pub session: String,
    pub rtfm: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaInfo {
    pub sd: Option<String>,
    pub hd: Option<String>,
    pub gif: Option<String>,
    pub poster: Option<String>,
    pub thumbnail: Option<String>,
    pub vthumbnail: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GifInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub gif_type: u32,
    #[serde(rename = "userName")]
    pub username: String,
    pub published: bool,
    pub verified: bool,
    pub views: u32,
    pub duration: Option<f32>,
    pub urls: MediaInfo,
    pub hls: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GifList {
    pub page: u32,
    pub pages: u32,
    pub total: u32,
    pub gifs: Vec<GifInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub error: ErrorInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorInfo {
    pub code: String,
    pub message: String,
    pub status: u32,
}

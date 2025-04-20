use crate::api::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WaifuImages {
    pub images: Vec<WaifuImageInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WaifuImageInfo {
    pub url: String,
    pub width: u32,
    pub height: u32,
    pub byte_size: u32,
    pub is_nsfw: bool,
}

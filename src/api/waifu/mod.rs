pub mod types;

use types::{WaifuImageInfo, WaifuImages};

use super::prelude::*;

static URL: &str = "https://api.waifu.im";

#[derive(Debug)]
pub struct WaifuApi {
    client: reqwest::Client,
}

impl WaifuApi {
    pub fn new() -> Result<Self> {
        let client = super::ClientBuilder::new_default()?;
        Ok(Self { client })
    }

    pub async fn search(&self, is_nsfw: bool, gif: Option<bool>) -> Result<WaifuImageInfo> {
        let mut params = String::new();

        params += &format!("is_nsfw={}", if is_nsfw { "true" } else { "false" });

        if let Some(gif) = gif {
            params += &format!("gif={}", if gif { "true" } else { "false" });
        }

        let result = self
            .client
            .get(format!("{URL}/search?{params}"))
            .send()
            .await?;

        let text = result.text().await?;
        let list = serde_json::from_str::<WaifuImages>(&text)?;

        if list.images.is_empty() {
            Err(anyhow::anyhow!("No images found"))
        } else {
            Ok(list.images[0].clone())
        }
    }
}

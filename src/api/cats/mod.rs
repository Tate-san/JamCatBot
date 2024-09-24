mod types;

use super::prelude::*;
use types::CatInfo;

static URL: &str = "https://api.thecatapi.com";

#[derive(Debug)]
pub struct CatsApi {
    client: reqwest::Client,
}

impl CatsApi {
    pub fn new() -> Result<Self> {
        let client = super::ClientBuilder::new_default()?;
        Ok(Self { client })
    }

    pub async fn random_cat(&self) -> Result<CatInfo> {
        let result = self
            .client
            .get(format!("{}/v1/images/search", URL))
            .send()
            .await?;

        let text = result.text().await?;

        let list: Vec<CatInfo> = serde_json::from_str(&text)?;

        Ok(list[0].clone())
    }
}

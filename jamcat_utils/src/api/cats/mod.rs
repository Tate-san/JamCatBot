mod model;

use super::prelude::*;
use model::CatInfo;

static URL: &str = "https://api.thecatapi.com";

#[derive(Debug)]
pub struct CatsApi {
    client: reqwest::Client,
}

impl CatsApi {
    pub fn new() -> Result<Self, ApiError> {
        let client = HttpClientBuilder::new_default()?;
        Ok(Self { client })
    }

    pub async fn random_cat(&self) -> Result<CatInfo, ApiError> {
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

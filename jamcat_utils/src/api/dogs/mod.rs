mod model;

use super::prelude::*;
use model::DogInfo;

static URL: &str = "https://api.thedogapi.com";

#[derive(Debug)]
pub struct DogsApi {
    client: reqwest::Client,
}

impl DogsApi {
    pub fn new() -> Result<Self, ApiError> {
        let client = HttpClientBuilder::new_default()?;
        Ok(Self { client })
    }

    pub async fn random_cat(&self) -> Result<DogInfo, ApiError> {
        let result = self
            .client
            .get(format!("{}/v1/images/search", URL))
            .send()
            .await?;

        let text = result.text().await?;

        let list: Vec<DogInfo> = serde_json::from_str(&text)?;

        Ok(list[0].clone())
    }
}

mod types;

use super::prelude::*;
use reqwest::header::HeaderValue;
use types::{GifList, TemporaryAccessResponse};

static URL: &str = "https://api.redgifs.com";

pub enum SearchOrder {
    Best,
    Latest,
    Oldest,
    Trending,
    Top,
    Top7,
    Top24,
}

impl ToString for SearchOrder {
    fn to_string(&self) -> String {
        match self {
            Self::Best => "best",
            Self::Latest => "latest",
            Self::Oldest => "oldest",
            Self::Trending => "trending",
            Self::Top => "top",
            Self::Top7 => "top7",
            Self::Top24 => "top24",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub struct RedgifsApi {
    client: reqwest::Client,
}

impl RedgifsApi {
    pub fn new() -> Result<Self, ApiError> {
        let client = super::ClientBuilder::new_default()?;
        Ok(Self { client })
    }

    async fn get_temporary_access(&mut self) -> Result<TemporaryAccessResponse, ApiError> {
        let result = self
            .client
            .get(format!("{}/v2/auth/temporary", URL))
            .send()
            .await?;

        let text = result.text().await?;

        Ok(serde_json::from_str(&text)?)
    }

    pub async fn login_temporary(&mut self) -> Result<(), ApiError> {
        let access = self.get_temporary_access().await?;
        let bearer = HeaderValue::from_str(&format!("Bearer {}", access.token))
            .expect("Should always be valid");

        self.client = super::ClientBuilder::new()
            .header("Authorization", bearer)
            .build()?;

        Ok(())
    }

    pub async fn search(
        &mut self,
        page: u32,
        count: u32,
        order: SearchOrder,
        search_text: impl ToString,
    ) -> Result<GifList, ApiError> {
        let order_str = order.to_string();
        let search_text = search_text.to_string();

        let search_query = if search_text.is_empty() {
            String::new()
        } else {
            format!("&search_text={search_text}")
        };

        let res = self
            .client
            .get(format!(
                "{URL}/v2/gifs/search?page={page}&count={count}&order={order_str}{search_query}"
            ))
            .send()
            .await?;

        let status = res.status();
        let text = res.text().await?;

        match status {
            reqwest::StatusCode::OK => {
                let res = serde_json::from_str::<types::GifList>(&text)?;
                Ok(res)
            }
            _ => {
                let error_message = serde_json::from_str::<types::Error>(&text)?;
                Err(ApiError::ResponseError(error_message.error.message))
            }
        }
    }
}

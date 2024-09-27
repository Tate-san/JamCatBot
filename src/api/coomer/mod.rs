pub mod types;

use std::io::Read;

use rand::Rng;
use types::{CreatorInfo, PostInfo};

use super::prelude::*;

static URL: &str = "https://coomer.su/api/v1";
static URL_SITE: &str = "https://coomer.su";
static URL_ICONS: &str = "https://img.coomer.su/icons";

#[derive(Debug)]
pub struct CoomerApi {
    client: reqwest::Client,
}

impl CoomerApi {
    pub fn new() -> Result<Self, ApiError> {
        let client = super::ClientBuilder::new_default()?;
        Ok(Self { client })
    }

    pub async fn creators(&self) -> Result<Vec<CreatorInfo>, ApiError> {
        let result = self
            .client
            .get(format!("{}/creators.txt", URL))
            .send()
            .await?;

        let text = result.text().await?;

        Ok(serde_json::from_str(&text)?)
    }

    pub async fn creators_cached(&self) -> Result<Vec<CreatorInfo>, ApiError> {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open("./creators.txt")
            .expect("File creators.txt should exist");

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .expect("File creators.txt should be readable");

        Ok(serde_json::from_str(&buffer)?)
    }

    pub async fn find_creator_by_name(
        &self,
        name: impl ToString,
        creators: &Vec<CreatorInfo>,
    ) -> Option<CreatorInfo> {
        let name = name.to_string().to_lowercase();

        for creator in creators {
            if creator.name.to_lowercase() == name {
                return Some(creator.clone());
            }
        }

        None
    }

    pub async fn random_creator(&self) -> Result<CreatorInfo, ApiError> {
        let creators = self.creators().await?;
        let creators_len = creators.len();

        let random_creator_idx = rand::thread_rng().gen_range(0..creators_len);

        Ok(creators[random_creator_idx].clone())
    }

    pub async fn creator_posts(&self, creator: &CreatorInfo) -> Result<Vec<PostInfo>, ApiError> {
        let service = creator.service.clone();
        let id = creator.id.clone();

        let result = self
            .client
            .get(format!("{URL}/{service}/user/{id}"))
            .send()
            .await?;

        let status = result.status();
        let text = result.text().await?.replace("{}", "null");

        match status {
            reqwest::StatusCode::OK => Ok(serde_json::from_str(&text)?),
            _ => Err(ApiError::ResponseError(text)),
        }
    }

    pub fn get_creator_url(&self, creator: &CreatorInfo) -> String {
        format!("{URL_SITE}/{}/user/{}", &creator.service, &creator.id)
    }

    pub fn get_creator_icon_url(&self, creator: &CreatorInfo) -> String {
        format!("{URL_ICONS}/{}/{}", &creator.service, &creator.id)
    }

    pub async fn get_file_url(&self, post_path: impl ToString) -> Result<String, ApiError> {
        let post_path = post_path.to_string();
        let file_url = format!("{URL_SITE}{post_path}");

        let result = self.client.get(file_url).send().await?;

        match result.status() {
            reqwest::StatusCode::OK => Ok(result.url().to_string()),
            _ => Err(ApiError::Generic(format!(
                "Unable to get the image, got reponse: {}",
                result.status().as_u16()
            ))),
        }
    }
}

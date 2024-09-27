pub use super::prelude::*;

pub struct ClientBuilder {
    user_agent: String,
    headers: reqwest::header::HeaderMap,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            user_agent: "Mozilla/5.0 (X11; Linux x86_64; rv:130.0) Gecko/20100101 Firefox/130.0"
                .to_string(),
            headers: reqwest::header::HeaderMap::new(),
        }
    }
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_default() -> Result<reqwest::Client, ApiError> {
        Self::new().build()
    }

    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = user_agent.to_string();
        self
    }

    pub fn header<K>(mut self, key: K, value: reqwest::header::HeaderValue) -> Self
    where
        K: reqwest::header::IntoHeaderName,
    {
        self.headers.append(key, value);
        self
    }

    pub fn build(self) -> Result<reqwest::Client, ApiError> {
        Ok(reqwest::ClientBuilder::new()
            .user_agent(self.user_agent)
            .default_headers(self.headers)
            .build()?)
    }
}

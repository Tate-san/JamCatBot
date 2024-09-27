use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("{0}")]
    Generic(String),
    #[error("Response error: {0}")]
    ResponseError(String),
    #[error("Request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Json error: {0}")]
    JsonError(#[from] serde_json::Error),
}

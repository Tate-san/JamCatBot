use crate::prelude::*;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("{0}")]
    Generic(String),
    #[error("Response error: {0}")]
    ResponseError(String),
    #[error("{0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
}
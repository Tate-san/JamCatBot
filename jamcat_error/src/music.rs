use crate::prelude::*;

#[derive(Error, Debug)]
pub enum MusicError {
    #[error("{0}")]
    Generic(String),
    #[error("Queue is empty")]
    QueueEmpty,
    #[error("Sorry master, I wasn't to found the requested track")]
    NotFound,
    #[error("Invalid link")]
    InvalidLink,
    #[error("Spotify not authorized")]
    SpotifyAuthError,
    #[error("Missing api key: {0}")]
    ApiKeyMissing(String),
    #[error("{0}")]
    RSpotifyError(#[from] rspotify::ClientError),
    #[error("Out of bounds. Cannot move track from '{from}' to '{to}' in queue of size '{queue_len}' ")]
    QueueMove {
        from: usize,
        to: usize,
        queue_len: usize,
    },
    #[error("[Track]({0}) is unavailable")]
    Unavailable(String),
    #[error(
        "Unable to fetch track. (Check logs)
        FUCK YOU YOUTUBE, YOU CAN SUCK MY DICK. 🖕"
    )]
    TrackFetch,
    #[error("Unable to run '{executable}': {error}")]
    Executable {
        executable: String,
        error: String
    },
    #[error("{0}")]
    ControlError(#[from] songbird::error::ControlError),
}



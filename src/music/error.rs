use thiserror::Error;

#[derive(Error, Debug)]
pub enum MusicError {
    #[error("No such bullshit found")]
    SearchNotFound,
    #[error("Invalid link")]
    InvalidLink,
    #[error(
        "Unable to fetch track.
        Prolly youtube is blocking me again due to too many requests.
        FUCK YOU YOUTUBE, YOU CAN SUCK MY DICK. 🖕"
    )]
    TrackFetch,
}

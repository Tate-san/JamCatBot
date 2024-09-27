use thiserror::Error;

#[derive(Error, Debug)]
pub enum MusicError {
    #[error("No such bullshit found")]
    SearchNotFound,
    #[error("Invalid link")]
    InvalidLink,
    #[error("Unable to fetch track")]
    TrackFetch,
}

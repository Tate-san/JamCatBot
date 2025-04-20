use regex::Regex;
use std::path::PathBuf;
use lazy_static::lazy_static;

lazy_static! {
    // Regex for keywords that indicate the url is a playlist
    pub static ref YT_PLAYLIST_URL_REGEX: Regex = Regex::new(r"list=").unwrap();
    pub static ref RESOURCES_PATH: PathBuf = PathBuf::from("./resources");
    pub static ref SOUNDS_RESOURCES_PATH: PathBuf = RESOURCES_PATH.join("sounds");
}

// Using this suffix we can search for songs without video
pub static MUSIC_ONLY_SUFFIX: &str = r#"\"topic\""#;
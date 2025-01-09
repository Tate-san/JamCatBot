use std::path::PathBuf;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref RESOURCES_PATH: PathBuf = PathBuf::from("./resources");
    pub static ref SOUNDS_RESOURCES_PATH: PathBuf = RESOURCES_PATH.join("sounds");
}

pub static MUSIC_ONLY_SUFFIX: &str = r#"\"topic\""#;

pub static TMP_DIR: &str = "/tmp";
pub static MAX_FILESIZE_MB: f32 = 10.0;

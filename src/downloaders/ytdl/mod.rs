pub mod types;

use anyhow::{Ok, Result};
use std::process::Command;
use types::PlaylistQueryItem;

pub struct Ytdl {
    program: String,
}

impl Ytdl {
    pub fn new() -> Self {
        Self::new_program("yt-dlp")
    }

    pub fn new_program(program: impl ToString) -> Self {
        Self {
            program: program.to_string(),
        }
    }

    pub async fn get_playlist_items(&self, playlist: String) -> Result<Vec<PlaylistQueryItem>> {
        let output = Command::new(&self.program)
            .args(vec![
                "-j",
                "--flat-playlist",
                "-no-abort-on-error",
                &playlist,
            ])
            .output()?;

        let output = String::from_utf8(output.stdout)?;

        let mut links = vec![];

        for line in output.lines() {
            let query: PlaylistQueryItem = serde_json::from_str(line)?;
            links.push(query);
        }

        Ok(links)
    }
}

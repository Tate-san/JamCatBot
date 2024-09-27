mod tests;
pub mod types;

use anyhow::{Ok, Result};
use std::process::Command;
use types::{PlaylistQueryItem, QueryItem};

pub struct Ytdl {
    program: String,
}

impl Ytdl {
    pub fn new() -> Self {
        Self::new_program("yt-dlp")
    }

    pub fn new_program(program: &str) -> Self {
        Self {
            program: program.to_owned(),
        }
    }

    pub async fn query_playlist(&self, url: &str) -> Result<Vec<PlaylistQueryItem>> {
        let output = Command::new(&self.program)
            .args(vec!["-j", "--flat-playlist", "-no-abort-on-error", url])
            .output()?;

        let output = String::from_utf8(output.stdout)?;

        let mut links = vec![];

        for line in output.lines() {
            let query: PlaylistQueryItem = serde_json::from_str(line)?;
            links.push(query);
        }

        Ok(links)
    }

    pub async fn query(&self, url: &str) -> Result<QueryItem> {
        let output = Command::new(&self.program).args(vec!["-j", url]).output()?;
        let output = String::from_utf8(output.stdout)?;

        let mut query: QueryItem = serde_json::from_str(&output)?;
        query.url = url.to_string();

        Ok(query)
    }

    /// Searches for a video by keywords.
    ///
    /// If n_results is None, defaults to 1.
    pub async fn search(
        &self,
        query: &str,
        n_results: Option<usize>,
    ) -> Result<Vec<PlaylistQueryItem>> {
        let n_results = n_results.unwrap_or(1);

        let output = Command::new(&self.program)
            .args(vec![
                "-j",
                "--flat-playlist",
                "--skip-download",
                "--quiet",
                "--ignore-errors",
                &format!("ytsearch{n_results}:'{query}'"),
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

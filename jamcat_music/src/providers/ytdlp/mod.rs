mod tests;
pub mod model;

use jamcat_utils::executable::Executable;
use jamcat_error::{BotError, MusicError};
use model::{PlaylistQueryItem, QueryItem};

pub struct Ytdl {
    program: String,
}

impl Ytdl {
    pub fn new() -> Self {
        Self::new_program("yt-dlp")
    }

    pub fn new_program(program: &str) -> Self {
        Self {
            program: program.to_string(),
        }
    }

    fn base_executable(&self) -> Executable {
        let mut executable = Executable::new(&self.program);
        executable.arg("-4");

        executable
    }

    pub async fn query_playlist(&self, url: &str) -> Result<Vec<PlaylistQueryItem>, BotError> {
        let output = self.base_executable()
            .arg("-j")
            .arg("--flat-playlist")
            .arg("-no-abort-on-error")
            .arg(url)
            .execute()?;

        let stdout = String::from_utf8(output.stdout)?;
        let stderr = String::from_utf8(output.stderr)?;

        if !stderr.is_empty() {
            return Err(MusicError::Executable { executable: self.program.clone(), error: stderr }.into());
        }

        let mut links = vec![];

        for line in stdout.lines() {
            let query: PlaylistQueryItem = serde_json::from_str(line)?;
            links.push(query);
        }

        Ok(links)
    }

    pub async fn query(&self, url: &str) -> Result<QueryItem, BotError> {
        let output = self.base_executable()
            .arg("-j")
            .arg(url)
            .execute()?;

        let stdout = String::from_utf8(output.stdout)?;
        let stderr = String::from_utf8(output.stderr)?;

        if !stderr.is_empty() {
            return Err(MusicError::Executable { executable: self.program.clone(), error: stderr }.into());
        }

        let mut query: QueryItem = serde_json::from_str(&stdout)?;
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
    ) -> Result<Vec<QueryItem>, BotError> {
        let n_results = n_results.unwrap_or(1);

        let output = self.base_executable()
            .arg("-j")
            .arg("--flat-playlist")
            .arg("--skip-download")
            .arg("--quiet")
            .arg("--ignore-errors")
            .arg(format!("ytsearch{n_results}:'{query}'"))
            .execute()?;

        let stdout = String::from_utf8(output.stdout)?;
        let stderr = String::from_utf8(output.stderr)?;

        if !stderr.is_empty() {
            return Err(MusicError::Executable { executable: self.program.clone(), error: stderr }.into());
        }

        let mut links = vec![];

        for line in stdout.lines() {
            let query: QueryItem = serde_json::from_str(line)?;
            links.push(query);
        }

        Ok(links)
    }

    pub async fn search_song(&self, query: &str) -> Result<QueryItem, BotError> {
        let songs = self.search(query, None).await?;

        if songs.is_empty() {
            return Err(MusicError::NotFound.into());
        }

        Ok(songs[0].clone())
    }
}

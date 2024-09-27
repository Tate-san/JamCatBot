mod tests;

use crate::types::*;
use lazy_static::lazy_static;
use regex::Regex;
use rspotify::{
    model::{SimplifiedArtist, TrackId},
    prelude::BaseClient,
    ClientCredsSpotify, Credentials,
};
use std::{env, sync::Arc};
use tokio::sync::Mutex;

use crate::music::types::QueryType;

lazy_static! {
    pub static ref SPOTIFY: Arc<Mutex<Spotify>> = Arc::new(Mutex::new(Spotify::new()));
    pub static ref SPOTIFY_QUERY_REGEX: Regex =
        Regex::new(r"spotify.com/(?P<media_type>.+)/(?P<media_id>.*?)(?:\?|$)").unwrap();
}

pub struct Spotify {
    client: Option<ClientCredsSpotify>,
}

impl Spotify {
    pub fn new() -> Self {
        Self { client: None }
    }

    pub async fn auth(&mut self) -> Result<(), Error> {
        let spotify_client_id = env::var("SPOTIFY_CLIENT_ID")
            .map_err(|_| BotError::ApiKeyMissing("SPOTIFY_CLIENT_ID".to_string()))?;

        let spotify_client_secret = env::var("SPOTIFY_CLIENT_SECRET")
            .map_err(|_| BotError::ApiKeyMissing("SPOTIFY_CLIENT_SECRET".to_string()))?;

        let credentials = Credentials::new(&spotify_client_id, &spotify_client_secret);
        let client = ClientCredsSpotify::new(credentials);
        client.request_token().await?;

        self.client = Some(client);

        Ok(())
    }

    pub async fn get_track(&self, id: &str) -> Result<rspotify::model::FullTrack, Error> {
        let client = match &self.client {
            Some(client) => client,
            None => return Err(BotError::AuthError),
        };

        let track_id = TrackId::from_id(id).map_err(|e| BotError::Generic(e.to_string()))?;
        let track = client.track(track_id, None).await?;

        Ok(track)
    }

    pub async fn get_track_keywords(&self, id: &str) -> Result<String, Error> {
        let track = self.get_track(id).await?;
        let artists = Self::join_artists(&track.artists);

        Ok(Self::build_keyword(&artists, &track.name))
    }

    pub async fn extract(&self, url: impl ToString) -> Result<QueryType, Error> {
        let url = url.to_string();

        let captures = SPOTIFY_QUERY_REGEX
            .captures(&url)
            .ok_or(BotError::Generic("Invalid spotify query".to_string()))?;

        let media_type = captures
            .name("media_type")
            .ok_or(BotError::Generic("Invalid spotify query".to_string()))?
            .as_str();

        let media_id = captures
            .name("media_id")
            .ok_or(BotError::Generic("Invalid spotify query".to_string()))?
            .as_str();

        Ok(match media_type {
            "track" => QueryType::Keywords(self.get_track_keywords(media_id).await?),
            _ => return Err(BotError::Generic("Invalid spotify query".to_string())),
        })
    }

    fn build_keyword(artists: &str, title: &str) -> String {
        format!("{artists} - {title}")
    }

    fn join_artists(artists: &[SimplifiedArtist]) -> String {
        artists
            .iter()
            .map(|a| a.name.clone())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

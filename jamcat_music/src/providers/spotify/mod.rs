use crate::constants::MUSIC_ONLY_SUFFIX;
use jamcat_error::MusicError;
use lazy_static::lazy_static;
use regex::Regex;
use rspotify::{
    model::{
        AlbumId, FullTrack, PlayableItem, PlaylistId, SimplifiedArtist, SimplifiedTrack, TrackId,
    },
    prelude::BaseClient,
    ClientCredsSpotify, Credentials,
};
use serenity::futures::TryStreamExt;
use std::{env, sync::Arc};
use tokio::sync::Mutex;

use jamcat_common::types::music::QueryType;

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

    pub async fn auth(&mut self) -> Result<(), MusicError> {
        let spotify_client_id = env::var("SPOTIFY_CLIENT_ID")
            .map_err(|_| MusicError::ApiKeyMissing("SPOTIFY_CLIENT_ID".to_string()))?;

        let spotify_client_secret = env::var("SPOTIFY_CLIENT_SECRET")
            .map_err(|_| MusicError::ApiKeyMissing("SPOTIFY_CLIENT_SECRET".to_string()))?;

        let credentials = Credentials::new(&spotify_client_id, &spotify_client_secret);
        let client = ClientCredsSpotify::new(credentials);
        client.request_token().await?;

        self.client = Some(client);

        Ok(())
    }

    pub async fn get_track(&self, id: &str) -> Result<FullTrack, MusicError> {
        let client = match &self.client {
            Some(client) => client,
            None => return Err(MusicError::SpotifyAuthError),
        };

        let track_id = TrackId::from_id(id).map_err(|e| MusicError::Generic(e.to_string()))?;
        let track = client.track(track_id, None).await?;

        Ok(track)
    }

    pub async fn get_track_keywords(&self, id: &str) -> Result<String, MusicError> {
        let track = self.get_track(id).await?;

        Ok(Self::build_track_keyword_topic(&track.name, &track.artists))
    }

    pub async fn get_playlist_tracks(&self, id: &str) -> Result<Vec<FullTrack>, MusicError> {
        let client = match &self.client {
            Some(client) => client,
            None => return Err(MusicError::SpotifyAuthError),
        };

        let mut tracks = vec![];

        let playlist_id = PlaylistId::from_id(id).map_err(|e| MusicError::Generic(e.to_string()))?;
        let mut playlist = client.playlist_items(playlist_id, None, None);

        while let Ok(Some(item)) = playlist.try_next().await {
            if let Some(PlayableItem::Track(track)) = item.track {
                tracks.push(track.clone());
            }
        }

        Ok(tracks)
    }

    pub async fn get_playlist_tracks_keywords(&self, id: &str) -> Result<Vec<String>, MusicError> {
        let tracks = self.get_playlist_tracks(id).await?;

        Ok(tracks
            .iter()
            .map(|x| Self::build_track_keyword_topic(&x.name, &x.artists))
            .collect())
    }

    pub async fn get_album_tracks(&self, id: &str) -> Result<Vec<SimplifiedTrack>, MusicError> {
        let client = match &self.client {
            Some(client) => client,
            None => return Err(MusicError::SpotifyAuthError),
        };

        let mut tracks = vec![];

        let album_id = AlbumId::from_id(id).map_err(|e| MusicError::Generic(e.to_string()))?;
        let mut album = client.album_track(album_id, None);

        while let Ok(Some(track)) = album.try_next().await {
            tracks.push(track);
        }

        Ok(tracks)
    }

    pub async fn get_album_tracks_keywords(&self, id: &str) -> Result<Vec<String>, MusicError> {
        let tracks = self.get_album_tracks(id).await?;

        Ok(tracks
            .iter()
            .map(|x| Self::build_track_keyword_topic(&x.name, &x.artists))
            .collect())
    }

    pub async fn extract(&self, url: impl ToString) -> Result<QueryType, MusicError> {
        let url = url.to_string();

        let captures = SPOTIFY_QUERY_REGEX
            .captures(&url)
            .ok_or(MusicError::Generic("Invalid spotify query".to_string()))?;

        let media_type = captures
            .name("media_type")
            .ok_or(MusicError::Generic("Invalid spotify query".to_string()))?
            .as_str();

        let media_id = captures
            .name("media_id")
            .ok_or(MusicError::Generic("Invalid spotify query".to_string()))?
            .as_str();

        Ok(match media_type {
            "track" => QueryType::Keywords(self.get_track_keywords(media_id).await?),
            "playlist" => {
                QueryType::KeywordsList(self.get_playlist_tracks_keywords(media_id).await?)
            }
            "album" => QueryType::KeywordsList(self.get_album_tracks_keywords(media_id).await?),
            _ => return Err(MusicError::Generic("Invalid Spotify URL".to_string())),
        })
    }

    fn build_track_keyword(name: &str, artists: &[SimplifiedArtist]) -> String {
        let artists = Self::join_artists(artists);

        format!("{artists} - {name}")
    }

    fn build_track_keyword_topic(name: &str, artists: &[SimplifiedArtist]) -> String {
        let keyword = Self::build_track_keyword(name, artists);
        format!("{keyword} {MUSIC_ONLY_SUFFIX}")
    }

    fn join_artists(artists: &[SimplifiedArtist]) -> String {
        artists
            .iter()
            .map(|a| a.name.clone())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

use crate::prelude::*;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("{0}")]
    Generic(String),
    #[error("Command can be used only on a server")]
    GuildOnly,
    #[error("You have to be in a voice channel")]
    UserNotInVoice,
    #[error("Bot is not in a voice channel")]
    BotNotInVoice,
    #[error("You are not in the same voice channel as bot")]
    UserNotInVoiceWithBot,
    #[error("Missing api key: {0}")]
    ApiKeyMissing(String),
    #[error("Spotify not authorized")]
    SpotifyAuthError,
    #[error("{0}")]
    JoinError(#[from] songbird::error::JoinError),
    #[error("{0}")]
    SerenityError(#[from] serenity::Error),
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    StringUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("{0}")]
    JsonError(#[from] serde_json::Error),
    #[error("{0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("{0}")]
    ApiError(#[from] crate::ApiError),
    #[error("{0}")]
    MusicError(#[from] crate::MusicError),
}


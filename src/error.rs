use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub enum BotError {
    #[error("Command can be used only on a server")]
    GuildOnly,
    #[error("You have to be in a voice channel")]
    NotInVoice,
}

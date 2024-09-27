use serenity::all::{Colour, CreateEmbed};
use std::fmt::Display;

#[derive(Debug)]
pub struct MessageParams {
    pub as_embed: bool,
    pub ephemeral: bool,
    pub reply: bool,
    pub color: Colour,
    pub message: Message,
    pub embed: Option<CreateEmbed>,
}

impl Default for MessageParams {
    fn default() -> Self {
        MessageParams {
            as_embed: true,
            ephemeral: false,
            reply: false,
            color: Colour::BLUE,
            message: Message::Other(String::new()),
            embed: None,
        }
    }
}

impl MessageParams {
    pub fn new(message: Message) -> Self {
        Self {
            message,
            ..Default::default()
        }
    }

    pub fn with_as_embed(self, as_embed: bool) -> Self {
        Self { as_embed, ..self }
    }

    pub fn with_ephemeral(self, ephemeral: bool) -> Self {
        Self { ephemeral, ..self }
    }

    pub fn with_reply(self, reply: bool) -> Self {
        Self { reply, ..self }
    }

    pub fn with_color(self, color: Colour) -> Self {
        Self { color, ..self }
    }

    pub fn with_msg(self, message: Message) -> Self {
        Self { message, ..self }
    }

    pub fn with_embed(self, embed: Option<CreateEmbed>) -> Self {
        Self { embed, ..self }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Success(String),
    Error(String),
    Embed(CreateEmbed),
    Other(String),
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(message) => f.write_str(&format!("✅ **{message}**")),
            Self::Error(message) => f.write_str(&format!("❗ **{message}**")),
            Self::Other(message) => f.write_str(&message),
            _ => f.write_str("Empty"),
        }
    }
}

impl From<&Message> for Colour {
    fn from(value: &Message) -> Self {
        match value {
            Message::Error(_) => Colour::RED,
            Message::Success(_) => Colour::DARK_GREEN,
            _ => Colour::BLUE,
        }
    }
}

impl From<Message> for Colour {
    fn from(value: Message) -> Self {
        (&value).into()
    }
}

impl From<Message> for MessageParams {
    fn from(value: Message) -> Self {
        MessageParams::new(value.clone()).with_color(value.into())
    }
}

impl From<Message> for CreateEmbed {
    fn from(value: Message) -> Self {
        CreateEmbed::new()
            .colour::<Colour>((&value).into())
            .description(&value.to_string())
    }
}

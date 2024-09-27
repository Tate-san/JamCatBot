use anyhow::Context;
use poise::FrameworkError;

use crate::messages;
use crate::prelude::*;
use crate::types::*;

static INVITE_URL: &str = "https://discord.com/oauth2/authorize?client_id=1185534216558084108";

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            tracing::info!("Logged in as {}", data_about_bot.user.name);
            tracing::info!("Here's your invite link: {INVITE_URL}");
        }
        _ => {}
    }
    Ok(())
}

pub async fn on_error<U, E: std::fmt::Display + std::fmt::Debug>(
    error: FrameworkError<'_, U, E>,
) -> Result<(), serenity::Error> {
    tracing::error!("{}", error);

    match error {
        FrameworkError::UnknownCommand { ctx, msg, .. } => {
            msg.reply(ctx.http.clone(), "Invalid command").await?;
        }
        FrameworkError::Command { ctx, error, .. } => {
            ctx.send(
                poise::CreateReply::default()
                    .embed(messages::Message::Error(format!("{error}")).into()),
            )
            .await?;
        }
        _ => return poise::builtins::on_error(error).await,
    };

    Ok(())
}

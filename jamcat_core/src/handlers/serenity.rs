use std::env;

use crate::prelude::*;
use poise::FrameworkError;

pub async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::CacheReady { guilds } => {
            for guild_id in guilds {
                let has_no_cache = data.guild_cache.lock().await.get(guild_id).is_none();

                if has_no_cache {
                    data.guild_cache
                        .lock()
                        .await
                        .insert(*guild_id, guild::GuildCache::new(data.http.clone()));
                }
            }
        }
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            tracing::info!("Logged in as {}", data_about_bot.user.name);
            if let Ok(link) = env::var("INVITE_LINK") {
                tracing::info!("Here's your invite link: {link}");
            }
            else {
                tracing::warn!("Invite link has not been set");
            }
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
                    .embed(Message::Error(format!("{error}")).into()),
            )
            .await?;
        }
        _ => return poise::builtins::on_error(error).await,
    };

    Ok(())
}
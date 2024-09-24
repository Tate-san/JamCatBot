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

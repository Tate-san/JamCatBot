use super::prelude::*;

#[poise::command(prefix_command, guild_only, category = "Music")]
pub async fn resume(ctx: Context<'_>) -> Result<(), Error> {
    let call = ctx.get_bot_call().await?;
    let handle = call.lock().await;

    if handle.queue().resume().is_ok() {
        ctx.send_message(Message::Success("Resuming track".to_string()))
            .await?;

        Ok(())
    } else {
        Err(BotError::Generic(
            "Can't resume, not playing anything right now".to_string(),
        ))
    }
}

use jamcat_error::MusicError;
use super::prelude::*;

#[poise::command(prefix_command, guild_only, aliases("np"), category = "Music")]
pub async fn now_playing(ctx: Context<'_>) -> Result<(), Error> {
    if let Ok(_) = ctx.get_bot_call().await {
        let queue = ctx.get_queue().await?;
        let track_info = queue.lock().await.current_track_info();

        if let Some(handle) = queue.lock().await.current_track_handle() {
            let track_info = track_info.unwrap();

            let handle_info = handle.get_info()
                .await
                .map_err(|e| BotError::MusicError(MusicError::ControlError(e)))?;

            ctx.send_embed(message::create_now_playing_embed(
                &track_info,
                &handle_info,
            ))
            .await?;

            return Ok(());
        }
    }

    ctx.send_message(Message::Other(
        "I'm not playing anything right now".to_string(),
    ))
    .await?;

    Ok(())
}

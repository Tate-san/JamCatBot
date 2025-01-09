use crate::music::types::TrackInfo;

use super::prelude::*;

#[poise::command(prefix_command, guild_only, aliases("np", "n"), category = "Music")]
pub async fn now_playing(ctx: Context<'_>) -> Result<(), Error> {
    if let Ok(call) = ctx.get_bot_call().await {
        let handle = call.lock().await;

        if let Some(track) = handle.queue().current() {
            let handle_state = track
                .get_info()
                .await
                .map_err(|e| BotError::Generic(e.to_string()))?;

            let track_info = track
                .typemap()
                .read()
                .await
                .get::<TrackInfo>()
                .unwrap()
                .clone();

            ctx.send_embed(messages::factory::create_now_playing_embed(
                &track_info,
                &handle_state,
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

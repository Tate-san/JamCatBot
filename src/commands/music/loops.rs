use songbird::tracks::LoopState;

use super::prelude::*;

#[poise::command(prefix_command, guild_only, category = "Music")]
pub async fn loops(
    ctx: Context<'_>,
    #[description = "How many times to loop the track"] repeat_n: Option<usize>,
) -> Result<(), Error> {
    if let Ok(call) = ctx.get_bot_call().await {
        let handle = call.lock().await;

        if let Some(track) = handle.queue().current() {
            let handle_state = track
                .get_info()
                .await
                .map_err(|e| BotError::Generic(e.to_string()))?;

            if let Some(repeat) = repeat_n {
                let _ = track.loop_for(repeat);

                ctx.send_message(Message::Success(format!("Track looping set to {repeat}")))
                    .await?;
            } else if handle_state.loops == LoopState::Infinite {
                track
                    .disable_loop()
                    .map_err(|e| BotError::Generic(e.to_string()))?;

                ctx.send_message(Message::Success("Disabled infinite loop".to_string()))
                    .await?;
            } else {
                track
                    .enable_loop()
                    .map_err(|e| BotError::Generic(e.to_string()))?;

                ctx.send_message(Message::Success("Enabled infinite loop".to_string()))
                    .await?;
            }

            return Ok(());
        }
    }

    ctx.send_message(Message::Other(
        "I'm not playing anything right now".to_string(),
    ))
    .await?;
    Ok(())
}

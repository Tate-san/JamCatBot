use super::prelude::*;
use crate::music;

#[poise::command(prefix_command, guild_only, aliases("v"), category = "Music")]
pub async fn volume(ctx: Context<'_>, volume: Option<f32>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap().clone();
    let guild_id = guild.id.clone();

    if volume.is_none() {
        if let Some(cache) = ctx.data().guild_cache.lock().await.get(&guild_id) {
            ctx.send_message(Message::Other(format!("Volume is at {}%", cache.volume)))
                .await?;
        }
        return Ok(());
    }

    let manager = &ctx.data().songbird;
    let call = ctx.get_bot_call().await;
    let has_handler = call.is_ok();

    if has_handler {
        let user_voice_state = if let Some(voice_state) = ctx.get_author_voice_state().await {
            voice_state
        } else {
            ctx.send_message(Message::Error(
                "You have to be in a voice channel".to_string(),
            ))
            .await?;
            return Ok(());
        };

        let call = call?;

        let call_lock = call.lock().await;

        let user_channel = user_voice_state.channel_id.unwrap();
        let bot_channel = call_lock.current_channel().unwrap();

        if user_channel.get() != bot_channel.0.get() {
            ctx.send_message(Message::Error(
                "You are not in the same voice channel as bot".to_string(),
            ))
            .await?;
            return Ok(());
        }

        // Has to be dropped otherwise the remove causes deadlock
        drop(call_lock);

        let volume = volume.expect("Should be valid at this point");

        music::set_volume(&ctx, volume.clone()).await?;

        ctx.send_message(Message::Success(format!(
            "Volume has been changed to {volume}%"
        )))
        .await?;
    } else {
        ctx.send_message(Message::Error("Bot is not in a voice channel".to_string()))
            .await?;
        return Ok(());
    }

    Ok(())
}

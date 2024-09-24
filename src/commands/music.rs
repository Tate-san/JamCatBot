use super::prelude::*;

#[poise::command(prefix_command, slash_command, guild_only, category = "Music")]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Video/Song URL"] url: String,
) -> Result<(), Error> {
    if !url.starts_with("http") {
        messages::common::error(&ctx, "Must provide a valid URL").await;
        return Ok(());
    }

    let guild = ctx.guild().unwrap().clone();

    let user_voice_state = if let Some(voice_state) = guild.voice_states.get(&ctx.author().id) {
        voice_state
    } else {
        messages::common::error(&ctx, "You have to be in a voice channel").await;
        return Ok(());
    };

    let manager = ctx.data().songbird.clone();

    let handler = match manager.get(guild.id) {
        Some(handler) => handler,
        None => {
            manager
                .join(guild.id, user_voice_state.channel_id.unwrap())
                .await?
        }
    };

    let mut handler_lock = handler.lock().await;

    let mut src = songbird::input::YoutubeDl::new(ctx.data().http.clone(), url);

    handler_lock.add_global_event(
        songbird::TrackEvent::Error.into(),
        crate::handlers::songbird::TrackErrorNotifier,
    );

    let search_result = match src.search(None).await {
        Ok(res) => res,
        Err(_) => {
            messages::common::error(&ctx, "Invalid link").await;
            return Ok(());
        }
    };

    let _ = handler_lock.enqueue(src.into()).await;

    tracing::info!("{:#?}", search_result);

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title(search_result[0].clone().title.unwrap_or_default())
                .thumbnail(search_result[0].clone().thumbnail.unwrap_or_default()),
        ),
    )
    .await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command, guild_only, category = "Music")]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap().clone();
    let guild_id = guild.id.clone();

    let manager = &ctx.data().songbird;
    let has_handler = manager.get(guild_id.clone()).is_some();

    if has_handler {
        let user_voice_state = if let Some(voice_state) = guild.voice_states.get(&ctx.author().id) {
            voice_state
        } else {
            messages::common::error(&ctx, "You have to be in a voice channel").await;
            return Ok(());
        };

        let call = manager.get(guild_id).unwrap();

        let call_lock = call.lock().await;

        let user_channel = user_voice_state.channel_id.unwrap();
        let bot_channel = call_lock.current_channel().unwrap();

        if user_channel.get() != bot_channel.0.get() {
            messages::common::error(&ctx, "You are not in the same voice channel as bot").await;
            return Ok(());
        }

        // Has to be dropped otherwise the remove causes deadlock
        drop(call_lock);

        if let Err(e) = manager.remove(guild_id).await {
            messages::common::error(&ctx, format!("{:?}", e)).await;
            return Ok(());
        }

        messages::common::success(&ctx, "Bye bye senpai 👉👈 🥹").await;
    } else {
        messages::common::error(&ctx, "Bot is not in a voice channel").await;
        return Ok(());
    }

    Ok(())
}

#[poise::command(prefix_command, slash_command, guild_only, category = "Music")]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap().clone();
    let guild_id = guild.id.clone();

    let manager = &ctx.data().songbird;
    let has_handler = manager.get(guild_id.clone()).is_some();

    if has_handler {
        let user_voice_state = if let Some(voice_state) = guild.voice_states.get(&ctx.author().id) {
            voice_state
        } else {
            messages::common::error(&ctx, "You have to be in a voice channel").await;
            return Ok(());
        };

        let call = manager.get(guild_id).unwrap();

        let call_lock = call.lock().await;

        let user_channel = user_voice_state.channel_id.unwrap();
        let bot_channel = call_lock.current_channel().unwrap();

        if user_channel.get() != bot_channel.0.get() {
            messages::common::error(&ctx, "You are not in the same voice channel as bot").await;
            return Ok(());
        }

        if let Err(error) = call_lock.queue().skip() {
            let message = format!("Unable to skip the track: {error}");

            tracing::error!(message);
            messages::common::error(&ctx, message).await;
        }
    } else {
        messages::common::error(&ctx, "Bot is not in a voice channel").await;
        return Ok(());
    }

    Ok(())
}

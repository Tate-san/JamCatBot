use std::sync::Arc;

use guild::GuildCache;
use songbird::Call;
use songbird::TrackEvent;
use tokio::sync::Mutex;

use crate::error::BotError;
use crate::handlers;
use crate::prelude::*;
use crate::types::*;

pub async fn get_call_or_join(ctx: &Context<'_>) -> Result<Arc<Mutex<Call>>, Error> {
    let guild = ctx.guild().unwrap().clone();
    let guild_id = guild.id.clone();

    let user_voice_state = if let Some(voice_state) = ctx.get_author_voice_state().await {
        voice_state
    } else {
        return Err(BotError::NotInVoice);
    };

    let manager = ctx.data().songbird.clone();

    let handler = match ctx.get_bot_call().await {
        Ok(handler) => handler,
        Err(_) => {
            let handler = manager
                .join(guild.id, user_voice_state.channel_id.unwrap())
                .await?;

            register_call_handlers(ctx, handler.clone()).await;

            handler
        }
    };

    Ok(handler)
}

pub async fn register_call_handlers(ctx: &Context<'_>, call: Arc<Mutex<Call>>) {
    let mut handler = call.lock().await;

    handler.remove_all_global_events();

    handler.add_global_event(
        TrackEvent::Error.into(),
        handlers::songbird::TrackErrorNotifier,
    );

    handler.add_global_event(
        TrackEvent::Play.into(),
        handlers::songbird::TrackPlayNotifier {
            channel_id: ctx.channel_id().clone(),
            http: ctx.serenity_context().http.clone(),
        },
    );
}

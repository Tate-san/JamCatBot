use std::sync::Arc;

use songbird::Call;
use songbird::CoreEvent;
use songbird::TrackEvent;
use tokio::sync::Mutex;

use jamcat_error::BotError;
use jamcat_common::types::Context;
use crate::constants::SOUNDS_RESOURCES_PATH;
use crate::handlers;
use jamcat_core::prelude::*;

pub async fn get_call_or_join(
    ctx: &Context<'_>,
    sound_on_join: bool,
) -> Result<Arc<Mutex<Call>>, Error> {
    let guild = ctx.guild().unwrap().clone();

    let user_voice_state = if let Some(voice_state) = ctx.get_author_voice_state().await {
        voice_state
    } else {
        return Err(BotError::UserNotInVoice);
    };

    let manager = ctx.data().songbird.clone();

    let handler = match ctx.get_bot_call().await {
        Ok(handler) => handler,
        Err(_) => {
            let handler = manager
                .join(guild.id, user_voice_state.channel_id.unwrap())
                .await?;

            register_call_handlers(ctx, handler.clone()).await;

            let join_resource_path = SOUNDS_RESOURCES_PATH.join("join.mp3");

            if !join_resource_path.exists() {
                tracing::warn!("Join sound doesn't exist, skipping");
            }

            if sound_on_join && join_resource_path.exists() {
                // TODO change
                let track_input = std::fs::read(join_resource_path)?;

                handler
                    .lock()
                    .await
                    .enqueue_input(track_input.into())
                    .await
                    .set_volume(0.05)
                    .unwrap();
            }

            handler
        }
    };

    Ok(handler)
}

pub async fn leave_call(ctx: &Context<'_>, sound_on_leave: bool) -> Result<(), Error> {
    match ctx.get_bot_call().await {
        Ok(handler) => {
            let guild = ctx.guild().unwrap().clone();
            let guild_id = guild.id;

            let manager = &ctx.data().songbird;

            let user_voice_state = if let Some(voice_state) = ctx.get_author_voice_state().await {
                voice_state
            } else {
                return Err(BotError::UserNotInVoice);
            };

            let mut handler_lock = handler.lock().await;

            let user_channel = user_voice_state.channel_id.unwrap();
            let bot_channel = handler_lock.current_channel().unwrap();

            if user_channel.get() != bot_channel.0.get() {
                return Err(BotError::UserNotInVoiceWithBot);
            }

            let leave_resource_path = SOUNDS_RESOURCES_PATH.join("leave.mp3");

            if !leave_resource_path.exists() {
                tracing::warn!("Leave sound doesn't exist, skipping");
            }

            if sound_on_leave && leave_resource_path.exists() {
                let track_input = std::fs::read(leave_resource_path)?;

                let track_handle = handler_lock.play_only_input(track_input.into());
                track_handle.set_volume(0.05).unwrap();

                while !track_handle.get_info().await.unwrap().playing.is_done() {}
            }

            drop(handler_lock);

            if let Err(e) = manager.remove(guild_id).await {
                ctx.send_message(Message::Error(e.to_string())).await?;
                return Ok(());
            }

            ctx.send_message(Message::Other("**Bye bye senpai** 👉👈 🥹".to_string()))
                .await?;

            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub async fn register_call_handlers(ctx: &Context<'_>, call: Arc<Mutex<Call>>) {
    let queue = ctx.get_queue().await.expect("Queue is already registered at this point");
    let call_handle = call.clone();
    let mut handler = call.lock().await;

    handler.remove_all_global_events();

    handler.add_global_event(
        CoreEvent::DriverDisconnect.into(), 
        handlers::songbird::DisconnectNotifier {
            call: call_handle.clone(),
            queue: queue.clone()
        },
    );

    handler.add_global_event(
        TrackEvent::Error.into(),
        handlers::songbird::TrackErrorNotifier,
    );

    handler.add_global_event(
        TrackEvent::Play.into(),
        handlers::songbird::TrackPlayNotifier {
            channel_id: ctx.channel_id(),
            http: ctx.serenity_context().http.clone(),
            queue: queue.clone()
        },
    );

    handler.add_global_event(
        TrackEvent::End.into(),
        handlers::songbird::TrackEndNotifier {
            call: call_handle.clone(),
            queue: queue.clone()
        },
    );

    /* 
    handler.add_global_event(
        TrackEvent::Play.into(),
        handlers::songbird::TrackPlayNotifier {
            channel_id: ctx.channel_id(),
            http: ctx.serenity_context().http.clone(),
        },
    );

    handler.add_global_event(
        CoreEvent::DriverDisconnect.into(), 
        handlers::songbird::DisconnectNotifier {
            call: disconnect_handler,
        },
    );
    */
}


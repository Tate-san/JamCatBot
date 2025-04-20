use jamcat_common::types::queue::Queue;
use jamcat_core::message;
use std::sync::Arc;
use tokio::sync::Mutex;
use serenity::all::{ChannelId, CreateMessage, Http};
use songbird::{events::{Event, EventContext, EventHandler as VoiceEventHandler}, Call};

pub struct DisconnectNotifier {
    pub call: Arc<Mutex<Call>>,
    pub queue: Arc<Mutex<Queue>>,
}

#[serenity::async_trait]
impl VoiceEventHandler for DisconnectNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::DriverDisconnect(d) = ctx {
            tracing::warn!("Bot disconnected from guild({:?}), voice({:?}), reason: {:?}", d.guild_id.0, d.channel_id, d.reason);

            let mut queue = self.queue.lock().await;
            queue.stop(self.call.clone()).await;
            let _ = self.call.lock().await.leave().await;
        }

        None
    }
}

pub struct TrackErrorNotifier;

#[serenity::async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                tracing::error!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }
        None
    }
}

pub struct TrackPlayNotifier {
    pub channel_id: ChannelId,
    pub http: Arc<Http>,
    pub queue: Arc<Mutex<Queue>>
}

#[serenity::async_trait]
impl VoiceEventHandler for TrackPlayNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track([(state, handle)]) = ctx {
            let queue = self.queue.lock().await;
            tracing::info!("Starting song: {}", handle.uuid());

            if let Some(info) = queue.current_track_info() {
                let _ = self
                    .channel_id
                    .send_message(
                        self.http.clone(),
                        CreateMessage::new()
                            .add_embed(message::create_now_playing_embed(&info, state)),
                    )
                    .await;
            }

        }
        None
    }
}

pub struct TrackEndNotifier {
    pub call: Arc<Mutex<Call>>,
    pub queue: Arc<Mutex<Queue>>
}

#[serenity::async_trait]
impl VoiceEventHandler for TrackEndNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track([(_, _)]) = ctx {
            let mut queue = self.queue.lock().await;
            queue.stop(self.call.clone()).await;

            if queue.current_track_handle().is_none() {
                _ = queue.play_next(self.call.clone()).await;
            }
        }
        None
    }
}
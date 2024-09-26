use std::{sync::Arc, time::Duration};

use serenity::all::{ChannelId, CreateMessage, Http};
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler};

use crate::{messages, music::types::TrackInfo};

pub struct TrackErrorNotifier;

#[serenity::async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        tracing::info!("{:#?}", ctx);
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
}

#[serenity::async_trait]
impl VoiceEventHandler for TrackPlayNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            let (state, handle) = track_list.get(0).expect("Track is expected to be present");
            tracing::info!("Starting song: {}", handle.uuid());

            let typemap = handle.typemap().read().await;
            if let Some(info) = typemap.get::<TrackInfo>() {
                tracing::info!("{:#?}", info);

                let _ = self
                    .channel_id
                    .send_message(
                        self.http.clone(),
                        CreateMessage::new()
                            .add_embed(messages::factory::create_now_playing_embed(info.clone())),
                    )
                    .await;
            }
        }
        None
    }
}

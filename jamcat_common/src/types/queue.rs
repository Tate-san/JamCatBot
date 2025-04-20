use std::{collections::VecDeque, sync::Arc};

use jamcat_error::MusicError;
use songbird::{input::YoutubeDl, tracks::TrackHandle, Call};
use tokio::sync::Mutex;

use super::music::TrackInfo;

static VOLUME_MULTIPLIER: f32 = 250.0;

#[derive(Debug, Clone, Default)]
pub struct Queue {
    current_handle: Option<TrackHandle>,
    current_track: Option<TrackInfo>,
    queue: VecDeque<TrackInfo>,
    volume: f32,
    http: reqwest::Client
}

impl Queue {
    pub fn new(http: reqwest::Client) -> Self {
        Self {
            http,
            volume: 50.0 / VOLUME_MULTIPLIER,
            ..Default::default()
        }
    }

    pub fn volume(&self) -> f32 {
        self.volume.clone() * VOLUME_MULTIPLIER
    }

    pub fn set_volume(&mut self, volume: f32) -> Result<(), MusicError> {
        self.volume = volume / VOLUME_MULTIPLIER;

        if let Some(handle) = &self.current_handle {
            handle.set_volume(self.volume)?;
        }

        Ok(())
    }

    pub fn current_track_handle(&self) -> Option<TrackHandle> {
        self.current_handle.clone()
    }

    pub fn current_track_info(&self) -> Option<TrackInfo> {
        self.current_track.clone()
    }

    pub fn enqueue(&mut self, track: TrackInfo) {
        self.queue.push_back(track);
    }

    pub fn remove(&mut self, index: usize) -> Option<TrackInfo> {
        self.queue.remove(index)
    }

    pub fn remove_first(&mut self) -> Option<TrackInfo> {
        self.queue.pop_front()
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn queue(&mut self) -> &mut VecDeque<TrackInfo> {
        &mut self.queue
    }

    pub fn move_track(&mut self, from: usize, to: usize) -> Result<(), MusicError> {

        let queue_len = self.queue.len();

        if from >= queue_len || to >= queue_len {
            return Err(MusicError::QueueMove { from, to, queue_len });
        } 

        if let Some(removed) = self.queue.remove(from) {
            self.queue.insert(to, removed); 
        }

        Ok(())
    }

    pub async fn play_next(&mut self, call: Arc<Mutex<Call>>) -> Result<TrackHandle, MusicError> {
        self.stop(call.clone()).await;

        if let Some(next) = self.remove_first() {
            let res = self.play(call, &next.url).await;

            if res.is_err() {
                self.current_track = None;
                self.current_handle = None;
            }
            else {
                self.current_track = Some(next);
            }

            res
        }
        else {
            Err(MusicError::QueueEmpty)
        }
    }

    async fn play(&mut self, call: Arc<Mutex<Call>>, url: impl ToString) -> Result<TrackHandle, MusicError> {
        let mut call_handle = call.lock().await;
        let source = YoutubeDl::new(self.http.clone(), url.to_string());
        let handle = call_handle.play_input(source.into());
        handle.pause()?;
        handle.play()?;
        handle.set_volume(self.volume)?;
        self.current_handle = Some(handle.clone());

        Ok(handle)
    }
    pub async fn stop(&mut self, call: Arc<Mutex<Call>>) {
        let mut call_handle = call.lock().await;
        call_handle.stop();

        self.current_handle = None;
        self.current_track = None;
    }
}
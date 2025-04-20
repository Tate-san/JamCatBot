pub mod brainrot;
pub mod http;
pub mod api;
pub mod executable;

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub fn duration_string(duration: Duration) -> String {
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let hours = (duration.as_secs() / 60) / 60;

    if hours > 0 {
        format!("{hours:0>2}:{minutes:0>2}:{seconds:0>2}")
    } else {
        format!("{minutes:0>2}:{seconds:0>2}")
    }
}

pub fn get_total_pages(items_count: usize, items_per_page: usize) -> usize {
    (items_count as f32 / items_per_page as f32).ceil() as usize
}


/// Generated unique filename
pub fn gen_unique_filename() -> String {
    let uuid = Uuid::new_v4();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    format!("{uuid}_{timestamp}")
}


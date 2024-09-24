pub mod cats;
pub mod client;
pub mod coomer;
pub mod dogs;
pub mod redgifs;
pub mod waifu;

pub use client::ClientBuilder;
pub use {cats::CatsApi, coomer::CoomerApi, dogs::DogsApi, redgifs::RedgifsApi, waifu::WaifuApi};

pub mod prelude {
    pub use anyhow::{Ok, Result};
    pub use serde::{Deserialize, Serialize};
}

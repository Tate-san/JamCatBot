pub mod cats;
pub mod client;
pub mod coomer;
pub mod dogs;
pub mod error;
pub mod redgifs;
pub mod waifu;

pub use client::ClientBuilder;
pub use error::ApiError;
pub use {cats::CatsApi, coomer::CoomerApi, dogs::DogsApi, redgifs::RedgifsApi, waifu::WaifuApi};

pub mod prelude {
    pub use super::ApiError;
    pub use serde::{Deserialize, Serialize};
}

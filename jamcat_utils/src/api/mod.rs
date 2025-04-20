pub mod cats;
pub mod dogs;
pub mod coomer;
pub mod redgifs;
pub mod waifu;

pub mod prelude {
    pub use serde::{Deserialize, Serialize};
    pub use crate::http::HttpClientBuilder;
    pub use jamcat_error::ApiError;
}


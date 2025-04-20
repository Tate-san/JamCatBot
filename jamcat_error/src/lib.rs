mod error;
mod music;
mod api;

pub use error::*;
pub use music::*;
pub use api::*;

mod prelude {
    pub use thiserror::Error;
}

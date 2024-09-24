pub mod common;

pub fn check_msg<T>(result: serenity::Result<T>) {
    if let Err(error) = result {
        tracing::error!("{error}");
    }
}

pub mod prelude {
    pub use super::check_msg;
    pub use crate::prelude::*;
    pub use crate::types::*;
    pub use anyhow::{Ok, Result};
}

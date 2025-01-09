pub mod anime;

pub mod prelude {
    pub use serde::{Deserialize, Serialize};
    pub use sqlx::FromRow;
}

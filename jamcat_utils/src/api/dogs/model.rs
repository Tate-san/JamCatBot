use crate::api::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DogInfo {
    pub id: String,
    pub url: String,
    pub width: u32,
    pub height: u32,
}

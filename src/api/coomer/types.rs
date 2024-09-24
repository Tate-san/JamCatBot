use crate::api::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreatorInfo {
    pub id: String,
    pub name: String,
    pub service: String,
    pub indexed: u64,
    pub updated: u64,
    pub favorited: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostInfo {
    pub id: String,
    pub user: String,
    pub service: String,
    pub title: String,
    pub content: String,
    pub shared_file: bool,
    pub added: String,
    pub published: String,
    pub file: Option<FileInfo>,
    pub attachments: Vec<FileInfo>,
}

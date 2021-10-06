use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewContent {
    pub name: String,
    pub mime_type: Option<String>,
    pub tags: Vec<String>,
    pub content: Value,
    pub raw: bool,
    pub cache_control: Option<String>,
    pub taxonomy_id: Option<Uuid>
}





#[derive(Serialize, Deserialize, Debug)]
pub struct RawContent {
    pub content_item_raw_id: Uuid,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateContent {
    pub mime_type: Option<String>,
    pub tags: Option<Vec<String>>,
    pub content: Option<String>,
    pub cache_control: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub content_id: Uuid,
    pub name: String,
    pub mime_type: Option<String>,
    pub site_id: Uuid,
    pub collection_id: Uuid,
    pub tags: Vec<String>,
    pub content_length: i32,
    pub version: i32,
    pub created_by: String,
    pub modified: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentResponse {
    pub content_str: String,
    pub cache_control: String,
    pub mime_type: String,
}

pub struct ContentSet {
    pub keys: Vec<String>,
    pub values: Vec<String>,
}

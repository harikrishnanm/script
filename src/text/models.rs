use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewText {
    pub name: String,
    pub mime_type: Option<String>,
    pub tags: Vec<String>,
    pub content: String,
    pub cache_control: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    pub text_id: Uuid,
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

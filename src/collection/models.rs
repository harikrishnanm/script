use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewCollection {
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub cache_control: Option<String>,
    pub public: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    pub collection_id: Uuid,
    pub site_id: Uuid,
    pub site_name: String,
    pub name: String,
    pub cache_control: String,
    pub parent_id: Option<Uuid>,
    pub created_by: String,
}

#[derive(Serialize, Debug)]
pub struct CollectionResponse {
    pub name: String,
    pub content: Vec<TextContent>,
}

#[derive(Serialize, Debug)]
pub struct TextContent {
    pub name: String,
    pub tags: Vec<String>,
    pub content: String,
    pub mime_type: String,
    pub url: String,
}

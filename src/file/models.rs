use serde::Serialize;
use uuid::Uuid;

#[derive(Debug)]
pub struct FileDetails {
    pub file_id: Uuid,
    pub name: String,
    pub original_name: String,
    pub cache_control: String,
    pub tags: Vec<String>,
    pub size: i32,
    pub path: String,
    pub mime_type: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct File {
    pub file_id: Uuid,
    pub name: String,
    pub original_name: String,
    pub cache_control: String,
    pub tags: Vec<String>,
    pub size: i32,
    pub path: String,
    pub mime_type: String,
    pub site_name: String,
    pub created_by: String,
}

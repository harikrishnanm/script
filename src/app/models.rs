use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct NewApp {
    pub name: String,
    pub path: String,
    pub slug: Option<String>,
    pub url: Option<String>,
    pub cors_enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct App {
    pub app_id: Uuid,
    pub name: String,
    pub path: String,
    pub slug: Option<String>,
    pub url: Option<String>,
    pub cors_enabled: Option<bool>,
    pub created_by: String,
    pub created: NaiveDateTime,
    pub modified: Option<NaiveDateTime>,
}

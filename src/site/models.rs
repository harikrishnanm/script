use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct NewSite {
  pub name: String,
  pub path: String,
  pub slug: Option<String>,
  pub url: Option<String>,
  pub cors_enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
  site_id: Uuid,
  name: String,
  path: String,
  slug: Option<String>,
  url: Option<String>,
  cors_enabled: Option<bool>,
  created_by: String,
  created: NaiveDateTime,
  modified: NaiveDateTime,
}

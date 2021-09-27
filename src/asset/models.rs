use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct NewAsset {
  name: String,
  file_id: Uuid,
  content_disposition: String,
}

#[derive(Serialize, Debug)]
pub struct Asset {
  name: String,
  file_id: Uuid,
  file_name: String,
  site_id: Uuid,
  site_name: String,
  coll_id: Uuid,
  coll_name: String,
}

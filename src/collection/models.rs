use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewCollection {
  pub name: String,
  pub parent_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
  pub collection_id: Uuid,
  pub site_id: Uuid,
  pub site_name: String,
  pub name: String,
  pub parent_id: Option<Uuid>,
  pub created_by: String,
}

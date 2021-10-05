use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Taxonomy {
  pub taxonomy_id: Uuid,
  pub name: String,
  pub site_id: Uuid,
  pub site_name: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct NewTaxonomy {
  pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaxonomyItem {
  pub taxonomy_item_id: Uuid,
  pub taxonomy_id: Uuid,
  pub item_name: String,
  pub item_type: String,
  pub ordinal: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewTaxonomyItem {
  pub item_name: String,
  pub item_type: String,
  pub ordinal: i32,
}

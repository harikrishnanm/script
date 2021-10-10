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

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
pub enum ItemType {
    A,
    O,
    N,
    S,
    B,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaxonomyItem {
    pub taxonomy_item_id: Uuid,
    pub taxonomy_id: Uuid,
    pub item_name: String,
    pub item_type: String,
    pub item_taxonomy_id: Option<Uuid>,
    pub ordinal: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaxonomyItemResponse {
    pub taxonomy_item_id: Uuid,
    pub taxonomy_id: Uuid,
    pub item_name: String,
    pub item_type: String,
    pub item_taxonomy_id: Option<Uuid>,
    pub ordinal: i32,
    pub taxonomy_item: Option<Box<Vec<TaxonomyItemResponse>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewTaxonomyItem {
    pub item_name: String,
    pub item_type: String,
    pub item_taxonomy_id: Option<Uuid>,
    pub ordinal: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaxonomyListItem {
    pub taxonomy_id: Uuid,
    pub name: String,
}

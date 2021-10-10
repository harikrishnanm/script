use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct NewAsset {
    pub name: String,
    pub file_id: Uuid,
    //pub file_name: String,
    pub content_disposition: String,
}

#[derive(Serialize, Debug)]
pub struct Asset {
    pub name: String,
    pub file_id: Uuid,
    pub file_name: String,
    pub site_id: Uuid,
    pub site_name: String,
    pub coll_id: Uuid,
    pub coll_name: String,
}

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct NewFolder {
  pub name: String,
  pub size_limit: Option<i32>,
}

#[derive(Serialize, Debug)]
pub struct Folder {
  pub name: String,
  pub full_path: String,
  pub size_limit: Option<i32>,
  pub num_files: i32,
  pub free_space: Option<i32>,
}

use crate::folder::models::*;
use log::*;
use std::env;
use std::fs::DirBuilder;
use std::io::Error;
use std::path::Path;

impl NewFolder {
  pub async fn create_folder(self: &Self, path: &str) -> Result<(), Error> {
    debug!("Full path {}", path);

    let root_path = match env::var("FILE_STORE_ROOT") {
      Ok(root) => root,
      Err(e) => {
        error!("Cannot read FILE_STORE_ROOT env variable. Will use default ./file_store");
        "/file_store".to_string()
      }
    };

    let folder_path_str = format!("{}/{}", root_path, path);

    let parent_path = Path::new(&folder_path_str);

    debug!("Creating path");
    match DirBuilder::new().recursive(true).create(parent_path) {
      Ok(_) => {
        debug!("Created folder");
        Ok(())
      }
      Err(e) => {
        error!("Error creating folder {}", e);
        Err(e)
      }
    }
  }
}

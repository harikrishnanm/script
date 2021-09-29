use crate::folder::models::*;
use log::*;
use sqlx::Error;
use std::env;
use std::fs::{DirBuilder, DirEntry, File};
use std::path::Path;

impl NewFolder {
  pub async fn create_folder(
    self: &Self,
    site_name: &str,
    parent: &str,
    folder_name: &str,
  ) -> Result<(), Error> {
    debug!("Check if parent folder exists");

    let root_path = match env::var("FILE_STORE_ROOT") {
      Ok(root) => root,
      Err(e) => {
        error!("Cannot read FILE_STORE_ROOT env variable. Will use default ./tmp");
        "/tmp".to_string()
      }
    };
    debug!("Parent path {}", parent);
    let mut parent_path_str = String::new();
    if !parent.eq_ignore_ascii_case("root") {
      parent_path_str = format!("{}/{}/{}/{}", root_path, site_name, parent, folder_name);
    } else {
      parent_path_str = format!("{}/{}/{}", root_path, site_name, folder_name);
    }
    
    debug!("Full path {}", parent_path_str);
    let parent_path = Path::new(&parent_path_str);

    if parent_path.exists() {
      debug!("Path exists");
    } else {
      debug!("Creating path");
      match DirBuilder::new().recursive(true).create(parent_path) {
        Ok(_) => debug!("Created folder"),
        Err(e) => error!("Error creating folder {}", e),
      }
    }

    Ok(())
  }
}

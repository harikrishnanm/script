pub mod folder;
pub mod models;

use crate::common::utils;
use crate::error::ScriptError;
use crate::folder::models::*;
use crate::rbac::models::Identity;
use crate::AppData;
use actix_web::{
  get, patch, post,
  web::{Data, Path, ReqData},
  HttpResponse,
};
use actix_web_validator::Json;
use log::*;
use std::fs;

/* Create folder code to be added under put/patch
debug!("Create folder {}", &folder_name);
    DirBuilder::new()
        .recursive(true)
        .create(&folder_name)
        .unwrap();
        */

#[get("/site/{site_name}/folder/{folder:.*}")]
pub async fn get(
  identity: ReqData<Identity>,
  data: Data<AppData>,
  Path((site_name, folder)): Path<(String, String)>,
) -> Result<HttpResponse, ScriptError> {
  let mut subfolders: Vec<FolderEntry> = Vec::new();
  let mut files: Vec<FileEntry> = Vec::new();

  let root_path = utils::get_root_path();

  let base_path = format!("{}/{}", root_path, site_name);
  let base_path_len = base_path.len();
  let full_path = format!("{}/{}", base_path, folder);
  debug!("Getting details for {}", full_path);
  let path = std::path::Path::new(&full_path);

  if let Ok(entries) = fs::read_dir(full_path) {
    for entry in entries {
      if let Ok(entry) = entry {
        if let Ok(metadata) = entry.metadata() {
          debug!("{:?}: {:?}", entry.path(), metadata);
          if metadata.is_file() {
            let file_name = format!("{:?}", entry.file_name());
            files.push(FileEntry {
              name: entry.file_name().into_string().unwrap(),
              size: metadata.len(),
            });
          } else if metadata.is_dir() {
            let mut path = entry.path().to_str().unwrap().to_string();
            path.replace_range(..base_path_len, "");
            subfolders.push(FolderEntry { name: path })
          }
        } else {
          error!("Couldnt get file metadata")
        }
      }
    }
  }
  let response = FolderListing {
    files: files,
    folders: subfolders,
  };
  Ok(HttpResponse::Ok().json(response))
}

#[patch("/site/{site_name}/folder/{parent:.*}")]
pub async fn create(
  identity: ReqData<Identity>,
  data: Data<AppData>,
  new_folder: Json<NewFolder>,
  Path((site_name, parent)): Path<(String, String)>,
) -> Result<HttpResponse, ScriptError> {
  debug!("Creating folder {}", parent);

  let folder_name = &new_folder.name;
  match new_folder
    .create_folder(&site_name, &parent, folder_name)
    .await
  {
    Ok(_) => debug!("done"),
    Err(e) => error!("{}", e),
  }

  Ok(HttpResponse::Ok().finish())
}

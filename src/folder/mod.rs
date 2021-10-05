pub mod folder;
pub mod models;

use crate::common::utils;
use crate::error::ScriptError;
use crate::folder::models::*;
use crate::rbac::models::Identity;
use crate::AppData;
use actix_web::{
  get, patch,
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

#[get("/site/{site_name}/folders/{folder:.*}")]
pub async fn get(
  _identity: ReqData<Identity>,
  _data: Data<AppData>,
  Path((site_name, folder)): Path<(String, String)>,
) -> Result<HttpResponse, ScriptError> {
  debug!("Getting folder listing for {}", folder);
  let root_path = utils::get_root_path();

  let base_path = format!("{}/{}", root_path, site_name);
  let _base_path_len = base_path.len();

  let listing = get_folder_entries(&base_path, Some(&folder));
  Ok(HttpResponse::Ok().json(listing))
}

#[get("/site/{site_name}/folders")]
pub async fn get_root(
  _identity: ReqData<Identity>,
  _data: Data<AppData>,
  Path(site_name): Path<String>,
) -> Result<HttpResponse, ScriptError> {
  debug!("Getting folder listing for root");
  let root_path = utils::get_root_path();

  let base_path = format!("{}/{}", root_path, site_name);
  let _base_path_len = base_path.len();

  let listing = get_folder_entries(&base_path, None);
  Ok(HttpResponse::Ok().json(listing))
}

fn get_folder_entries(base_path: &str, folder: Option<&str>) -> FolderListing {
  debug!("Getting folder listing of {}", base_path);
  let full_path = match folder {
    Some(f) => format!("{}/{}", base_path, f),
    None => base_path.to_string(),
  };
  let base_path_len = base_path.len();
  let mut subfolders: Vec<FolderEntry> = Vec::new();
  let mut files: Vec<FileEntry> = Vec::new();

  if let Ok(entries) = fs::read_dir(full_path) {
    for entry in entries {
      if let Ok(entry) = entry {
        if let Ok(metadata) = entry.metadata() {
          debug!("{:?}: {:?}", entry.path(), metadata);
          if metadata.is_file() {
            let _file_name = format!("{:?}", entry.file_name());
            files.push(FileEntry {
              name: entry.file_name().into_string().unwrap(),
              size: metadata.len(),
            });
          } else if metadata.is_dir() {
            let mut name = entry.path().to_str().unwrap().to_string();
            name.replace_range(..base_path_len, "");
            subfolders.push(FolderEntry { name: name })
          }
        } else {
          error!("Couldnt get file metadata")
        }
      }
    }
  }
  let listing = FolderListing {
    files: files,
    folders: subfolders,
  };
  listing
}

#[patch("/site/{site_name}/folder/{parent:.*}")]
pub async fn create(
  _identity: ReqData<Identity>,
  _data: Data<AppData>,
  new_folder_req: Json<NewFolder>,
  Path((site_name, parent)): Path<(String, String)>,
) -> Result<HttpResponse, ScriptError> {
  debug!("Creating folder {}", parent);

  let new_folder = new_folder_req.into_inner();
  let folder_name = &new_folder.name;

  let folder_path = format!("{}/{}/{}", site_name, parent, folder_name);

  match new_folder.create_folder(&folder_path).await {
    Ok(_) => {
      debug!("Created folder");
      Ok(HttpResponse::Created().finish())
    }
    Err(e) => {
      error!("Error creating dir {}", e);
      Err(ScriptError::FolderCreationError)
    }
  }
}

#[patch("/site/{site_name}/folder")]
pub async fn create_root(
  _identity: ReqData<Identity>,
  _data: Data<AppData>,
  new_folder_req: Json<NewFolder>,
  Path(site_name): Path<String>,
) -> Result<HttpResponse, ScriptError> {
  debug!("Creating folder in root");

  let new_folder = new_folder_req.into_inner();
  let folder_name = &new_folder.name;

  let folder_path = format!("{}/{}", site_name, folder_name);

  match new_folder.create_folder(&folder_path).await {
    Ok(_) => {
      debug!("Created folder");
      Ok(HttpResponse::Created().finish())
    }
    Err(e) => {
      error!("Error creating dir {}", e);
      Err(ScriptError::FolderCreationError)
    }
  }
}

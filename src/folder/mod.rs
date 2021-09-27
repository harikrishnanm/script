pub mod folder;
pub mod models;

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
use std::env;
use std::fs;

#[get("/site/{site_name}/folder/{folder:.*}")]
pub async fn get(
  identity: ReqData<Identity>,
  data: Data<AppData>,
  Path((site_name, folder)): Path<(String, String)>,
) -> Result<HttpResponse, ScriptError> {
  let root_path = match env::var("FILE_STORE_ROOT") {
    Ok(root) => root,
    Err(e) => {
      error!("Cannot read FILE_STORE_ROOT env variable. Will use default ./tmp");
      "/tmp".to_string()
    }
  };

  let full_path = format!("{}/{}/{}", root_path, site_name, folder);
  debug!("Getting details for {}", full_path);
  let path = std::path::Path::new(&full_path);
  for entry in fs::read_dir(full_path).unwrap() {
    let dir = entry.unwrap();
    debug!("dir {:?}", dir.path());
  }
  Ok(HttpResponse::Ok().finish())
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

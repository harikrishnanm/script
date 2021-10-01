pub mod asset;
pub mod models;

use actix_web::{
  post, web,
  web::{Data, Path, ReqData},
  HttpResponse,
};

use log::*;

use crate::common::utils;

use crate::asset::models::*;
use crate::error::ScriptError;
use crate::rbac::models::Identity;
use crate::AppData;

#[post("/site/{site_name}/collection/{collection_name}/asset")]
async fn save(
  identity: ReqData<Identity>,
  data: Data<AppData>,
  new_asset: web::Json<NewAsset>,
  Path((site_name, coll_name)): Path<(String, String)>,
) -> Result<HttpResponse, ScriptError> {
  debug!("Creating asset under {}/{}", site_name, coll_name);

  let db_pool = &data.db_pool;
  match new_asset
    .save(&identity, db_pool, &site_name, &coll_name)
    .await
  {
    Ok(_) => Ok(HttpResponse::Created().finish()),
    Err(e) => {
      error!("Error creating asset {}", e);
      return Err(ScriptError::AssetCreationError);
    }
  }

  //Ok(HttpResponse::Ok().finish())
}

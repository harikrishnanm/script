pub mod models;

use actix_web::{post, web, web::{Path, ReqData, Data}, HttpResponse};


use log::*;

use crate::rbac::models::Identity;
use crate::AppData;
use crate::error::ScriptError;

#[post("/site/{site_name}/collection/{collection_name}/asset")]
async fn get_text(
    identity: ReqData<Identity>,
    data: Data<AppData>,
    Path((site_name, coll_name)): Path<(String, String)>,
  ) -> Result<HttpResponse, ScriptError> {

    debug!("Creating asset under {}/{}", site_name, coll_name);

    Ok(HttpResponse::Ok().finish())



}
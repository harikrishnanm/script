pub mod models;
pub mod taxonomy;

use crate::common::utils;
use crate::error::ScriptError;
use crate::rbac::models::Identity;
use crate::taxonomy::models::*;
use crate::AppData;
use crate::DBPool;
use actix_web::{get, post, web, web::Path, HttpResponse};
use log::*;

#[get("/site/{site_name}/taxonomy/{taxonomy_name}")]
pub async fn get(
  identity: web::ReqData<Identity>,
  data: web::Data<AppData>,
  Path((site_name, taxonomy_name)): Path<(String, String)>,
) -> Result<HttpResponse, ScriptError> {
  let db_pool = &data.db_pool;

  match get_taxonomy(db_pool, &taxonomy_name, &site_name).await {
    Ok(result) => Ok(HttpResponse::Ok().json(result)),
    Err(e) => Err(ScriptError::UnexpectedError),
  }
}

#[post("/site/{site_name}/taxonomy")]
pub async fn save(
  identity: web::ReqData<Identity>,
  data: web::Data<AppData>,
  new_taxonomy: web::Json<NewTaxonomy>,
  Path(site_name): Path<String>,
) -> Result<HttpResponse, ScriptError> {
  info!("Got reqest for creating taxonomy {:?}", new_taxonomy.name);
  trace!("Identity {:?}", identity);

  let db_pool = &data.db_pool;

  match new_taxonomy.save(db_pool, &site_name).await {
    Ok(taxonomy) => Ok(HttpResponse::Created().json(taxonomy)),
    Err(e) => Err(ScriptError::UnexpectedError),
  }
}

#[post("/site/{site_name}/taxonomy/{taxonomy_name}/item")]
pub async fn save_item(
  identity: web::ReqData<Identity>,
  data: web::Data<AppData>,
  new_taxonomy_item: web::Json<NewTaxonomyItem>,
  Path((site_name, taxonomy_name)): Path<(String, String)>,
) -> Result<HttpResponse, ScriptError> {
  info!(
    "Got reqest for creating taxonomy item {:?}",
    new_taxonomy_item.item_name
  );
  trace!("Identity {:?}", identity);

  let db_pool = &data.db_pool;

  match new_taxonomy_item
    .save(db_pool, &site_name, &taxonomy_name)
    .await
  {
    Ok(taxonomy_item) => Ok(HttpResponse::Created().json(taxonomy_item)),
    Err(e) => Err(ScriptError::UnexpectedError),
  }
}

pub async fn get_taxonomy(
  db_pool: &DBPool,
  taxonomy_name: &str,
  site_name: &str,
) -> Result<Vec<TaxonomyItem>, ScriptError> {
  debug!("Getting taxonomy items for {}", taxonomy_name);

  let taxonomy_id = utils::get_taxonomy_id(&taxonomy_name, &site_name, db_pool)
    .await
    .unwrap();

  match sqlx::query_as!(
    TaxonomyItem,
    "SELECT taxonomy_id, taxonomy_item_id, taxonomy_item.item_name, 
      item_type, ordinal 
      FROM taxonomy_item
      WHERE taxonomy_id = $1
      ORDER BY ordinal ASC",
    taxonomy_id
  )
  .fetch_all(db_pool)
  .await
  {
    Ok(result) => Ok(result),
    Err(e) => Err(ScriptError::UnexpectedError),
  }
}

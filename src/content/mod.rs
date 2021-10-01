pub mod content;
pub mod models;

use actix_web::{get, http, post, put, web, web::Path, HttpResponse};
use log::*;
use r2d2_redis::redis;
use r2d2_redis::redis::Commands;

use crate::content::models::*;
use crate::error::ScriptError;
use crate::rbac::models::Identity;
use crate::AppData;
use std::ops::DerefMut;

use crate::RedisConnection;

#[get("/site/{site}/collection/{collection}/content/{content_name}")]
async fn get_text(
  identity: web::ReqData<Identity>,
  data: web::Data<AppData>,
  Path((site_name, coll_name, content_name)): Path<(String, String, String)>,
) -> Result<HttpResponse, ScriptError> {
  debug!("Getting {} in collection {}", coll_name, content_name);

  let cache_key = format!("script:{}:{}:{}", site_name, coll_name, content_name);

  let mut cache_conn: RedisConnection = data.redis_pool.get().unwrap();

  match redis::cmd("GET")
    .arg(&cache_key)
    .query::<String>(cache_conn.deref_mut())
  {
    Ok(result) => debug!("Got from redis cache {}", result),
    Err(e) => error!("Error getting from cache {}", e),
  }

  match sqlx::query!(
    "SELECT content, mime_type, cache_control FROM content
      WHERE site_name = $1 AND collection_name = $2 AND name = $3",
    site_name,
    coll_name,
    content_name
  )
  .fetch_one(&data.db_pool)
  .await
  {
    Ok(result) => {
      let content_str = &result.content;
      let cache_control: &str = &result.cache_control;
      let mime_header = match result.mime_type {
        Some(header) => header,
        None => "application/octet-stream".to_string(),
      };
      let mut builder = HttpResponse::Ok();
      match redis::cmd("SET")
        .arg(&cache_key)
        .arg(content_str)
        .query::<String>(cache_conn.deref_mut())
      {
        Ok(_) => debug!("Added to redis cache"),
        Err(e) => error!("Error addingh to cache {}", e),
      }
      Ok(
        builder
          .content_type(mime_header)
          .header(http::header::CACHE_CONTROL, cache_control)
          .body(content_str),
      )
    }
    Err(e) => {
      error!("Error getting content {}", e);
      Err(ScriptError::FileNotFound)
    }
  }
}

#[post("/site/{site}/collection/{collection}/content")]
async fn save(
  identity: web::ReqData<Identity>,
  data: web::Data<AppData>,
  new_content: web::Json<NewContent>,
  Path((site_name, coll_name)): Path<(String, String)>,
) -> Result<HttpResponse, ScriptError> {
  debug!("Got request for saving Content data");
  match new_content
    .save(
      &identity.into_inner(),
      &data.db_pool,
      &site_name,
      &coll_name,
    )
    .await
  {
    Ok(text) => Ok(HttpResponse::Created().json(text)),
    Err(e) => {
      error!("Error saving content text {}", e);
      Err(ScriptError::ContentCreationFailure)
    }
  }
}

#[put("/site/{site}/collection/{collection}/content/{content_name}")]
async fn update(
  identity: web::ReqData<Identity>,
  data: web::Data<AppData>,
  update_content: web::Json<UpdateContent>,
  Path((site_name, coll_name, content_name)): Path<(String, String, String)>,
) -> Result<HttpResponse, ScriptError> {
  debug!("Got request for updating content data");
  match update_content
    .update(
      &identity.into_inner(),
      &data.db_pool,
      &site_name,
      &coll_name,
      &content_name,
    )
    .await
  {
    Ok(text) => Ok(HttpResponse::Created().json(text)),
    Err(e) => {
      error!("Error saving content text {}", e);
      Err(ScriptError::ContentCreationFailure)
    }
  }
}

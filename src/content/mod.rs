pub mod models;
pub mod raw_content;

use actix_web::{get, http, post, put, web, web::Path, HttpResponse};
use log::*;

use crate::content::models::*;
use crate::error::ScriptError;
use crate::rbac::models::Identity;
use crate::AppData;

use crate::common::cache;
use crate::RedisConnection;

#[get("/site/{site}/collection/{collection}/content/{content_name}")]
async fn get(
    _identity: web::ReqData<Identity>,
    data: web::Data<AppData>,
    Path((site_name, coll_name, content_name)): Path<(String, String, String)>,
) -> Result<HttpResponse, ScriptError> {
    debug!("Getting {} in collection {}", coll_name, content_name);
    //Ok(HttpResponse::Ok().finish())
    let cache_key = format!("script:{}:{}:{}", site_name, coll_name, content_name);

    let _cache_conn: RedisConnection = data.redis_pool.get().unwrap();
    let content_response = match cache::get::<String>(&data.redis_pool, &cache_key) {
        Some(val) => {
            debug!("Got value from redis");
            trace!("Value: {}", val);
            serde_json::from_str(&val).unwrap()
        }
        None => {
            debug!("Value not found in redis. Getting data from database");
            match sqlx::query!(
                "SELECT content_item_raw.content, content_item_raw.mime_type, content.cache_control FROM content_item_raw
                 INNER JOIN content ON content_item_raw_id = content.content_item_id 
                 AND content.site_name = $1 AND content.collection_name = $2 and content.name = $3",
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

                    let content_response = ContentResponse {
                        cache_control: cache_control.to_string(),
                        content_str: content_str.to_string(),
                        mime_type: mime_header.clone(),
                    };
                    let content_response_json = serde_json::to_string(&content_response).unwrap();
                    cache::put::<String>(&data.redis_pool, &cache_key, content_response_json);
                    content_response
                }
                Err(e) => {
                    error!("Error getting content {}", e);
                    return Err(ScriptError::FileNotFound);
                }
            }
        }
    };

    let mut builder = HttpResponse::Ok();

    Ok(builder
        .content_type(&content_response.mime_type)
        .header(http::header::CACHE_CONTROL, content_response.cache_control)
        .body(content_response.content_str))
}

#[post("/site/{site}/collection/{collection}/content")]
async fn save(
    identity: web::ReqData<Identity>,
    data: web::Data<AppData>,
    raw_content: web::Json<NewRawContent>,
    Path((site_name, coll_name)): Path<(String, String)>,
) -> Result<HttpResponse, ScriptError> {
    debug!("Got request for saving Content data");
    match raw_content
        .save(
            &identity.into_inner(),
            &data.db_pool,
            &site_name,
            &coll_name,
        )
        .await
    {
        Ok(text) => {
            let coll_cache_key = format!("script:{}:{}", site_name, coll_name);
            cache::delete(&data.redis_pool, &coll_cache_key);
            Ok(HttpResponse::Created().json(text))
        }
        Err(e) => Err(ScriptError::ContentCreationFailure),
    }
}

#[put("/site/{site}/collection/{collection}/content/{content_name}")]
async fn update(
    identity: web::ReqData<Identity>,
    data: web::Data<AppData>,
    update_content: web::Json<UpdateContent>,
    Path((site_name, coll_name, content_name)): Path<(String, String, String)>,
) -> Result<HttpResponse, ScriptError> {
    info!(
        "Got request for updating collection content for {}",
        coll_name
    );
    debug!("Adding new content to collection {:?}", update_content);
    Ok(HttpResponse::Ok().finish())
    /*match update_content
      .update(
        &identity.into_inner(),
        &data.db_pool,
        &site_name,
        &coll_name,
        &content_name,
      )
      .await
    {
      Ok(text) => {
        debug!("Updated content. Now removing entries from cache(s)");
        let content_cache_key = format!("script:{}:{}:{}", site_name, coll_name, content_name);
        let coll_cache_key = format!("script:{}:{}", site_name, coll_name);
        cache::delete(&data.redis_pool, &content_cache_key);
        cache::delete(&data.redis_pool, &coll_cache_key);
        Ok(HttpResponse::Created().json(text))
      }
      Err(e) => {
        error!("Error saving content text {}", e);
        Err(ScriptError::ContentCreationFailure)
      }
    }*/
}

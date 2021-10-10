pub mod collection;
pub mod models;

use crate::collection::models::*;
use crate::error::ScriptError;
use crate::rbac;
use crate::rbac::models::Identity;
use crate::AppData;
use actix_web::{get, http::header::*, post, web, web::Path, HttpResponse};
use futures::try_join;
use log::*;

use crate::common::cache;
use crate::RedisConnection;

#[get("/site/{site_name}/collection/{collection_name}")]
pub async fn get(
    identity: web::ReqData<Identity>,
    data: web::Data<AppData>,
    Path((site_name, collection_name)): Path<(String, String)>,
) -> Result<HttpResponse, ScriptError> {
    info!(
        "Getting collection {}/{} for {}",
        site_name, collection_name, identity.user
    );
    let cache_key = format!("script:{}:{}", site_name, collection_name);
    let _cache_conn: RedisConnection = data.redis_pool.get().unwrap();

    let mut cache_control = String::new();

    let cached_collection_response = match cache::get::<String>(&data.redis_pool, &cache_key) {
        Some(val) => {
            debug!("Got value from redis");
            trace!("Cached Value {}", val);
            serde_json::from_str(&val).unwrap()
        }
        None => {}
    };

    /*let response = CollectionResponse {
        name: cached_collection_response.name,
        contents: cached_collection_response.contents,
        assets: cached_collection_response.assets,
    };

    let mut builder = HttpResponse::Ok();
    let k: actix_web::HttpResponse = builder
        .header(CACHE_CONTROL, cached_collection_response.cache_control)
        .json(response);
    // get all text/content  for given

    match sqlx::query!(
        "SELECT * FROM content WHERE site_name = $1 AND collection_name = $2",
        site_name,
        collection_name
    )
    .fetch_all(&data.db_pool)
    .await
    {
        Ok(rows) => {
            for row in rows {
                debug!("Got row {:?}", row);
                let content = Content {
                    id: row.content_id,
                    name: row.name.clone(),
                    content: row.content,
                    mime_type: row.mime_type.unwrap(),
                    tags: row.tags,
                    url: format!(
                        "/site/{}/collection/{}/content/{}",
                        site_name, collection_name, row.name
                    ),
                };
                contents.push(content);
            }
        }
        Err(e) => {
            error!("Error getting content {}", e);
            return Err(ScriptError::FileNotFound);
        }
    }

    // Get assets

    let response = CollectionResponse {
        name: collection_name,
        content: contents,
    };

    let mut builder = HttpResponse::Ok();
    Ok(builder.header(CACHE_CONTROL, cache_control).json(response))*/
    Ok(HttpResponse::Ok().finish())
}

#[post("/site/{site_name}/collection")]
pub async fn save(
    identity: web::ReqData<Identity>,
    data: web::Data<AppData>,
    new_collection: web::Json<NewCollection>,
    Path(site_name): Path<String>,
) -> HttpResponse {
    match new_collection
        .save(site_name, identity.into_inner(), &data.db_pool)
        .await
    {
        Ok(coll) => {
            rbac::reload_rbac(&data).await.unwrap();
            HttpResponse::Ok().json(coll)
        }
        Err(_e) => HttpResponse::Conflict().finish(),
    }
}

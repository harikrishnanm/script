pub mod collection;
pub mod models;

use crate::collection::models::*;
use crate::error::ScriptError;
use crate::rbac::models::Identity;
use crate::AppData;
use actix_web::{get, http::header::*, post, web, web::Path, HttpResponse};
use futures::try_join;
use log::*;

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

    let mut cache_control = String::new();

    // get header
    match sqlx::query!(
        "SELECT name, cache_control FROM collection WHERE site_name = $1 AND name = $2",
        site_name,
        collection_name
    )
    .fetch_one(&data.db_pool)
    .await
    {
        Ok(result) => cache_control = result.cache_control,
        Err(e) => {
            error!(
                "Couldnt get cache control header..will not process further {}",
                e
            );
            return Err(ScriptError::FileNotFound);
        }
    };

    // get all text/content  for given

    let mut contents: Vec<Content> = Vec::new();
    let mut assets: Vec<Asset> = Vec::new();

    let content_query_fut = sqlx::query_as!(
        Content,
        "SELECT content_id as id, name, content, mime_type, tags, 
        site_name||'/'||collection_name||'/'||name as url
         FROM content WHERE site_name = $1 AND collection_name = $2",
        site_name,
        collection_name
    )
    .fetch_all(&data.db_pool);

    let asset_query_fut = sqlx::query_as!(
        Asset,
        "SELECT asset.asset_id as id, asset.name, 
            file.name as file_name, file.mime_type, file.path, file.size
        FROM asset 
        INNER JOIN file 
        ON asset.file_id = file.file_id 
        AND asset.coll_name = $1
        AND asset.site_name = $2",
        collection_name,
        site_name
    )
    .fetch_all(&data.db_pool);

    match try_join!(content_query_fut, asset_query_fut) {
        Ok((contents, assets)) => {
            debug!("Result {:?},## {:?}", contents, assets);
            let response = CollectionResponse {
                name: collection_name,
                contents: contents,
                assets: assets,
            };
            let mut builder = HttpResponse::Ok();
            Ok(builder.header(CACHE_CONTROL, cache_control).json(response))
        }
        Err(e) => {
            error!("Error {}", e);
            Err(ScriptError::FileNotFound)
        }
    }

    /*match sqlx::query!(
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
    }*/

    // Get assets

    /*let response = CollectionResponse {
        name: collection_name,
        content: contents,
    };

    let mut builder = HttpResponse::Ok();
    Ok(builder.header(CACHE_CONTROL, cache_control).json(response))*/
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
        Ok(coll) => HttpResponse::Ok().json(coll),
        Err(_e) => HttpResponse::Conflict().finish(),
    }
}

use crate::DBPool;
use sqlx::Error;
use uuid::Uuid;

use log::*;

pub async fn get_file_name(file_id: &Uuid, db_pool: &DBPool) -> Result<String, Error> {
    let file_name: String = match sqlx::query!("SELECT name FROM file WHERE file_id = $1", file_id)
        .fetch_one(db_pool)
        .await
    {
        Ok(result) => result.name,
        Err(e) => {
            error!("Error getting file name {}", e);
            return Err(e);
        }
    };
    Ok(file_name)
}

pub async fn get_site_id(site_name: &str, db_pool: &DBPool) -> Result<Uuid, Error> {
    let site_id: Uuid = match sqlx::query!("SELECT site_id FROM site where name = $1", site_name)
        .fetch_one(db_pool)
        .await
    {
        Ok(site_id) => site_id.site_id,
        Err(e) => {
            error!("Could not fetch site id {}", e);
            return Err(e);
        }
    };
    Ok(site_id)
}

pub async fn get_collection_id(
    site_name: &str,
    collection_name: &str,
    db_pool: &DBPool,
) -> Result<Uuid, Error> {
    let collection_id: Uuid = match sqlx::query!(
        "SELECT collection_id FROM collection where site_name = $1 and name = $2",
        site_name,
        collection_name
    )
    .fetch_one(db_pool)
    .await
    {
        Ok(collection_id) => collection_id.collection_id,
        Err(e) => {
            error!("Could not fetch collection id {}", e);
            return Err(e);
        }
    };
    Ok(collection_id)
}

pub async fn get_site_and_coll_id(
    site_name: &str,
    collection_name: &str,
    db_pool: &DBPool,
) -> Result<(Uuid, Uuid), Error> {
    debug!("Getting site_id and collection_id");
    let site_id = match get_site_id(site_name, db_pool).await {
        Ok(site_id) => site_id,
        Err(e) => {
            return Err(e);
        }
    };

    let coll_id = match get_collection_id(site_name, collection_name, db_pool).await {
        Ok(coll_id) => coll_id,
        Err(e) => {
            return Err(e);
        }
    };

    Ok((site_id, coll_id))
}

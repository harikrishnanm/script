use crate::asset::models::*;
use crate::rbac::models::Identity;
use crate::DBPool;
use log::*;
use sqlx::Error;
use uuid::Uuid;

use crate::common::utils;

impl NewAsset {
  pub async fn save(
    self: &Self,
    _identity: &Identity,
    db_pool: &DBPool,
    site_name: &str,
    coll_name: &str,
  ) -> Result<(), Error> {
    debug!("Saving new asset at {}/{}", site_name, coll_name);
    let asset_id = Uuid::new_v4();
    let _coll_id = match utils::get_collection_id(site_name, coll_name, db_pool).await {
      Ok(coll_id) => coll_id,
      Err(e) => {
        error!(
          "Error saving asset data {}. Could not get site_id / coll_name",
          e
        );
        return Err(e);
      }
    };

    let (site_id, coll_id) =
      match utils::get_site_and_coll_id(&site_name, &coll_name, db_pool).await {
        Ok(r) => r,
        Err(e) => {
          error!("Error getting site/coll id {}", e);
          return Err(e);
        }
      };

    /*let file_id_uuid = match Uuid::parse_str(&self.file_id) {
      Ok(uuid) => uuid,
      Err(e) => {
        error!("Error reading uuid {}", e);
        Uuid::new_v4()
      }
    };*/

    match sqlx::query!(
      "INSERT INTO asset (asset_id, name, file_id, coll_id, coll_name, site_id, site_name)
       VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING asset_id",
      asset_id,
      self.name,
      self.file_id,
      coll_id,
      coll_name,
      site_id,
      site_name
    )
    .fetch_one(db_pool)
    .await
    {
      Ok(result) => {
        debug!("Inserted asset record {:?}", result);
        Ok(())
        /*
        Example query......
        select asset.asset_id, asset.file_id, asset.name, collection.collection_id,
        collection.name, site.name, site.site_id FROM asset INNER JOIN
        collection ON asset.coll_id=collection.collection_id AND
        asset.asset_id='df234481-0475-4f0a-9465-d4818ba50594'
        INNER JOIN site ON collection.site_id = site.site_id;
        match sqlx::query_as!(
          Asset,
          "SELECT asset.site_id, asset.file_id, asset.name, site.name AS site_name,
          coll_id, collection.name AS coll_name,
          file.name AS file_name FROM asset
        INNER JOIN site ON asset.site_id = site.site_id AND asset.asset_id = $1
        INNER JOIN collection ON asset.coll_id = collection.collection_id
        INNER JOIN file ON asset.file_id = file.file_id",
          result.asset_id
        )
        .fetch_one(db_pool)
        .await
        {
          Ok(asset) => Ok(asset),
          Err(e) => {
            error!("Error getting asset details {}", e);
            Err(e)
          }
        }*/
      }
      Err(e) => {
        error!("Error creating asset {}", e);
        return Err(e);
      }
    }
  }
}

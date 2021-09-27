use crate::asset::models::*;
use crate::rbac::models::Identity;
use crate::DBPool;
use log::*;
use sqlx::Error;
use uuid::Uuid;

use crate::common::utils;

impl NewAsset {
  /*pub async fn save(
    self: &Self,
    identity: &Identity,
    db_pool: &DBPool,
    site_name: &str,
    coll_name: &str,
  ) -> Result<Asset, Error> {
    debug!("Saving new asset at {}/{}", site_name, coll_name);
    let asset_id = Uuid::new_v4();
    let (site_id, coll_id) = utils::get_site_and_coll_id(site_name, coll_name , db_pool).await{
      Ok((site_id, coll_id)) => (site_id, coll_id),
      Err(e) {
        error!("Error saving asset data {}", e);
        Err(e)
      }
    };

    match sqlq::query_as!(Asset,
    "INSERT INTO asset (asset_id, name, file_id, site_id, coll_id)
    VALUES ($1, $2, $3, $4, $5)
    RETURNING name, file_id, file_name, site_id, site_name, coll_id, coll_name",
    asset_id,
    self.name,
    self.file_id,
    site_id,
    coll_id
    ).fetch_one_

    )

  }*/
}

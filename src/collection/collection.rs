use crate::collection::models::*;
use crate::rbac::models::Identity;
use crate::DBPool;
use log::*;
use sqlx::Error;
use uuid::Uuid;

impl NewCollection {
    pub async fn save(
        self: &Self,
        site_name: String,
        identity: Identity,
        db_pool: &DBPool,
    ) -> Result<Collection, Error> {
        debug!("Creating collection {:?}", self);

        let collection_id = Uuid::new_v4();

        let site_id = match sqlx::query!("SELECT site_id FROM site WHERE name = $1", site_name)
            .fetch_one(db_pool)
            .await
        {
            Ok(result) => result.site_id,
            Err(e) => {
                error!("Error fetching site_id {}", e);
                return Err(e);
            }
        };

        match sqlx::query_as!(
      Collection,
      "INSERT INTO collection (collection_id, name, parent_id, cache_control, site_id, site_name, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
          RETURNING collection_id, name, parent_id, cache_control, site_id, site_name, created_by",
      collection_id,
      self.name,
      self.parent_id,
      match &self.cache_control {
        Some(val) => val,
        None => "max-age=0, no-store, must-revalidate",
      },
      site_id,
      site_name,
      identity.user,
    )
        .fetch_one(db_pool)
        .await
        {
            Ok(collection) => Ok(collection),
            Err(e) => {
                error!("Error creating collection {}", e);
                Err(e)
            }
        }
    }
}

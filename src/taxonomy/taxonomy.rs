use crate::taxonomy::models::*;
use log::*;
use uuid::Uuid;

use crate::common::utils;
use crate::error::ScriptError;
use crate::DBPool;
use sqlx::Error;

impl NewTaxonomy {
    pub async fn save(self: &Self, db_pool: &DBPool, site_name: &str) -> Result<Taxonomy, Error> {
        debug!("Creating new taxonomy");
        trace!("{:?}", self);

        let site_id = utils::get_site_id(site_name, db_pool).await.unwrap();
        let taxonomy_id = Uuid::new_v4();
        match sqlx::query_as!(
            Taxonomy,
            "INSERT INTO taxonomy (taxonomy_id, name, site_id, site_name) 
      VALUES ($1, $2, $3, $4) RETURNING taxonomy_id, name, site_id, site_name",
            taxonomy_id,
            &self.name,
            site_id,
            site_name
        )
        .fetch_one(db_pool)
        .await
        {
            Ok(taxonomy) => {
                debug!("New taxonomy created");
                Ok(taxonomy)
            }
            Err(e) => {
                error!("Error creating taxonomy {}", e);
                Err(e)
            }
        }
    }
}

impl NewTaxonomyItem {
    pub async fn save(
        self: &Self,
        db_pool: &DBPool,
        site_name: &str,
        taxonomy_name: &str,
    ) -> Result<TaxonomyItem, ScriptError> {
        debug!("Creating new taxonomy");
        let taxonomy_item_id = Uuid::new_v4();
        let _site_id = utils::get_site_id(site_name, db_pool).await.unwrap();
        let taxonomy_id = utils::get_taxonomy_id(taxonomy_name, site_name, db_pool)
            .await
            .unwrap();

        debug!("Checking if arr_type_taxonomy_id is specified for array type");
        let item_type = &self.item_type[..];
        trace!("Item type in request {}", item_type);

        let type_a = "A".to_string();
        let type_o = "O".to_string();
        match item_type {
            "A" => {
                if let None = &self.item_taxonomy_id {
                    return Err(ScriptError::BadRequest(
                        "Taxonomy for array item is not specified".to_string(),
                    ));
                }
            }
            "O" => {
                if let None = &self.item_taxonomy_id {
                    return Err(ScriptError::BadRequest(
                        "Taxonomy for the nested object is not specified".to_string(),
                    ));
                }
            }
            "N" | "S" | "B" => {
                debug!("Non array type");
            }
            _ => {
                error!("Invalid type");
                return Err(ScriptError::BadRequest("Invalid type".to_string()));
            }
        }

        match sqlx::query_as!(TaxonomyItem,
        "INSERT INTO taxonomy_item (taxonomy_item_id, taxonomy_id, item_name, item_type, ordinal, item_taxonomy_id) 
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING taxonomy_item_id, taxonomy_id, item_name, item_type, ordinal, item_taxonomy_id",
        taxonomy_item_id,
        taxonomy_id,
        &self.item_name,
        &self.item_type,
        &self.ordinal,
        self.item_taxonomy_id
        )
        .fetch_one(db_pool)
        .await
        {
            Ok(taxonomy_item) => Ok(taxonomy_item),
            Err(e) => {
                error!("Error creating taxonomy item {}", e);
                Err(ScriptError::DatabaseError(e.to_string()))
            }
        }
    }
}

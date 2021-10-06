use crate::{error::ScriptError, rbac::models::Identity};
use crate::content::models::*;
use crate::{DBPool, taxonomy};
use crate::common::utils;
use log::*;
use crate::taxonomy::models::TaxonomyItem;
use serde_json::{Value, Map};
use sqlx::{Error, Transaction, Postgres};
use uuid::Uuid;




impl NewContent{
    pub async fn save(
        self: &Self,
        identity: &Identity,
        db_pool: &DBPool,
        site_name: &str,
        coll_name: &str,
    ) -> Result<(), Error> {
        let content_id = Uuid::new_v4();
        let content_item_id = Uuid::new_v4();

        let (site_id, collection_id) = match utils::get_site_and_coll_id(site_name, coll_name, db_pool).await {
            Ok(site_id) => site_id,
            Err(e) => {
                error!("Could not fetch site id {}", e);
                return Err(e);
            }
        };

        let mut tx = match db_pool.begin().await {
            Ok(tx) => tx,
            Err(e) => {
                error!("Could not start update transaction");
                return Err(e);
            }
        };


        //Check for content type and save to respective table
        //If raw the uuid is directly inserted into the content table as uuid
        //If not raw, the content_set id is inserted into the content table.
        // The ordinal in the content storage tables can be used to group the content set.
        //all conent instances with the same ordinal will be part of a group.

        let mut updated_tags = self.tags.clone();
        updated_tags.push(self.name.clone());
        match &self.taxonomy_id {
            None => {
                debug!("Raw content recieved");
                let content_str = match &self.content {
                    Value::String(val) => val,
                    _ => return Ok(())
                };
                let content_item_raw_id = Uuid::new_v4();
                match sqlx::query!(
                "INSERT INTO content_item_raw (content_item_raw_id, content) VALUES ($1, $2)", 
                content_item_raw_id,
                content_str,)
                .execute(&mut tx)
                .await {
                    Ok(_) => {
                        debug!("Content item created");
                    },
                    Err(e) => {
                        error!("Error creating content_item ");
                        tx.rollback().await?;
                        return Err(e);
                    }
                }
            },
            Some(taxonomy_id) => {
                debug!("Structured content recieved. getting taxonomy");
                match &self.content {
                    Value::Object(content_obj) => {
                        
        
                    },
                    _ => {
                        error!("Cannot save content since its not an object")
                    }
                }
                

                return Ok(());   
                
         
            }
        }


        //Update content table

        let cache_control_str = match &self.cache_control{
            Some(value) => value,
            None => "private"
        };

        match sqlx::query!(
            "INSERT INTO content 
            (content_id, name, tags, site_id, site_name, collection_id, collection_name, 
                content_item_id, raw, taxonomy_id, cache_control, created_by ) 
            VALUES 
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
        content_id, 
        &self.name,
        &updated_tags,
        site_id,
        site_name, 
        collection_id, 
        coll_name,
        content_item_id,
        &self.raw,
        self.taxonomy_id,
        &cache_control_str,
        identity.user,
        ).execute(&mut tx).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    async fn check_taxonomy (db_pool: &DBPool, taxonomy_id: Uuid, content_map: Map<String, Value>) -> Result<bool, ScriptError>{

        let taxonomy_items = match utils::get_taxonomy_items(taxonomy_id, db_pool).await {
            Ok(taxonomy_items) => taxonomy_items,
            Err(e) => return Err(ScriptError::UnexpectedError)
        };
        
        let keys: Vec<&String> = content_map.keys()
            .map(|key| key)
            .collect();
        debug!("Keys from the request {:?}", keys);

        let taxonomy_keys: Vec<&String> = taxonomy_items.iter()
            .map(|taxonomy_item: &TaxonomyItem| &taxonomy_item.item_name)
            .collect::<Vec<&String>>();
        debug!("Keys from the taxonomy {:?}", taxonomy_keys);

        let key_match = utils::do_vecs_match::<String>(&keys, &taxonomy_keys);
        debug!("Keys match {}", key_match);


        //Check if keys and content_item_keys match
        Ok(key_match)
    }
}


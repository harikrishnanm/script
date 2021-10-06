use crate::{error::ScriptError, rbac::models::Identity};
use crate::content::models::*;
use crate::DBPool;
use crate::common::utils;
use log::*;
use serde_json::Value;
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
        match &self.raw {
            true => {
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
            false => {
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
            (content_id, name, tags, site_id, site_name, collection_id, collection_name, content_item_id, raw, taxonomy_id, cache_control, created_by ) 
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


        /*match sqlx::query_as!(
            Content,
            "INSERT INTO content (content_id, name, mime_type, tags, site_id, site_name, 
                collection_id, collection_name, content_item_id, cache_control, created_by)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) 
                RETURNING content_id, name, mime_type, site_id, 
                    collection_id, content_length, tags, created_by, modified, version",
            content_id,
            self.name,
            self.mime_type,
            &updated_tags,
            site_id,
            site_name, 
            collection_id,
            coll_name,
            &updated_tags,
            
            self.content.len() as i32,
            match &self.cache_control {
                Some(val) => val,
                None => "max-age=0, no-store, must-revalidate",
            },
            identity.user
        )
        .fetch_one(&mut tx)
        .await
        {
            Ok(text) => Ok(text),
            Err(e) => {
                error!("Error saving content {}", e);
                Err(e)
            }
        }*/
    }
}


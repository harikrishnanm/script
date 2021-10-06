
use crate::{error::ScriptError, rbac::models::Identity};
use crate::content::models::*;
use crate::DBPool;
use crate::common::utils;
use log::*;

use sqlx::{Error, Transaction, Postgres};
use uuid::Uuid;



///Update content implementation
impl UpdateContent {
    pub async fn update(
        self: &Self,
        _identity: &Identity,
        db_pool: &DBPool,
        site_name: &str,
        coll_name: &str,
        text_name: &str,
    ) -> Result<Content, Error> {
        debug!("Updating content");
        info!("Updating content");
        let mut tx = match db_pool.begin().await {
            Ok(tx) => tx,
            Err(e) => {
                error!("Could not start update transaction");
                return Err(e);
            }
        };

        debug!("Archiving existing version");

        match sqlx::query!("INSERT INTO content_archive 
                (content_id, name,  mime_type, tags ,site_id, site_name, collection_id, collection_name, 
                content, content_length, cache_control, version, created_by, created, modified)
            SELECT 
                content_id, name,  mime_type, tags ,site_id, site_name, collection_id, collection_name, 
                content, content_length, cache_control, version, created_by, created, modified
                from content
            WHERE 
                site_name = $1 AND collection_name = $2 AND name = $3", 
        site_name,
        coll_name,
        text_name
        ).execute(&mut tx).await{
          Ok(_) => debug!("Inserted into archive"),
          Err(e) => {
              error!("Error archiving {}", e);
              let _ = tx.rollback().await;
              return Err(e);
            }
        };
        debug!("Archival complete");

        let (old_version, mut mime_type, mut content, mut cache_control, mut tags) =
            match sqlx::query!(
            "SELECT  mime_type, tags, content, cache_control, version FROM content WHERE site_name = $1 AND collection_name = $2 AND name = $3",
            site_name, coll_name, text_name
        )
            .fetch_one(&mut tx)
            .await
            {
                Ok(result) => {

                    (result.version,
                        match result.mime_type {
                        Some(val) => val,
                        None => "".to_string()
                    },
                    result.content, 
                    result.cache_control,
                    result.tags)
                    //tags = result.tags;
                    //cache_control= result.cache_control;
                    //content = result.content;
                    
                }
                Err(e) => {
                    error!("Error getting old version {}", e);
                    return Err(e);
                }
            };

        debug!("Old version {}", old_version);
        debug!("Updating content...");
        //Check all the options and if any are none, do not update.
        debug!("Checking which value to be updated");
        match &self.mime_type {
            Some(new_mime_type) => {
                debug!("mime_type needs to be updated");
                mime_type = new_mime_type.to_string();
            }
            None => (),
        };
        match &self.cache_control {
            Some(new_cache_control) => {
                debug!("cache_control needs to be updated");
                cache_control = new_cache_control.to_string();
            }
            None => (),
        };
        match &self.tags {
            Some(new_tags) => {
                debug!("tags needs to be updated");
                tags = new_tags.to_vec();
            }
            None => (),
        };
        match &self.content {
            Some(new_content) => {
                debug!("content needs to be updated");
                content = new_content.to_string();
            }
            None => (),
        };

        match sqlx::query_as!(
            Content,
            "UPDATE content SET mime_type = $1, tags = $2, 
                content = $3, content_length = $4,  cache_control = $5, version = $6
            WHERE site_name = $7 AND collection_name = $8 AND name = $9 
            RETURNING content_id, name, mime_type, site_id, 
                collection_id, content_length, tags, created_by, modified, version",
            mime_type,
            &tags,
            match content.len() {
                0 => "".to_string(),
                _ => content.to_string(),
            },
            content.len() as i32,
            cache_control,
            old_version + 1,
            site_name,
            coll_name,
            text_name
        )
        .fetch_one(&mut tx)
        .await
        {
            Ok(new_text) => {
                debug!("Can update as expected..commiting now");
                match tx.commit().await {
                    Ok(_) => {
                        debug!("Update complete");
                        Ok(new_text)
                    }
                    Err(e) => {
                        error!("Error commiting content {}", e);
                        Err(e)
                    }
                }
            }
            Err(e) => {
                error!("Error updating text {}", e);
                Err(e)
            }
        }
    }
}

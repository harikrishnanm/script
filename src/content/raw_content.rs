use crate::common::utils;
use crate::content::models::*;
use crate::DBPool;
use crate::{error::ScriptError, rbac::models::Identity};
use log::*;
use uuid::Uuid;

impl NewRawContent {
    pub async fn save(
        self: &Self,
        identity: &Identity,
        db_pool: &DBPool,
        site_name: &str,
        coll_name: &str,
    ) -> Result<(), ScriptError> {
        debug!("Saving raw content");

        let content_id = Uuid::new_v4();
        let content_item_id = Uuid::new_v4();

        let (site_id, collection_id) =
            match utils::get_site_and_coll_id(site_name, coll_name, db_pool).await {
                Ok(site_id) => site_id,
                Err(e) => {
                    error!("Could not fetch site id {}", e);
                    return Err(ScriptError::UnexpectedError);
                }
            };

        debug!(
            "Fetched site_id and collection_id {}, {}",
            site_id, collection_id
        );

        let mut tx = match db_pool.begin().await {
            Ok(tx) => tx,
            Err(e) => {
                error!("Could not start update transaction");
                return Err(ScriptError::TransactionError);
            }
        };

        debug!("Intitialized transaction");

        let mut updated_tags = self.tags.clone();
        updated_tags.push(self.name.clone());

        debug!("Final set of tags {:?}", updated_tags);

        let content_item_raw_id = Uuid::new_v4();
        match sqlx::query!(
            "INSERT INTO content_item_raw (content_item_raw_id, content, name) VALUES ($1, $2, $3)",
            content_item_raw_id,
            &self.content,
            &self.name,
        )
        .execute(&mut tx)
        .await
        {
            Ok(_) => {
                debug!("Content item created");
            }
            Err(e) => {
                error!("Error creating content item {} ", e);
                tx.rollback().await.unwrap();
                return Err(ScriptError::UnexpectedError);
            }
        }
        //Update content table

        let cache_control_str = match &self.cache_control {
            Some(value) => value,
            None => "private",
        };

        match sqlx::query!(
            "INSERT INTO content 
            (content_id, name, tags, site_id, site_name, collection_id, collection_name, 
                content_item_id, cache_control, created_by ) 
            VALUES 
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
            content_id,
            &self.name,
            &updated_tags,
            site_id,
            site_name,
            collection_id,
            coll_name,
            content_item_raw_id,
            &cache_control_str,
            identity.user,
        )
        .execute(&mut tx)
        .await
        {
            Ok(_) => match tx.commit().await {
                Ok(_) => Ok(()),
                Err(e) => {
                    error!("Error commiting transaction {}", e);
                    Err(ScriptError::ContentCreationFailure)
                }
            },
            Err(e) => {
                error!("Error creating content header {}", e);
                Err(ScriptError::UnexpectedError)
            }
        }
    }
}

/*


async fn save_map<'a>(self: &Self, tx: &Transaction<'a, Postgres>, taxonomy_id: &Uuid) {
        debug!("Saving data for taxonomy {}", taxonomy_id);
        let content_map = &self.content;
        for (key, value) in content_map {
            match value {
                Value::String(val) => {
                    match sqlx::query!{
                        "INSERT INTO content_item_text ("
                    }
                }
            }
        }
    }

async fn check_taxonomy(
        self: &Self,
        db_pool: &DBPool,
        taxonomy_id: &Uuid,
    ) -> Result<bool, ScriptError> {
        let taxonomy_items = match utils::get_taxonomy_items(taxonomy_id, db_pool).await {
            Ok(taxonomy_items) => taxonomy_items,
            Err(e) => return Err(ScriptError::UnexpectedError),
        };

        let mut key_match: bool = false;
        if let Value::Object(content_map) = &self.content {
            let keys: Vec<&String> = content_map.keys().map(|key| key).collect();
            debug!("Keys from the request {:?}", keys);

            let taxonomy_keys: Vec<&String> = taxonomy_items
                .iter()
                .map(|taxonomy_item: &TaxonomyItem| &taxonomy_item.item_name)
                .collect::<Vec<&String>>();
            debug!("Keys from the taxonomy {:?}", taxonomy_keys);

            key_match = utils::do_vecs_match::<String>(&keys, &taxonomy_keys);
            debug!("Keys match {}", key_match);
        } else {
            Err(ScriptError::BadRequest(
                "Content is not structured properly",
            ))
        }
        Ok(key_match)
    }


*/

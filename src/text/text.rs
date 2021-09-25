use crate::rbac::models::Identity;
use crate::text::models::*;
use crate::DBPool;
use log::*;
use sqlx::Error;
use uuid::Uuid;

impl NewText {
    pub async fn save(
        self: &Self,
        identity: &Identity,
        db_pool: &DBPool,
        site_name: &str,
        coll_name: &str,
    ) -> Result<Text, Error> {
        let text_id = Uuid::new_v4();

        struct SiteId {
            site_id: Uuid,
        }

        struct CollectionId {
            collection_id: Uuid,
        }
        let site_id: Uuid = match sqlx::query_as!(
            SiteId,
            "SELECT site_id FROM site where name = $1",
            site_name
        )
        .fetch_one(db_pool)
        .await
        {
            Ok(site_id) => site_id.site_id,
            Err(e) => {
                error!("Could not fetch site id {}", e);
                return Err(e);
            }
        };

        let collection_id: Uuid = match sqlx::query_as!(
            CollectionId,
            "SELECT collection_id FROM collection where site_name = $1 and name = $2",
            site_name,
            coll_name
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

        let mut updated_tags = self.tags.clone();
        updated_tags.push(self.name.clone());

        match sqlx::query_as!(
            Text,
            "INSERT INTO text (text_id, name, mime_type, site_id, site_name, 
          collection_id, collection_name, tags, content, 
            content_length, cache_control, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) 
          RETURNING text_id, name, mime_type, site_id, 
            collection_id, content_length, tags, created_by, modified, version",
            text_id,
            self.name,
            self.mime_type,
            site_id,
            site_name,
            collection_id,
            coll_name,
            &updated_tags,
            match self.content.len() {
                0 => "".to_string(),
                _ => self.content.to_string(),
            },
            self.content.len() as i32,
            match &self.cache_control {
                Some(val) => val,
                None => "max-age=0, no-store, must-revalidate",
            },
            identity.user
        )
        .fetch_one(db_pool)
        .await
        {
            Ok(text) => Ok(text),
            Err(e) => {
                error!("Error saving text {}", e);
                Err(e)
            }
        }
    }
}

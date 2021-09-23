use crate::constants;
use crate::DBPool;
use chrono::NaiveDateTime;
use log::*;
use serde::{Deserialize, Serialize};
use sqlx::Error;
use uuid::Uuid;

use crate::rbac::models::*;
const OWNER: &str = "O";
const SITE_BASE_PATH: &str = "/site/";

#[derive(Deserialize, Debug)]
pub struct NewSite {
  pub name: String,
  pub path: String,
  pub slug: Option<String>,
  pub url: Option<String>,
  pub cors_enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
  site_id: Uuid,
  name: String,
  path: String,
  slug: Option<String>,
  url: Option<String>,
  cors_enabled: Option<bool>,
  created_by: String,
  created: NaiveDateTime,
  modified: NaiveDateTime,
}

impl NewSite {
  pub async fn save(self: &Self, identity: Identity, db_pool: &DBPool) -> Result<Site, Error> {
    //Start first transaction

    let mut tx = match db_pool.begin().await {
      Ok(tx) => tx,
      Err(e) => {
        error!("Could not start transaction {}", e);
        return Err(e); //Error out
      }
    };

    let site_id = uuid::Uuid::new_v4();
    let new_site = match sqlx::query_as!(
      Site,
      "INSERT INTO site (site_id, name, path, slug, url, cors_enabled, created_by)
          VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING site_id, name, path, slug, url, cors_enabled, created_by, created, modified",
      site_id,
      &self.name,
      &self.path,
      match &self.slug {
        Some(slug) => slug,
        None => "",
      },
      match &self.url {
        Some(url) => url,
        None => "",
      },
      self.cors_enabled,
      identity.user
    )
    .fetch_one(&mut tx)
    .await
    {
      Ok(new_site) => new_site,
      Err(e) => {
        error!("Error saving site data {}", e);
        return Err(e);
      }
    };

    let mut site_path = SITE_BASE_PATH.to_owned();
    let user = &identity.user;
    site_path.push_str(&self.name);

    // Create new RBAC Policy
    let default_rbac_policy = NewRbacPolicy {
      path: site_path,
      path_match: constants::EXACT.to_string(),
      method: constants::WILDCARD.to_string(),
      rbac_user: user.to_string(),
      rbac_role: constants::WILDCARD.to_string(),
      description: None,
    };

    match default_rbac_policy.save_tx(&mut tx, &identity).await {
      Ok(_) => {
        debug!("Created rbac policy");
        match tx.commit().await {
          Ok(_) => Ok(new_site),
          Err(e) => Err(e),
        }
      }
      Err(e) => {
        error!("Error creating RBAC policy {}", e);
        let _ = tx.rollback().await;
        Err(e)
      }
    }
  }
}

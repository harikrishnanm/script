use crate::auth::Authority;
use crate::auth::Identity;
use crate::auth::NewRbacPolicy;
use crate::constants;
use crate::AppData;
use crate::DBPool;
use actix_web::{post, web, HttpResponse};
use chrono::NaiveDateTime;
use log::{debug, error, info, trace};
use serde::{Deserialize, Serialize};
use sqlx::Error;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

const OWNER: &str = "O";
const SITE_BASE_PATH: &str = "/site/";

#[derive(Deserialize, Debug)]
pub struct NewSite {
  name: String,
  path: String,
  slug: Option<String>,
  url: Option<String>,
  cors_enabled: Option<bool>,
  authorities: Vec<Authority>,
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
      path_match: "EXACT".to_string(),
      method: constants::WILDCARD.to_string(),
      rbac_user: user.to_string(),
      rbac_role: constants::WILDCARD.to_string(),
      description: None,
    };
    // cant use tx here coz its already moved.
    let mut rbac_tx = match db_pool.begin().await {
      Ok(rbac_tx) => rbac_tx,
      Err(e) => {
        error!("Could not start transaction {}", e);
        tx.rollback().await;
        return Err(e); //Error out
      }
    };

    match default_rbac_policy.save_tx(tx, &identity).await {
      Ok(_) => {
        debug!("Created rbac policy");
        rbac_tx.commit().await;
        return Ok(new_site);
      }
      Err(e) => {
        error!("Error creating RBAC policy {}", e);
        rbac_tx.rollback().await;
        return Err(e);
      }
    }
  }
}

#[derive(Serialize)]
struct ErrorResponse {
  error_msg: String,
}

#[post("/admin/site")]
pub async fn save(
  identity: web::ReqData<Identity>,
  data: web::Data<AppData>,
  site: web::Json<NewSite>,
) -> HttpResponse {
  info!("Got reqest for creating site {:?}", site.name);
  trace!("Create site request json {:?}", site);
  trace!("Identity {:?}", identity);

  match site.save(identity.into_inner(), &data.db_pool).await {
    Ok(r) => HttpResponse::Created().json(r),
    Err(e) => {
      error!("{}", e);
      let err_response = ErrorResponse {
        error_msg: e.to_string(),
      };
      match &e {
        Error::Database(database_err) => {
          error!("{}", database_err);
          HttpResponse::BadRequest().json(err_response)
        }
        _ => {
          error!("An unknown error has occured {}", e);
          HttpResponse::InternalServerError().json(err_response)
        }
      }
    }
  }
}

use crate::auth::Identity;
use crate::AppData;
use crate::DBPool;
use actix_web::{post, web, HttpResponse};
use chrono::NaiveDateTime;
use log::{debug, error, info, trace};
use serde::{Deserialize, Serialize};
use sqlx::Error;
use uuid::Uuid;
use sqlx::{Postgres, Transaction};
use crate::auth::Authority;
use crate::auth::NewRbacPolicy;
use crate::constants;

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
    //TODO: validate url.

    let url = 

    //Start first transaction

    match db_pool.begin().await {
      Ok(mut tx) => {

        let site_id = uuid::Uuid::new_v4();
        match sqlx::query_as!(
          Site,
          "INSERT INTO site (site_id, name, path, slug, url, cors_enabled, created_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
              RETURNING site_id, name, path, slug, url, cors_enabled, created_by, created, modified",
          site_id,
          &self.name,
          &self.path,
          match &self.slug {
            Some(slug) => slug,
            None => ""
          },
          match &self.url {
            Some(url) => url,
            None => "",
          },
          self.cors_enabled,
          identity.user
        )
        .fetch_one(&mut tx)
        .await {
          Ok(new_site) => {
            // First sql done...
            match db_pool.begin().await {
              Ok(mut nested_tx) => {
                //Create RBAC policy
                let mut site_path = SITE_BASE_PATH.to_owned();
                let user = &identity.user;
                site_path.push_str(&self.name);
                let default_rbac_policy = NewRbacPolicy {
                  path: site_path,
                  path_match: "EXACT".to_string(),
                  method: constants::WILDCARD.to_string(),
                  rbac_user: user.to_string(),
                  rbac_role: constants::WILDCARD.to_string(),
                  description: None,
                };

                match default_rbac_policy.save_tx(nested_tx, &identity).await {
                  Ok(_) => {
                    debug!("Created rbac policy");
                    tx.commit().await;
                    return Ok(new_site)
                  },
                  Err(e) => {
                    error!("Error creating RBAC policy {}", e);
                    tx.rollback().await;
                    return Err(e);
                  }
                }
              },
              Err(e) => {
                error!("Error starting nested transaction");
                tx.rollback().await;
                return Err(e);
              }
            }
          },
          Err(e) => {
            error!("Error creating site {}", e);
            tx.rollback().await;
            return Err(e);
          }
        }
      },
      Err(e) => {
        error!("Error starting outer transaction: {}", e);
        return Err(e);
      }
    };
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

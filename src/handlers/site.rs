use crate::auth::Identity;
use crate::AppData;
use crate::DBPool;
use actix_web::{post, web, HttpResponse};
use chrono::NaiveDateTime;
use log::{error, info, trace};
use serde::{Deserialize, Serialize};
use sqlx::Error;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct NewSite {
  name: String,
  url: Option<String>,
  cors_enabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
  id: Uuid,
  name: String,
  url: Option<String>,
  cors_enabled: Option<bool>,
  created_by: String,
  created: NaiveDateTime,
  modified: NaiveDateTime,
}

impl NewSite {
  pub async fn save(self: &Self, identity: Identity, db_pool: &DBPool) -> Result<Site, Error> {
    //TODO: validate url.

    let url = match &self.url {
      Some(url) => url,
      None => "",
    };

    let id = uuid::Uuid::new_v4();

    sqlx::query_as!(
      Site,
      "INSERT INTO site (id, name, url, cors_enabled, created_by)
        VALUES ($1, $2, $3, $4, $5)
          RETURNING id, name, url, cors_enabled, created_by, created, modified",
      id,
      &self.name,
      url,
      self.cors_enabled,
      identity.user
    )
    .fetch_one(db_pool)
    .await
  }
}

#[derive(Serialize)]
struct ErrorResponse {
  error_msg: String,
}

#[post("/admin/siter")]
pub async fn saver() -> HttpResponse {
  HttpResponse::Ok().finish()
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

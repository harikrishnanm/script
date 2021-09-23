pub mod models;
pub mod site;

use crate::rbac;
use crate::rbac::models::*;
use crate::site::models::*;

use crate::AppData;
use actix_web::{post, web, HttpResponse};

use log::*;
use sqlx::Error;

use crate::error::ErrorResponse;

#[post("/admin/site")]
pub async fn save(
  identity: web::ReqData<Identity>,
  data: web::Data<AppData>,
  new_site: web::Json<NewSite>,
) -> HttpResponse {
  info!("Got reqest for creating site {:?}", new_site.name);
  trace!("Create site request json {:?}", new_site);
  trace!("Identity {:?}", identity);

  match new_site.save(identity.into_inner(), &data.db_pool).await {
    Ok(r) => {
      rbac::reload_rbac(&data).await;
      HttpResponse::Created().json(r)
    }
    Err(e) => {
      error!("{}", e);
      let err_response = ErrorResponse {
        error_message: e.to_string(),
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

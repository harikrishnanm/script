pub mod site;

use crate::rbac::models::*;

use crate::AppData;
use actix_web::{post, web, HttpResponse};

use log::*;
use sqlx::Error;

use crate::error::ErrorResponse;
use crate::rbac;
use site::*;

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

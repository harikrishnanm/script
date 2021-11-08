pub mod app;
pub mod models;

use crate::rbac;
use crate::rbac::models::*;

use crate::app::models::NewApp;
use crate::AppData;
use actix_web::{post, web, HttpResponse};
use log::*;

use crate::error::ScriptError;

#[post("/admin/app")]
pub async fn save(
    identity: web::ReqData<Identity>,
    data: web::Data<AppData>,
    new_app: web::Json<NewApp>,
) -> Result<HttpResponse, ScriptError> {
    info!("Got reqest for creating app {:?}", new_app.name);
    trace!("Create app request json {:?}", new_app);
    trace!("Identity {:?}", identity);

    match new_app.save(identity.into_inner(), &data.data_store).await {
        Ok(r) => {
            match rbac::reload_rbac(&data).await {
                Ok(_) => debug!("RBAC reloaded"),
                Err(e) => {
                    error!("Error reloading rbac {}", e);
                }
            }
            Ok(HttpResponse::Created().json(r))
        }
        Err(e) => Err(e),
    }
}

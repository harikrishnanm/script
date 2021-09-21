use actix_web::{
    get,
    web::{Data, Path, ReqData},
    HttpResponse,
};
use log::*;

use crate::auth::Identity;
use crate::AppData;

#[get("/{site}")]
pub async fn get_site(
    identity: ReqData<Identity>,
    data: Data<AppData>,
    Path(site): Path<String>,
) -> HttpResponse {
    debug!("Get request for {}", site);
    HttpResponse::Ok().finish()
}

use actix_web::{
    get,
    web::{Data, Path, ReqData},
    HttpResponse,
};
use log::*;

use crate::auth::Identity;
use crate::AppData;

#[get("/query/{scope}")]
pub async fn scoped_query(
    identity: ReqData<Identity>,
    data: Data<AppData>,
    Path(scope): Path<String>,
) -> HttpResponse {
    debug!("Get request for {}", scope);
    HttpResponse::Ok().finish()
}

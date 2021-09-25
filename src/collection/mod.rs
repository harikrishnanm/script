pub mod collection;
pub mod models;

use actix_web::{post, web, web::Path, HttpResponse};

use crate::collection::models::NewCollection;
use crate::rbac::models::Identity;
use crate::AppData;

#[post("/site/{site_name}/collection")]
pub async fn save(
    identity: web::ReqData<Identity>,
    data: web::Data<AppData>,
    new_collection: web::Json<NewCollection>,
    Path(site_name): Path<String>,
) -> HttpResponse {
    match new_collection
        .save(site_name, identity.into_inner(), &data.db_pool)
        .await
    {
        Ok(coll) => HttpResponse::Ok().json(coll),
        Err(_e) => HttpResponse::Conflict().finish(),
    }
}

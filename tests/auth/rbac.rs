/*use super::*;
use crate::db;
use crate::rbac;
use crate::test_utils;
use actix_web::middleware::Condition;
use actix_web::{test, web, App};
use dotenv::dotenv;
use env_logger::Env;
use std::sync::Mutex;

#[actix_rt::test]\
async fn spawn_app() -> Result<()> {
    script::run().await
}


/*#[actix_rt::test]
async fn test_rbac_post() {
    debug!("Starting tests");
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    dotenv().ok();
    let db_pool = db::init().await;
    let rbac_result = rbac::load(&db_pool).await.unwrap();
    let app_data = web::Data::new(AppData {
        db_pool: db_pool.clone(),
        rbac: Mutex::new(rbac_result),
        rbac_cache: Mutex::new(HashMap::new()),
    });

    let server = App::new()
        .app_data(app_data.clone())
        .wrap(Condition::new(true, crate::auth::Authenticate))
        .service(save);

    let rbac_policy = NewRbacPolicy {
        path: "/site/admin".to_string(),
        path_match: "EXACT".to_string(),
        method: "DELETE".to_string(),
        rbac_role: "TEST ROLE".to_string(),
        rbac_user: "test_user".to_string(),
        description: None,
    };

    let bearer_token = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJPbmxpbmUgSldUIEJ1aWxkZXIiLCJpYXQiOjE2MzE3MTc3NDksImV4cCI6MTY2MzI1Mzc0OSwiYXVkIjoid3d3LmV4YW1wbGUuY29tIiwic3ViIjoianJvY2tldEBleGFtcGxlLmNvbSIsInVzZXIiOiJjbXNhZG1pbiIsInJvbGVzIjpbIkNNUyBBRE1JTiIsIkFETUlOIl19.jvdHuFS4OXIFFRqllVF7nUTGBeGQFXY6kp2sVQUe284";

    let mut app = test::init_service(server).await;
    let req = test::TestRequest::post()
        .header("content-type", "application/json")
        .header("Authorization", bearer_token)
        .set_json(&rbac_policy)
        .uri("/admin/rbac")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    debug!("Response {:?}", resp);
    assert!(resp.status().is_success());
}*/



pub mod test_utils{

    use super::*;
    use crate::db;
    use crate::rbac;
    use actix_web::{test, web, App};
    use std::sync::Mutex;
    use dotenv::dotenv;
    use env_logger::Env;
    use actix_web::middleware::Condition;
    pub type TestServer =actix_web::App<impl actix_service::ServiceFactory, actix_web::dev::Body>;
    pub async fn get_test_server() -> TestServer {
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
        server
    }
}
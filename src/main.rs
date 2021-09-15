use actix_web::{
    middleware::{Compress, Condition, Logger},
    web, App, HttpServer,
};
use dotenv::dotenv;
use env_logger::Env;
use log::{debug, info};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::{io::Error, result::Result};

mod auth;
mod config;
mod db;
mod handlers;

use crate::auth::{rbac, RbacPolicy};

use crate::handlers::*;

pub type DBPool = sqlx::Pool<sqlx::Postgres>;
pub type RbacPolicySet = HashSet<RbacPolicy>;

pub struct AppData {
    db_pool: DBPool,
    rbac: Mutex<RbacPolicySet>,
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    info!("Starting SCRIPT api service");
    debug!("Reading configuration variables");
    let addr = config::get_server_address();
    let workers = config::get_worker_count();

    //let enable_auth = std::env::var("ENABLE_AUTH") == Ok("true".into());

    info!("Server Address: {}", &addr);
    info!("Worker threads: {}", &workers);

    info!("Connecting to the database");
    let db_pool = db::init().await;
    info!("Connected to the DB");

    info!("Reading RBAC");

    let rbac_result = rbac::init(&db_pool).await;

    let app_data = web::Data::new(AppData {
        db_pool: db_pool.clone(),
        rbac: Mutex::new(rbac_result.clone()),
    });
    info!("Starting app server workers");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Condition::new(true, auth::Authenticate))
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(site::save)
            .service(site::saver)
    })
    .workers(workers)
    .bind(addr)?
    .run()
    .await
}
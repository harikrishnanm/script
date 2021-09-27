use actix_web::{
    middleware::{Compress, Condition, Logger},
    web, App, HttpServer,
};
use dotenv::dotenv;
use env_logger::Env;
use log::{debug, info};
use std::collections::HashMap;
use std::sync::Mutex;
use std::{io::Error, result::Result};

mod collection;
mod config;
mod constants;
mod db;
mod error;
mod file;
mod rbac;
mod site;
mod content;
mod asset;
mod common;

pub type DBPool = sqlx::Pool<sqlx::Postgres>;

use crate::rbac::models::*;

pub struct AppData {
    db_pool: DBPool,
    rbac: Mutex<Rbac>,
    rbac_cache: Mutex<HashMap<String, bool>>,
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

    let rbac_result = rbac::load(&db_pool).await.unwrap();

    let app_data = web::Data::new(AppData {
        db_pool: db_pool.clone(),
        rbac: Mutex::new(rbac_result),
        rbac_cache: Mutex::new(HashMap::new()),
    });
    info!("Starting app server workers");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Condition::new(true, Authenticate))
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(rbac::save)
            .service(rbac::update)
            .service(rbac::delete)
            .service(rbac::get_all)
            .service(site::save)
            .service(file::upload)
            .service(file::list)
            .service(file::get_file)
            .service(collection::save)
            .service(collection::get)
            .service(content::save)
            .service(content::get_text)
            .service(content::update)
    })
    .workers(workers)
    .bind(addr)?
    .run()
    .await
}

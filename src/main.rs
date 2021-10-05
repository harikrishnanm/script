use actix_web::web::JsonConfig;
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

mod asset;
mod collection;
mod common;
mod config;
mod constants;
mod content;
mod db;
mod error;
mod file;
mod folder;
mod rbac;
mod redis;
mod site;
mod taxonomy;

pub type DBPool = sqlx::Pool<sqlx::Postgres>;
pub type RedisPool = r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>;
pub type RedisConnection = r2d2_redis::r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>;

use crate::rbac::models::*;

pub struct AppData {
    db_pool: DBPool,
    redis_pool: RedisPool,
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
    let json_config = JsonConfig::default().limit(128000usize);

    let redis_pool = redis::init();

    let app_data = web::Data::new(AppData {
        db_pool: db_pool.clone(),
        redis_pool: redis_pool,
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
            .service(content::get)
            .service(content::update)
            .service(folder::create_root)
            .service(folder::create)
            .service(folder::get)
            .service(folder::get_root)
            .service(asset::save)
            .service(taxonomy::save)
            .service(taxonomy::save_item)
            .service(taxonomy::get)
    })
    .workers(workers)
    .bind(addr)?
    .run()
    .await
}

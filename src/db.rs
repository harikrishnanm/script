use core::time::Duration;
use log::{error, info, trace};
use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::DBPool;

pub async fn init() -> DBPool {
    info!("Initializing DB");

    let db_url = &env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    trace!("DB URL {}", db_url);
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .min_connections(2)
        .connect_timeout(Duration::new(5, 0))
        .test_before_acquire(true)
        .connect(db_url)
        .await
    {
        Ok(pool) => {
            info!("Checking pending db migrations");
            match sqlx::migrate!("./migrations/").run(&pool).await {
                Ok(_) => info!("Migrations completed"),
                Err(e) => panic!("Error running migrations {}", e),
            }
            pool
        }
        Err(err) => {
            error!("Error initializing pool {:?}", err);
            panic!("Could not start pool")
        }
    };
    pool
}

use log::*;
use r2d2_redis::{r2d2, redis, RedisConnectionManager};
use std::env;

use crate::{RedisConnection, RedisPool};
use std::ops::DerefMut;

pub fn init() -> RedisPool {
    info!("Initializing redis pool");
    let _redis_url = match env::var("REDIS_URL") {
        Ok(url) => url,
        Err(_e) => "redis://localhost".to_string(),
    };
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool: RedisPool = r2d2::Pool::builder()
        .max_size(20u32)
        .min_idle(Some(5u32))
        .build(manager)
        .unwrap();
    let mut conn: RedisConnection = pool.clone().get().unwrap();

    info!("Testing redis connection..");
    let reply = redis::cmd("PING")
        .query::<String>(conn.deref_mut())
        .unwrap();
    match "PONG" == reply {
        true => debug!("Redis pool initialized"),
        false => error!("Error connecting to redis"),
    }
    pool
}

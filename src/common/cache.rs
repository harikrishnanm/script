use crate::{RedisConnection, RedisPool};
use r2d2_redis::redis;
use r2d2_redis::redis::{FromRedisValue, ToRedisArgs};
use std::ops::DerefMut;

use log::*;

pub fn put<T: ToRedisArgs>(cache_pool: &RedisPool, cache_key: &str, value: T) {
    let mut cache_conn: RedisConnection = cache_pool.get().unwrap();
    match redis::cmd("SET")
        .arg(cache_key)
        .arg(value)
        .query::<String>(cache_conn.deref_mut())
    {
        Ok(_) => debug!("Added to redis cache"),
        Err(e) => error!("Error addingh to cache {}", e),
    }
}

pub fn get<T: FromRedisValue>(cache_pool: &RedisPool, cache_key: &str) -> Option<T> {
    let mut cache_conn: RedisConnection = cache_pool.get().unwrap();
    match redis::cmd("GET")
        .arg(cache_key)
        .query::<T>(cache_conn.deref_mut())
    {
        Ok(result) => Some(result),
        Err(_e) => None,
    }
}

pub fn delete(cache_pool: &RedisPool, cache_key: &str) {
    let mut cache_conn: RedisConnection = cache_pool.get().unwrap();
    match redis::cmd("DEL")
        .arg(cache_key)
        .query::<i32>(cache_conn.deref_mut())
    {
        Ok(_) => debug!("Cleared cache"),
        Err(e) => error!("Error clearing cache {}", e),
    }
}

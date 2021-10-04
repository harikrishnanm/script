use log::{debug, info};
//use num_cpus;
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

pub fn get_server_address() -> SocketAddr {
    let default_address = IpAddr::from_str("0.0.0.0").unwrap();
    let default_port = 8088;

    let mut address: IpAddr = default_address;
    let mut port: u16 = default_port;

    match env::var("BIND_ADDRESS") {
        Ok(val) => {
            address = IpAddr::from_str(&val).unwrap();
            info!("Setting address to {:?}", address);
        }
        Err(_e_) => info!(
            "No BIND_ADDRESS environment variable set. Using default {}",
            address
        ),
    }

    match env::var("BIND_PORT") {
        Ok(val) => {
            port = u16::from_str(&val).unwrap();
            info!("Setting port to {}", port);
        }
        Err(_e) => info!(
            "No BIND_PORT environment variable set. Using default {}",
            default_port
        ),
    };
    debug!("Socket address: {:?} port {}", address, port);
    SocketAddr::new(address, port)
}

pub fn get_worker_count() -> usize {
    let mut workers = 8;
    match env::var("WORKERS") {
        Ok(val) => {
            workers = usize::from_str(&val).unwrap();
            info!("Setting worker count to {}", workers);
        }
        Err(_e) => info!("No WORKERS environment variable. Using default {}", workers),
    };
    workers
}

pub fn get_cache_size() -> usize {
    let mut cache_size = 1000000;
    match env::var("CACHE_SIZE") {
        Ok(val) => {
            cache_size = usize::from_str(&val).unwrap();
            info!("Setting cache size to {}", cache_size);
        }
        Err(_e) => info!(
            "No CACHE_SIZE environment variable. Using default {}",
            cache_size
        ),
    };
    cache_size
}

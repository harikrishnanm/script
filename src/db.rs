use crate::error::ScriptError;
use crate::DataStore;
use log::*;
use mongodb::Client;
use std::env;

pub async fn init() -> Result<DataStore, ScriptError> {
    info!("Initializing DB");

    let db_url = &env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    trace!("DB URL {}", db_url);

    let client = Client::with_uri_str(db_url).await.unwrap();
    let database = client.database("script");
    Ok(DataStore {
        client: client,
        db: database,
    })
}

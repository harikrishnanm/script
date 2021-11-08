use crate::error::ScriptError;
use crate::DataStore;
use config::*;
use log::*;
use mongodb::bson::doc;
use mongodb::{
    options::{ClientOptions, IndexOptions},
    Client, Database, IndexModel,
};
use std::collections::HashMap;
use std::env;
use std::vec::Vec;

pub async fn init() -> Result<DataStore, ScriptError> {
    info!("Initializing DB");
    let mut settings = Config::default();
    settings.merge(File::with_name("conf/db.toml")).unwrap();
    let db_url = settings
        .get_str("server.db_url")
        .expect("    URL is not configured");

    //let db_url = &env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    trace!("DB URL {}", db_url);

    let mut collections = settings.get_array("collection").unwrap();
    trace!("Collections: {:?}", collections);

    for (_, collection) in collections.iter_mut().enumerate() {
        match collection.clone().into_table() {
            Ok(coll) => {
                let coll_name = match coll.get("name") {
                    Some(coll_name_value) => match coll_name_value.clone().into_str() {
                        Ok(coll_name) => coll_name,
                        Err(e) => {
                            panic!("Configuration error {}", e);
                        }
                    },
                    None => {
                        panic!("Collection name is null or incorrect. Configuration error");
                    }
                };
                trace!("Processing Collection {}", coll_name);
                match coll.get("index") {
                    Some(index_values) => {
                        trace!("Index found {:?}", index_values);
                        for index_value in index_values.clone().into_array() {
                            debug!("Processing index {:?}", index_value);
                            let fields = index_value.get_array("fields");
                        }
                    }
                    None => {
                        info!("No index found for {}", coll_name);
                    }
                }
            }
            Err(e) => {
                error!("Error reading db configuration {}", e);
                panic!("Cannot continue due a DB misconfiguration");
            }
        }
    }

    //let client = Client::with_uri_str(db_url).await.unwrap();
    let client_options = ClientOptions::parse(db_url).await.unwrap();
    trace!("Client options {:?}", client_options);
    let client = Client::with_options(client_options).unwrap();
    let database = client.database("script");

    create_app_indexes(&database).await;

    Ok(DataStore {
        client: client,
        db: database,
    })
}

async fn create_app_indexes(database: &Database) {
    info!("Creating indexes for APP collection");
    let coll = database.collection::<crate::app::models::App>("APP");
    let mut app_coll_idx_models: Vec<IndexModel> = Vec::new();

    let index_names = coll.list_index_names().await.unwrap();
    let app_name_idx_name = "app_name_idx".to_string();
    let app_path_idx_name = "app_path_idx".to_string();

    if !index_names.contains(&app_name_idx_name) {
        let idx = doc! {"name": 1};
        let idx_options = IndexOptions::builder()
            .unique(true)
            .name(app_name_idx_name)
            .build();

        let name_idx_model = IndexModel::builder().keys(idx).options(idx_options).build();
        app_coll_idx_models.push(name_idx_model);
    } else {
    }

    if !index_names.contains(&app_path_idx_name) {
        let idx = doc! {"path": 1};
        let idx_options = IndexOptions::builder()
            .unique(true)
            .name(app_path_idx_name)
            .build();

        let path_idx_model = IndexModel::builder().keys(idx).options(idx_options).build();
        app_coll_idx_models.push(path_idx_model);
    }

    match coll.drop_indexes(None).await {
        Ok(_) => match coll.create_indexes(app_coll_idx_models, None).await {
            Ok(r) => info!("Created/updated indexes {:?}", r),
            Err(e) => {
                error!("Error creating indexes {}", e);
            }
        },
        Err(e) => {
            error!("Error updating indexes");
        }
    }
}

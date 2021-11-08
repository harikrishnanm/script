use crate::constants;
use crate::error::*;
use crate::rbac::models::RbacPolicyRequest;
use crate::DataStore;
use chrono::offset::Utc;
use mongodb::{
    bson::doc,
    error::{TRANSIENT_TRANSACTION_ERROR, UNKNOWN_TRANSACTION_COMMIT_RESULT},
    options::{Acknowledgment, ReadConcern, TransactionOptions, WriteConcern},
    ClientSession, Collection,
};

use log::*;

use crate::rbac::models::*;
//const OWNER: &str = "O";
const SITE_BASE_PATH: &str = "/app/";

use crate::app::models::*;

impl NewApp {
    pub async fn save(
        self: &Self,
        identity: Identity,
        data_store: &DataStore,
    ) -> Result<App, ScriptError> {
        //Create client session
        let user = identity.user.clone();
        let client = &data_store.client;
        let mut session = match client.start_session(None).await {
            Ok(session) => session,
            Err(_e) => {
                return Err(ScriptError::TransactionError);
            }
        };

        let options = TransactionOptions::builder()
            .read_concern(ReadConcern::majority())
            .write_concern(WriteConcern::builder().w(Acknowledgment::Majority).build())
            .build();

        match session.start_transaction(options).await {
            Ok(_) => debug!("Transaction started"),
            Err(e) => {
                error!("Error starting transaction {}", e);
                return Err(ScriptError::TransactionError);
            }
        }

        let app_id = uuid::Uuid::new_v4();
        let app_coll = &data_store.db.collection::<App>("APP");

        let app = App {
            app_id: app_id,
            name: self.name.clone(),
            path: self.path.clone(),
            url: self.url.clone(),
            slug: self.slug.clone(),
            cors_enabled: self.cors_enabled,
            created_by: identity.user.clone(),
            created: Utc::now().naive_utc(),
            modified: None,
        };

        match app_coll
            .insert_one_with_session(&app, None, &mut session)
            .await
        {
            Ok(result) => debug!("{:?}", result),
            Err(e) => {
                error!("Error writing app document {}", e);
                return Err(ScriptError::TransactionError);
            }
        }

        let mut site_path = SITE_BASE_PATH.to_owned();
        site_path.push_str(&self.name);

        // Create new RBAC Policy
        let default_rbac_policy = RbacPolicyRequest {
            path: site_path,
            path_match: constants::STARTSWITH.to_string(),
            method: constants::WILDCARD.to_string(),
            rbac_user: user.to_string(),
            rbac_role: constants::WILDCARD.to_string(),
            description: None,
        };

        match session.commit_transaction().await {
            Ok(_) => info!("Transaction commited"),
            Err(e) => error!("Error commiting transaction {}", e),
        }

        Ok(app.clone())
    }
}

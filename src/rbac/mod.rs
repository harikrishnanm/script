pub mod middleware;
pub mod models;
pub mod rbac;
pub mod utils;
pub mod validators;

use actix_web::{
    delete, get, post, put, web,
    web::{Data, Path, ReqData},
    HttpResponse,
};
use actix_web_validator::Json;
use log::*;
use mongodb::{error::Error, Client};
use regex::RegexSet;
use std::collections::HashMap;
use uuid::Uuid;

use crate::constants;
use crate::error::ScriptError;
use crate::AppData;
use crate::DataStore;
use bson::document::Document;
use chrono::offset::Utc;
use futures::TryStreamExt;
use models::*;
use mongodb::options::IndexOptions;
use mongodb::IndexModel;

#[get("/admin/rbac/{rbac_id}")]
pub async fn get_rbac_by_id(
    _identity: ReqData<Identity>,
    _data: Data<AppData>,
    Path(_rbac_id): Path<Uuid>,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/admin/rbac")]
pub async fn get_all(
    identity: ReqData<Identity>,
    data: Data<AppData>,
) -> Result<HttpResponse, ScriptError> {
    debug!("Getting all rbac entires for {}", identity.user);
    let rbac_coll = data.data_store.db.collection::<RbacPolicy>("RBAC");
    match rbac_coll.find(None, None).await {
        Ok(cursor) => {
            let result: Result<Vec<RbacPolicy>, _> = cursor.try_collect().await;
            match result {
                Ok(r) => Ok(HttpResponse::Ok().json(r)),
                Err(e) => {
                    error!("Error getting RBAC policies {}", e);
                    Err(ScriptError::UnexpectedError)
                }
            }
        }
        Err(e) => Err(ScriptError::UnexpectedError),
    }
}
/*
#[delete("/admin/rbac/{rbac_id}")]
pub async fn delete(
    identity: ReqData<Identity>,
    data: Data<AppData>,
    Path(rbac_id): Path<Uuid>,
) -> HttpResponse {
    debug!(
        "Get request to delete rbac policy {} for {}",
        rbac_id, identity.user
    );

    let db_pool = &data.db_pool;

    let rows_deleted = sqlx::query("DELETE FROM rbac WHERE rbac_id = $1")
        .bind(rbac_id)
        .execute(db_pool)
        .await
        .unwrap()
        .rows_affected();

    match rows_deleted {
        0 => HttpResponse::InternalServerError().finish(),
        _ => {
            match reload_rbac(&data).await {
                Ok(_) => debug!("RBAC reloaded"),
                Err(e) => {
                    error!("Error reloading rbac {}", e);
                }
            }
            HttpResponse::Ok().finish()
        }
    }
}

#[put("/admin/rbac")]
pub async fn update(
    data: Data<AppData>,
    rbac_policy_req: Json<RbacPolicyRequest>,
    identity: web::ReqData<Identity>,
) -> HttpResponse {
    info!("Updating RBAC policy");

    match RbacPolicy::from_req_for_update(&rbac_policy_req.into_inner(), &identity.into_inner())
        .update(&data.db_pool)
        .await
    {
        Ok(rbac_policy) => {
            match reload_rbac(&data).await {
                Ok(_) => debug!("RBAC reloaded"),
                Err(e) => {
                    error!("Error reloading rbac {}", e);
                }
            }
            HttpResponse::Ok().json(rbac_policy)
        }
        Err(e) => {
            error!("Error updating policy {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
*/
#[post("/admin/rbac")]
pub async fn save(
    data: Data<AppData>,
    rbac_policy: Json<RbacPolicyRequest>,
    identity: web::ReqData<Identity>,
) -> Result<HttpResponse, ScriptError> {
    info!("Creating new RBAC policy");

    let data_store = &data.data_store;

    match rbac_policy.save(data_store, &identity.into_inner()).await {
        Ok(rbac_policy) => {
            info!("Saved data. Reloading RBAC rules");
            //Refresh the cache
            match reload_rbac(&data).await {
                Ok(_) => debug!("RBAC reloaded"),
                Err(e) => {
                    error!("Error reloading rbac {}", e);
                }
            }
            Ok(HttpResponse::Created().json(rbac_policy))
        }
        Err(e) => {
            error!("Error creating rbac policy {}", e);
            Err(e)
        }
    }
}

///Reload policies
///
pub async fn reload_rbac(data: &AppData) -> Result<(), Error> {
    debug!("Reloading rbac policies");
    match load(&data.data_store).await {
        Ok(rbac) => {
            debug!("Loaded new RBAC set");
            *data.rbac.lock().unwrap() = rbac;
            Ok(())
        }
        Err(e) => {
            error!("Error refreshing data {}", e);
            Err(e)
        }
    }
}

/// Load the rbac policies from the DB and update the Arc carrying the policies.

pub async fn load(data_store: &DataStore) -> Result<Rbac, Error> {
    debug!("Loading RBAC policies");

    let db = &data_store.db;
    let rbac_coll = db.collection::<RbacPolicy>("RBAC");

    let cursor = match rbac_coll.find(None, None).await {
        Ok(cursor) => cursor,
        Err(e) => {
            error!("Error loading RBAC {}", e);
            return Err(e);
        }
    };

    let mut path_regex: Vec<String> = Vec::new();
    let mut methods: HashMap<usize, Vec<String>> = HashMap::new();
    let mut users: HashMap<usize, Vec<String>> = HashMap::new();
    let mut roles: HashMap<usize, Vec<String>> = HashMap::new();
    let mut public_paths: Vec<String> = Vec::new();
    let mut idx: usize = 0;

    let mut docs: Vec<RbacPolicy> = cursor.try_collect().await?;
    if docs.len() == 0 {
        debug!("No RBAC policy found. Creating default policy");
        let uuid = Uuid::new_v4().to_string();
        let created_at = Utc::now().naive_utc();
        let default = RbacPolicy {
            rbac_id: uuid,
            path: "/admin".to_string(),
            path_match: "STARTSWITH".to_string(),
            method: "*".to_string(),
            rbac_role: "CMS ADMIN".to_string(),
            rbac_user: "cmsadmin".to_string(),
            description: "Allow CMS Admin access to /admin/* routes".to_string(),
            created_at: created_at,
            created_by: "Yoda".to_string(),
            modified_by: None,
            modified_at: None,
        };
        rbac_coll.insert_one(&default, None).await?;

        let mut index_doc = Document::new();
        index_doc.insert("path", 1);
        index_doc.insert("path_match", 1);
        index_doc.insert("method", 1);
        index_doc.insert("rbac_user", 1);
        index_doc.insert("rbac_role", 1);
        let index_options = IndexOptions::builder()
            .unique(Some(true))
            .name(Some("rbac_uniq_idx".to_string()))
            .build();

        let index_model = IndexModel::builder()
            .keys(index_doc)
            .options(index_options)
            .build();
        rbac_coll.create_index(index_model, None).await?;
        docs.push(default);
    }

    for doc in docs {
        let path = doc.path;
        let method = doc.method;
        let user = doc.rbac_user;
        let role = doc.rbac_role;
        let path_match = doc.path_match;

        if user == constants::WILDCARD && role == constants::WILDCARD && method == "GET" {
            let pub_path_str = format!(
                "{}{}{}",
                constants::REGEX_PREFIX,
                path.clone(),
                constants::REGEX_STARTSWITH_SUFFIX
            );
            debug!("Public path regex {}", pub_path_str);
            public_paths.push(pub_path_str);
        }

        let mut regex_str = constants::REGEX_PREFIX.to_string();
        let _ = &regex_str.push_str(&path);
        if path_match == constants::EXACT {
            let _ = &regex_str.push_str(constants::REGEX_EXACT_SUFFIX);
        } else if path_match == constants::STARTSWITH {
            let _ = &regex_str.push_str(constants::REGEX_STARTSWITH_SUFFIX);
        }

        debug!("Adding {} {} {} {} to set", regex_str, method, user, role);
        //Check if the pattern is already added.
        match path_regex.iter().position(|pattern| pattern.eq(&regex_str)) {
            None => {
                trace!("No exis ting path pattern found...");
                path_regex.push(regex_str.to_string());
                methods.insert(idx, vec![method]);
                users.insert(idx, vec![user]);
                roles.insert(idx, vec![role]);
                idx += 1;
            }
            Some(existing_idx) => {
                trace!("Path pattern found at {}.", existing_idx);
                let methods_vec = methods.get_mut(&existing_idx).unwrap();
                if !methods_vec.contains(&method) {
                    methods_vec.push(method);
                }

                let users_vec = users.get_mut(&existing_idx).unwrap();
                if !users_vec.contains(&user) {
                    users_vec.push(user);
                }

                let roles_vec = roles.get_mut(&existing_idx).unwrap();
                if !roles_vec.contains(&role) {
                    roles_vec.push(role);
                }
            }
        }
    }

    trace!("Path_regex_str vector  {:?}", path_regex);
    trace!("Methods hashmap {:?}", methods);
    trace!("Users hashmap {:?}", users);
    trace!("Roles hashmap {:?}", roles);
    trace!("Public paths vector {:?}", public_paths);

    let path_regex_set = RegexSet::new(path_regex).unwrap();
    let pub_paths_regex_set = RegexSet::new(public_paths).unwrap();
    Ok(Rbac {
        path_regex_set: path_regex_set,
        methods: methods,
        users: users,
        roles: roles,
        pub_paths_regex_set: pub_paths_regex_set,
    })
}

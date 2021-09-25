pub mod middleware;
pub mod models;
pub mod rbac;
pub mod utils;
pub mod validators;

use crate::DBPool;
use actix_web::{
  delete, get, post, put, web,
  web::{Data, Path, ReqData},
  HttpResponse,
};
use actix_web_validator::Json;
use log::*;
use regex::RegexSet;
use sqlx::Error;
use std::collections::HashMap;
use uuid::Uuid;

use crate::constants;
use crate::error::ScriptError;
use crate::AppData;
use models::*;

pub async fn reload_rbac(data: &AppData) -> Result<(), Error> {
  debug!("Reloading rbac policies");
  match load(&data.db_pool).await {
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

#[get("/admin/rbac/{rbac_id}")]
pub async fn get_rbac_by_id(
  identity: ReqData<Identity>,
  data: Data<AppData>,
  Path(rbac_id): Path<Uuid>,
) -> HttpResponse {
  HttpResponse::Ok().finish()
}

#[get("/admin/rbac")]
pub async fn get_all(identity: ReqData<Identity>, data: Data<AppData>) -> HttpResponse {
  debug!("Getting all rbac entires");
  let result  = match sqlx::query_as!(RbacPolicy, "SELECT rbac_id, path, path_match, method, rbac_role, rbac_user, description, modified, modified_by FROM rbac").fetch_all(&data.db_pool).await {
    Ok(result) => result,
    Err(e) => {
      error!("Error {}", e);
      return HttpResponse::InternalServerError().finish();
    }
  };
  HttpResponse::Ok().json(result)
}

#[delete("/admin/rbac/{rbac_id}")]
pub async fn delete(
  identity: ReqData<Identity>,
  data: Data<AppData>,
  Path(rbac_id): Path<Uuid>,
) -> HttpResponse {
  debug!("Get request to delete rbac policy {}", rbac_id);

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
      reload_rbac(&data).await;
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
      reload_rbac(&data).await;
      HttpResponse::Ok().json(rbac_policy)
    }
    Err(e) => {
      error!("Error updating policy {}", e);
      HttpResponse::InternalServerError().finish()
    }
  }
}

#[post("/rbac")]
pub async fn save(
  data: Data<AppData>,
  rbac_policy: Json<NewRbacPolicy>,
  identity: web::ReqData<Identity>,
) -> Result<HttpResponse, ScriptError> {
  info!("Creating new RBAC policy");
  match rbac_policy
    .save(&data.db_pool, &identity.into_inner())
    .await
  {
    Ok(rbac_policy) => {
      info!("Saved data");
      //Refresh the cache
      reload_rbac(&data).await;
      Ok(HttpResponse::Created().json(rbac_policy))
    }
    Err(e) => {
      error!("Error creating rbac policy {}", e);
      match e {
        Error::Database(fields) => Err(ScriptError::RbacCreationConflict(
          "Check params".to_string(),
        )),
        _ => Err(ScriptError::UnexpectedRbacCreationFailure),
      }
    }
  }
}

pub async fn load(db_pool: &DBPool) -> Result<Rbac, Error> {
  let rows =
    sqlx::query!("SELECT rbac_id, path, path_match, method, rbac_role, rbac_user FROM rbac")
      .fetch_all(db_pool)
      .await?;
  let mut path_regex: Vec<String> = Vec::new();
  let mut methods: HashMap<usize, Vec<String>> = HashMap::new();
  let mut users: HashMap<usize, Vec<String>> = HashMap::new();
  let mut roles: HashMap<usize, Vec<String>> = HashMap::new();
  let mut idx: usize = 0;

  for row in rows {
    let path = row.path;
    let method = row.method;
    let user = row.rbac_user;
    let role = row.rbac_role;
    let path_match = row.path_match;

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
        trace!("No exiting path pattern found...");
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

  let path_regex_set = RegexSet::new(path_regex).unwrap();
  Ok(Rbac {
    path_regex_set: path_regex_set,
    methods: methods,
    users: users,
    roles: roles,
  })
}

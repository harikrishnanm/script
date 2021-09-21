use crate::DBPool;
use actix_web::{
  post, web,
  web::{Data},
  HttpResponse,
};
use actix_web_validator::Json;
use log::*;
use regex::RegexSet;

use sqlx::Error;
use std::collections::HashMap;


use crate::auth::{Identity, Rbac, NewRbacPolicy};
use crate::constants;
use crate::AppData;

/*
Step1: Match the path regex
Step2: Match Method
If there is a match then
Step3: Match user
Step4: check if the role vectors have joint elements
*/


#[post("/admin/rbac")]
pub async fn save(
  data: Data<AppData>,
  rbac_policy: Json<NewRbacPolicy>,
  identity: web::ReqData<Identity>,
) -> HttpResponse {
  info!("Creating new RBAC policy");
  match rbac_policy.save(&data.db_pool, &identity.into_inner()).await {
    Ok(_) => {
      info!("Saved data");
      //Refresh the cache
      match load(&data.db_pool).await {
        Ok(rbac) => {
          *data.rbac.lock().unwrap() = rbac;
        }
        Err(e) => {
          error!("Error refreshing data {}", e);
        }
      };
      HttpResponse::Created().finish()
    }
    Err(e) => {
      error!("Error creating rbac policy {}", e);
      HttpResponse::Conflict().finish()
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

    debug!("Adding {} to set", regex_str);
    //Check if the pattern is already added.
    match path_regex.iter().position(|pattern| pattern.eq(&regex_str)) {
      None => {
        debug!("No exiting string found...");
        path_regex.push(regex_str.to_string());
        methods.insert(idx, vec![method]);
        users.insert(idx, vec![user]);
        roles.insert(idx, vec![role]);
        idx += 1;
      }
      Some(existing_idx) => {
        debug!("String found at {}.", existing_idx);
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

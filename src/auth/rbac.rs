use crate::DBPool;
use actix_web::{
  post,
  web::{Data, Json},
  HttpResponse,
};
use futures::TryStreamExt;
use log::{debug, error, info, trace};
use regex::RegexSet;
use serde::Deserialize;
use sqlx::{Error, Row};
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::iter::FromIterator;
use std::sync::Arc;

use crate::auth::Rbac;
use crate::AppData;
/*
Step1: Match the path regex
Step2: Match Method
If there is a match then
Step3: Match user
Step4: check if the role vectors have joint elements
*/
#[derive(Deserialize, Debug)]
pub enum PathMatch {
  STARTSWITH(String),
  EXACT(String),
}

#[derive(Deserialize, Debug)]
pub struct NewRbacPolicy {
  path: String,
  path_match: String,
  method: String,
  rbac_role: String,
  rbac_user: String,
  description: Option<String>,
}

/*impl NewRbacPolicy {
  pub async fn save(self: &Self, db_pool: &DBPool) -> Result<(), Error> {
    debug!("{:?}", self);
    sqlx::query_as!(
    RbacPolicy,
    "INSERT INTO rbac(path, path_match, method, rbac_role, rbac_user, description, created_by, created, modified)
      VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, url, cors_enabled, created_by, created, modified",
    &self.name,
    url,
    self.cors_enabled,
    identity.user
  )
  .fetch_one(db_pool)
  .await
  }
}*/

pub async fn init(db_pool: &DBPool) -> Result<Rbac, Error> {
  let rows = sqlx::query!("SELECT rbac_id, path_regex, method, rbac_role, rbac_user FROM rbac")
    .fetch_all(db_pool)
    .await?;
  let mut path_regex: Vec<String> = Vec::new();
  let mut methods: HashMap<usize, Vec<String>> = HashMap::new();
  let mut users: HashMap<usize, Vec<String>> = HashMap::new();
  let mut roles: HashMap<usize, Vec<String>> = HashMap::new();
  let mut idx: usize = 0;
  for row in rows {
    let regex_str = row.path_regex;
    let method = row.method;
    let user = row.rbac_user;
    let role = row.rbac_role;
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

#[post("/admin/rbac")]
pub async fn save(data: Data<AppData>, rbac_policy: Json<NewRbacPolicy>) -> HttpResponse {
  /*match rbac_policy.save(&data.db_pool).await {
    Ok(_) => {
      info!("Saved data");
    }
    Err(e) => {
      error!("Error creating rbac policy");
    }
  };*/

  HttpResponse::Ok().finish()
}

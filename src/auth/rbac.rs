use crate::DBPool;
use actix_web::{
  post, web,
  web::{Data},
  HttpResponse,
};
use actix_web_validator::Json;
use log::{debug, error, info, trace};
use regex::RegexSet;
use serde::Deserialize;
use sqlx::Error;
use std::collections::HashMap;
use validator::Validate;

use crate::auth::{Identity, Rbac};
use crate::constants;
use crate::AppData;
use crate::validators::*;
/*
Step1: Match the path regex
Step2: Match Method
If there is a match then
Step3: Match user
Step4: check if the role vectors have joint elements
*/
#[derive(Deserialize, Debug, Validate)]
pub struct NewRbacPolicy {
  #[validate (length(min = 1, max = 25))]
  path: String,
  #[validate (custom = "validate_path_match")]
  path_match: String,
  #[validate (custom = "validate_method_match")]
  method: String,
  rbac_role: String,
  rbac_user: String,
  #[validate (length(max = 100))]
  description: Option<String>,
}

impl NewRbacPolicy {

  pub fn new(path: &str, path_match: &str, method: &str, rbac_role: &str, rbac_user: &str, description: Option<&str>) -> Self{
    debug!("Constructing new rbac policy struct");
    Self{
      path: path.to_string(),
      path_match: path_match.to_string(),
      method: method.to_string(),
      rbac_role: rbac_role.to_string(),
      rbac_user: rbac_user.to_string(),
      description: match description {
        Some(desc) => Some(desc.to_string()),
        None => None
      }
    }
  }

  pub async fn save(self: &Self, db_pool: &DBPool, identity: &Identity) -> Result<(), Error> {
    debug!("{:?}", self);

    let description = match &self.description {
      Some(desc) => desc,
      None => "",
    };

    match sqlx::query!(
      "INSERT INTO rbac(path, path_match, method, rbac_role, rbac_user, description, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7)",
      &self.path,
      &self.path_match,
      &self.method,
      &self.rbac_role,
      &self.rbac_user,
      &description,
      identity.user
    )
    .execute(db_pool)
    .await
    {
      Ok(_) => Ok(()),
      Err(e) => Err(e),
    }
  }
}

#[post("/admin/rbac")]
pub async fn save(
  data: Data<AppData>,
  rbac_policy: Json<NewRbacPolicy>,
  identity: web::ReqData<Identity>,
) -> HttpResponse {
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

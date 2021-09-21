use crate::validators::*;
use crate::DBPool;
use log::*;
use regex::RegexSet;
use serde::{Deserialize, Serialize};
use sqlx::{Error, Postgres, Transaction};
use std::collections::HashMap;
use std::hash::Hash;
use validator::Validate;

pub mod middleware;
pub mod rbac;
pub mod utils;

pub struct Authenticate;

#[derive(Debug, Clone)]
pub struct Identity {
  pub user: String,
  pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub iat: i64,
  pub exp: i64,
  pub user: String,
  pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthError {
  err_type: String,
  err_msg: String,
}
#[derive(Debug)]
pub struct Rbac {
  path_regex_set: RegexSet,
  methods: HashMap<usize, Vec<String>>,
  users: HashMap<usize, Vec<String>>,
  roles: HashMap<usize, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash)]
pub struct RbacParams {
  path: String,
  method: String,
  rbac_role: Vec<String>,
  rbac_user: String,
}

impl RbacParams {
  fn hash(self: &Self) -> String {
    use fasthash::sea;
    let mut buf = String::from(&self.path);
    buf.push_str(&self.method);
    buf.push_str(&self.rbac_role.join(""));
    buf.push_str(&self.rbac_user);
    sea::hash64(&buf.into_bytes()).to_string()
  }
}

#[derive(Deserialize, Debug)]
pub struct Authority {
  pub user: String,
  pub authority: String,
}

#[derive(Deserialize, Debug, Validate)]
pub struct NewRbacPolicy {
  #[validate(length(min = 1, max = 25))]
  pub path: String,
  #[validate(custom = "validate_path_match")]
  pub path_match: String,
  #[validate(custom = "validate_method_match")]
  pub method: String,
  pub rbac_role: String,
  pub rbac_user: String,
  #[validate(length(max = 100))]
  pub description: Option<String>,
}

impl NewRbacPolicy {
  pub fn new(
    path: &str,
    path_match: &str,
    method: &str,
    rbac_role: &str,
    rbac_user: &str,
    description: Option<&str>,
  ) -> Self {
    debug!("Constructing new rbac policy struct");
    Self {
      path: path.to_string(),
      path_match: path_match.to_string(),
      method: method.to_string(),
      rbac_role: rbac_role.to_string(),
      rbac_user: rbac_user.to_string(),
      description: match description {
        Some(desc) => Some(desc.to_string()),
        None => None,
      },
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


  pub async fn save_tx(self: &Self, mut tx: sqlx::Transaction<'_, sqlx::Postgres>, identity: &Identity) -> Result<(), Error> {
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
    .execute(&mut tx)
    .await
    {
      Ok(_) => {
        match tx.commit().await {
          Ok(_) => Ok(()),
          Err(e) => Err(e)
        }
      }
      ,
      Err(e) => {
        tx.rollback().await;
        Err(e)
      },
    }
  }
}

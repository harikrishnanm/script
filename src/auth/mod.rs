use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RbacParams {
  path: String,
  method: String,
  rbac_role: Vec<String>,
  rbac_user: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq)]
pub struct RbacPolicy {
  path: String,
  path_match: String,
  method: String,
  rbac_role: String,
  rbac_user: String,
}

impl PartialEq for RbacPolicy {
  fn eq(&self, other: &Self) -> bool {
    self.path == other.path
      && self.method == other.method
      && self.rbac_role == other.rbac_role
      && self.rbac_user == other.rbac_user
  }
}
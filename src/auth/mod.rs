use regex::RegexSet;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

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

/*#[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq)]
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
}*/

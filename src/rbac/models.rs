use crate::rbac::validators::*;
use chrono::NaiveDateTime;
use regex::RegexSet;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

pub struct Authenticate;

#[derive(Debug, Clone)]
pub struct Identity {
  pub user: String,
  pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthError {
  pub err_type: String,
  pub err_msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub iat: i64,
  pub exp: i64,
  pub user: String,
  pub roles: Vec<String>,
}

#[derive(Debug)]
pub struct Rbac {
  pub path_regex_set: RegexSet,
  pub methods: HashMap<usize, Vec<String>>,
  pub users: HashMap<usize, Vec<String>>,
  pub roles: HashMap<usize, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash)]
pub struct RbacParams {
  pub path: String,
  pub method: String,
  pub rbac_role: Vec<String>,
  pub rbac_user: String,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
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

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct RbacPolicy {
  pub rbac_id: Uuid,
  #[validate(length(min = 1, max = 25))]
  pub path: String,
  #[validate(custom = "validate_path_match")]
  pub path_match: String,
  #[validate(custom = "validate_method_match")]
  pub method: String,
  pub rbac_role: String,
  pub rbac_user: String,
  #[validate(length(max = 100))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  pub modified: NaiveDateTime,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub modified_by: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct RbacPolicyRequest {
  pub rbac_id: Uuid,
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

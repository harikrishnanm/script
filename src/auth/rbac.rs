use crate::auth::RbacPolicy;
use crate::DBPool;
use lazy_static::lazy_static;
use log::debug;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::sync::Mutex;

/*
Step1: Match the path regex
Step2: Match Method
If there is a match then
Step3: Match user
Step4: check if the role vectors have joint elements
*/

pub struct Rbac {
  regex_list: Vec<Regex>,
}

pub async fn init(db_pool: &DBPool) -> HashSet<RbacPolicy> {
  match sqlx::query_as!(
    RbacPolicy,
    "SELECT path, path_match, method, rbac_role, rbac_user FROM rbac"
  )
  .fetch_all(db_pool)
  .await
  {
    Ok(vals) => {
      debug!("{} RBAC policies found", vals.len());
      HashSet::from_iter(vals.into_iter())
    }
    Err(e) => {
      panic!("Error loading RBAC policy. {}", e);
    }
  }
}

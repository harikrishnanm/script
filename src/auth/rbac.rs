use crate::auth::RbacPolicy;
use crate::DBPool;
use log::{info, error, debug};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use serde::Deserialize;
use actix_web::{HttpResponse, post, web::{Data, Json}};
use sqlx::Error;

use crate::AppData;
use crate::error::ScriptError;
/*
Step1: Match the path regex
Step2: Match Method
If there is a match then
Step3: Match user
Step4: check if the role vectors have joint elements
*/
#[derive(Deserialize, Debug)]
pub enum PathMatch{
  STARTSWITH(String),
  EXACT(String)
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

impl NewRbacPolicy {
  pub async fn save(self: &Self, db_pool: &DBPool)-> Result<(), Error>{

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
}

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

#[post("/admin/rbac")]
pub async fn save(
  data: Data<AppData>,
  rbac_policy: Json<NewRbacPolicy>,) -> HttpResponse {
  
    match rbac_policy.save(&data.db_pool).await {
      Ok(_) => {
        info!("Saved data");
      }, 
      Err(e) => {
        error!("Error creating rbac policy");
      }

    };
  
  HttpResponse::Ok().finish()
}


use crate::validators::*;
use crate::DBPool;
use chrono::offset::Utc;
use chrono::NaiveDateTime;
use log::*;
use regex::RegexSet;
use serde::{Deserialize, Serialize};
use sqlx::Error;
use std::collections::HashMap;
use std::hash::Hash;
use uuid::Uuid;
use validator::Validate;
use crate::AppData;

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

/*impl RbacParams {
    fn hash(self: &Self) -> String {
        use fasthash::sea;
        let mut buf = String::from(&self.path);
        buf.push_str(&self.method);
        buf.push_str(&self.rbac_role.join(""));
        buf.push_str(&self.rbac_user);
        sea::hash64(&buf.into_bytes()).to_string()
    }
}*/

#[derive(Deserialize, Debug)]
pub struct Authority {
    pub user: String,
    pub authority: String,
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

impl NewRbacPolicy {

    pub async fn save(
        self: &Self,
        db_pool: &DBPool,
        identity: &Identity,
    ) -> Result<RbacPolicy, Error> {
        debug!("{:?}", self);

        let description = match &self.description {
            Some(desc) => desc,
            None => "",
        };
        let rbac_id = uuid::Uuid::new_v4();
        match sqlx::query_as!(RbacPolicy,
      "INSERT INTO rbac(rbac_id, path, path_match, method, rbac_role, rbac_user, description, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
          RETURNING rbac_id, path, path_match, method, rbac_role, rbac_user, description, modified_by, modified",
      rbac_id,
      &self.path,
      &self.path_match,
      &self.method,
      &self.rbac_role,
      &self.rbac_user,
      &description,
      identity.user
    )
    .fetch_one(db_pool)
    .await
    {
      Ok(rbac_policy) => Ok(rbac_policy),
      Err(e) => Err(e),
    }
    }

    pub async fn save_tx(
        self: &Self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        identity: &Identity
    ) -> Result<(), Error> {
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
        .execute(tx)
        .await
        {
            Ok(_) => {
                Ok(())
            },
            Err(e) => {
                error!("Error saving rbac {}", e);
                Err(e)
            }
        }
    }
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

impl RbacPolicy {
    pub fn from_req_for_update(
        rbac_policy_request: &RbacPolicyRequest,
        identity: &Identity,
    ) -> RbacPolicy {
        RbacPolicy {
            rbac_id: rbac_policy_request.rbac_id,
            path: rbac_policy_request.path.clone(),
            path_match: rbac_policy_request.path_match.clone(),
            method: rbac_policy_request.method.clone(),
            rbac_role: rbac_policy_request.rbac_role.clone(),
            rbac_user: rbac_policy_request.rbac_user.clone(),
            description: rbac_policy_request.description.clone(),
            modified: Utc::now().naive_utc(),
            modified_by: Some(identity.user.clone()),
        }
    }

    pub async fn update(self: &Self, db_pool: &DBPool) -> Result<RbacPolicy, Error> {
        match sqlx::query_as!(RbacPolicy,
      "UPDATE rbac SET path = $1, path_match = $2, method = $3, rbac_role = $4, rbac_user = $5, description = $6, modified_by = $8, modified = $7
        WHERE rbac_id = $9 RETURNING rbac_id, path, path_match, method, rbac_role, rbac_user, description, modified_by, modified",
      &self.path,
      &self.path_match,
      &self.method,
      &self.rbac_role,
      &self.rbac_user,
      match &self.description{
        Some(description) => description,
        None => ""
      },
      self.modified,
      self.modified_by,
      self.rbac_id
    )
    .fetch_one(db_pool)
    .await
    {
      Ok(rbac_policy) => Ok(rbac_policy),
      Err(e) => {
        error!("Error updating rbac {}", e);
        Err(e)
      }
    }
    }
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

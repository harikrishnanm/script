use crate::error::ScriptError;
use crate::rbac::models::*;
use crate::DataStore;
use chrono::offset::Utc;
use log::*;

use mongodb::error::ErrorKind;

impl RbacPolicyRequest {
  pub async fn save(
    self: &Self,
    data_store: &DataStore,
    identity: &Identity,
  ) -> Result<RbacPolicy, ScriptError> {
    debug!("Sacing new RBAC policy {:?}", self);

    let description = match &self.description {
      Some(desc) => desc,
      None => "",
    }
    .to_string();

    let rbac_id = uuid::Uuid::new_v4().to_string();
    let created_by = String::from(&identity.user);
    let created_at = Utc::now().naive_utc();
    let path = String::from(&self.path);
    let path_match = String::from(&self.path_match);
    let rbac_user = String::from(&self.rbac_user);
    let rbac_role = String::from(&self.rbac_role);
    let method = String::from(&self.method);

    let new_policy = RbacPolicy {
      rbac_id,
      created_by,
      created_at,
      modified_at: None,
      modified_by: None,
      path,
      path_match,
      rbac_user,
      rbac_role,
      method,
      description,
    };
    let rbac_coll = &data_store.db.collection::<RbacPolicy>("RBAC");

    match rbac_coll.insert_one(&new_policy, None).await {
      Ok(result) => {
        debug!("Result from driver {:?}", result);
        Ok(new_policy)
      }
      Err(e) => {
        error!("Error saving RBAC policy {}", e);
        match *e.kind {
          ErrorKind::Write(_write_failure) => {
            return Err(ScriptError::RbacCreationConflict(
              "duplicate or conflicting params".to_string(),
            ));
          }
          _ => return Err(ScriptError::UnexpectedError),
        }
      }
    }
  }
}
/*
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
}*/

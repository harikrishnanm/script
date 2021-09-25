use crate::DBPool;
use chrono::offset::Utc;
use log::*;

use sqlx::Error;

use crate::rbac::models::*;

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
        identity: &Identity,
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
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Error saving rbac {}", e);
                Err(e)
            }
        }
    }
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

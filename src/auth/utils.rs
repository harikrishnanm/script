use actix_web::dev::ServiceRequest;
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::{debug, error, trace};

use crate::auth::{AuthError, Claims, Identity, RbacParams};
use crate::AppData;

pub fn check_token(req: &ServiceRequest) -> Result<Identity, AuthError> {
  match &req.head().headers.get("authorization") {
    None => {
      error!("No authentication header found");
      Err(AuthError {
        err_type: "No Token".to_string(),
        err_msg: "No Token found in request".to_string(),
      })
    }
    Some(bearer_token) => {
      trace!("Auth token val {}", bearer_token.to_str().unwrap());
      debug!("Getting authentication header");

      let token_str = match bearer_token.to_str() {
        Ok(value) => {
          if value.starts_with("bearer") || value.starts_with("Bearer") {
            let token = value[6..value.len()].trim();
            token
          } else {
            error!("Invalid token string. Does not start with Bearer or bearer");
            return Err(AuthError {
              err_type: "Token error".to_string(),
              err_msg: "Invalid authorization token".to_string(),
            });
          }
        }
        Err(e) => {
          error!("Error converting token to string {}", e);
          return Err(AuthError {
            err_type: "Token error".to_string(),
            err_msg: "Invalid token string".to_string(),
          });
        }
      };

      match decode::<Claims>(
        &token_str,
        &DecodingKey::from_secret("123456".as_ref()),
        &Validation::default(),
      ) {
        Ok(token_data) => {
          debug!("Token Decoded successfully");
          let user_str = &token_data.claims.user;
          let roles = &token_data.claims.roles;
          Ok(Identity {
            user: user_str.to_string(),
            roles: roles.to_vec(),
          })
        }
        Err(e) => {
          error!("Decoding error {}", e);
          Err(AuthError {
            err_type: "Token error".to_string(),
            err_msg: "Unable to decode token".to_string(),
          })
        }
      }
    }
  }
}

pub fn check_rbac(rbac_params: RbacParams, app_data: &AppData) -> Result<(), AuthError> {
  debug!("Checking rbac policy");
  Ok(())
}

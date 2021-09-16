use actix_web::dev::ServiceRequest;
use jsonwebtoken::{decode, DecodingKey, Validation};
use log::{debug, error, trace};

use crate::auth::{AuthError, Claims, Identity, RbacParams, Rbac};
use crate::AppData;

const WILDCARD: &str = r#"*"#;

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
  use std::collections::hash_map::DefaultHasher;
  use std::hash::Hash;
  use std::hash::Hasher;

  let mut hasher = DefaultHasher::new();
  let h = rbac_params.hash(&mut hasher);
  hasher.finish();
  debug!("Checking rbac policy");
  debug!("RbacParam hash {:?}", h);
  let rbac = &app_data.rbac.lock().unwrap();
  let path_regex_set = &rbac.path_regex_set;
  let methods = &rbac.methods;
  let users = &rbac.users;
  let roles = &rbac.roles;
  let matches: Vec<usize> = path_regex_set
    .matches(&rbac_params.path)
    .into_iter()
    .collect();

  match check_policy(&rbac_params, rbac, &matches) {
    true => {
      debug!("Route allowed");
      Ok(())
    }
    false => Err(AuthError {
      err_type: "RBAC".to_string(),
      err_msg: "Access denied by policy".to_string(),
    }),
  }
}

fn check_policy(rbac_params: &RbacParams, rbac: &Rbac, matches: &Vec<usize>) -> bool {
  let methods = &rbac.methods;
  let users = &rbac.users;
  let roles = &rbac.roles;
  let wildcard = &String::from(WILDCARD);

  match matches.len() {

    0 => false,
    _ => {
      let (mut method_pass, mut user_pass, mut role_pass) = (false, false, false);
      for m in matches {
        let methods_vec = methods.get(&m).unwrap();
        method_pass = methods_vec.contains(&wildcard) || methods_vec.contains(&rbac_params.method);
        debug!(
          "Checking for method match of {} in {:?}: {}",
          &rbac_params.method,
          methods.get(&m),
          method_pass
        );
        let users_vec = users.get(&m).unwrap();
        user_pass = users_vec.contains(&wildcard) || users_vec.contains(&rbac_params.rbac_user);
        debug!(
          "Checking for user match of {} in {:?}: {}",
          &rbac_params.rbac_user,
          users.get(&m),
          user_pass
        );

        let roles_vec = roles.get(&m).unwrap();
        role_pass = roles_vec.contains(&wildcard);
        if !role_pass {
          for role in roles_vec {
            if rbac_params.rbac_role.contains(role){
              role_pass = true;
              break;
            }
          }
        }
      }
      method_pass && user_pass && role_pass
    }
  }
}

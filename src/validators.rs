use log::{debug, error};

use crate::constants::PATH_MATCH;
use validator::ValidationError;
pub fn validate_path_match(path_match: &str) -> Result<(), ValidationError> {
  debug!("Validating rbac_path_match ");
  if !PATH_MATCH.contains(&path_match) {
    error!("Invalid path_match parameter");
    return Err(ValidationError::new("Invalid Path match"));
  }
  Ok(())
}

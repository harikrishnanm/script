use log::{debug, error};

use crate::constants;
use validator::ValidationError;

const PATH_MATCH: [&str; 2] = ["EXACT", "STARTSWITH"];
const METHODS: [&str; 7] = ["GET", "POST", "HEAD", "PUT", "DELETE", "PATCH", "OPTIONS"];

pub fn validate_path_match(path_match: &str) -> Result<(), ValidationError> {
    debug!("Validating rbac_path_match ");
    if !PATH_MATCH.contains(&path_match) {
        error!("Invalid path_match parameter");
        return Err(ValidationError::new("Invalid Path match"));
    }
    Ok(())
}

pub fn validate_method_match(method: &str) -> Result<(), ValidationError> {
    debug!("Validating rbac_method_match ");
    if !(METHODS.contains(&method) || method == constants::WILDCARD) {
        error!("Invalid path_match parameter");
        return Err(ValidationError::new("Invalid Method"));
    }
    Ok(())
}

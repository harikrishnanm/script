use actix_web::{dev::HttpResponseBuilder, error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use log::error;
use serde_derive::Serialize;

#[derive(Debug, Display, Error)]
pub enum ScriptError {
  #[display(fmt = "Error creating rbac policy")]
  RbacCreationError,
}

#[derive(Serialize)]
pub struct ErrorResponse {
  error_message: String,
}

impl Clone for ScriptError {
  fn clone(&self) -> Self {
    match self {
      ScriptError::RbacCreationError => ScriptError::RbacCreationError,
    }
  }
}

impl ResponseError for ScriptError {
  fn error_response(&self) -> HttpResponse {
    error!("Error {}", self.to_string());
    HttpResponseBuilder::new(self.status_code()).json(ErrorResponse {
      error_message: self.to_string(),
    })
  }

  fn status_code(&self) -> StatusCode {
    match *self {
      ScriptError::RbacCreationError => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

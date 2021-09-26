use actix_web::{dev::HttpResponseBuilder, error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::Display;
use log::error;
use serde_derive::Serialize;

#[derive(Debug, Display)]
pub enum ScriptError {
    #[display(fmt = "Duplicate policy. Creation failed {}", _0)]
    RbacCreationConflict(String),
    #[display(fmt = "Creation failed due to an unexpected error")]
    UnexpectedRbacCreationFailure,
    #[display(fmt = "Could not save text")]
    TextCreationFailure,
    #[display(fmt = "Requested File not found")]
    FileNotFound,
    #[display(fmt = "This request cannot be processed. {}", _0)]
    BadRequest(String),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error_message: String,
}

/*impl Clone for ScriptError {
    fn clone(&self) -> Self {
        match self {
            ScriptError::RbacCreationConflict => ScriptError::RbacCreationConflict,
            ScriptError::UnexpectedRbacCreationFailure => ScriptError::RbacCreationConflic,
        }
    }
}*/

impl ResponseError for ScriptError {
    fn error_response(&self) -> HttpResponse {
        error!("Error {}", self.to_string());
        HttpResponseBuilder::new(self.status_code()).json(ErrorResponse {
            error_message: self.to_string(),
        })
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ScriptError::RbacCreationConflict(_) => StatusCode::BAD_REQUEST,
            ScriptError::UnexpectedRbacCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
            ScriptError::FileNotFound => StatusCode::NOT_FOUND,
            ScriptError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ScriptError::TextCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

use actix_web::{dev::HttpResponseBuilder, error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::Display;
use log::error;
use serde_derive::Serialize;

#[derive(Debug, Display)]
pub enum ScriptError {
    #[display(fmt = "Requested resource not found")]
    ResourceNotFound,
    #[display(fmt = "Error writing to database")]
    DatabaseError(String),
    #[display(fmt = "Duplicate policy. Creation failed due to {}", _0)]
    RbacCreationConflict(String),
    #[display(fmt = "Creation failed due to an unexpected error")]
    UnexpectedRbacCreationFailure,
    #[display(fmt = "Could not save text")]
    ContentCreationFailure,
    #[display(fmt = "Requested resource not found")]
    FileNotFound,
    #[display(fmt = "Error creating folder")]
    FolderCreationError,
    #[display(fmt = "Error creating file")]
    FileCreationError,
    #[display(fmt = "Error creating asset")]
    AssetCreationError,
    #[display(fmt = "This request cannot be processed. {}", _0)]
    BadRequest(String),
    #[display(fmt = "Unexpexted Error")]
    UnexpectedError,
    #[display(fmt = "Transaction Error")]
    TransactionError,
    #[display(fmt = "Content Format Error")]
    TaxonomyMismatch,
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
            ScriptError::ResourceNotFound => StatusCode::NOT_FOUND,
            ScriptError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ScriptError::ContentCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
            ScriptError::FolderCreationError => StatusCode::INTERNAL_SERVER_ERROR,
            ScriptError::FileCreationError => StatusCode::INTERNAL_SERVER_ERROR,
            ScriptError::AssetCreationError => StatusCode::INTERNAL_SERVER_ERROR,
            ScriptError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
            ScriptError::TransactionError => StatusCode::INTERNAL_SERVER_ERROR,
            ScriptError::TaxonomyMismatch => StatusCode::BAD_REQUEST,
            ScriptError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

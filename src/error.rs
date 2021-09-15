use actix_web::{dev::HttpResponseBuilder, error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use log::error;
use serde_derive::Serialize;

#[derive(Debug, Display, Error)]
pub enum KryptoError {
  #[display(fmt = "Error encrypting data")]
  EncryptionError,
  #[display(fmt = "Error decrypting data")]
  DecryptionError,
  #[display(fmt = "Invalid encrypted input")]
  InvalidBase64CipherText,
  #[display(fmt = "Invalid cipher")]
  InvalidCipher,
}

#[derive(Serialize)]
pub struct ErrorResponse {
  error_message: String,
}

impl Clone for KryptoError {
  fn clone(&self) -> Self {
    match self {
      KryptoError::DecryptionError => KryptoError::DecryptionError,
      KryptoError::EncryptionError => KryptoError::EncryptionError,
      KryptoError::InvalidBase64CipherText => KryptoError::InvalidBase64CipherText,
      KryptoError::InvalidCipher => KryptoError::InvalidCipher,
    }
  }
}

impl ResponseError for KryptoError {
  fn error_response(&self) -> HttpResponse {
    error!("Error {}", self.to_string());
    HttpResponseBuilder::new(self.status_code()).json(ErrorResponse {
      error_message: self.to_string(),
    })
  }

  fn status_code(&self) -> StatusCode {
    match *self {
      KryptoError::EncryptionError => StatusCode::INTERNAL_SERVER_ERROR,
      KryptoError::DecryptionError => StatusCode::INTERNAL_SERVER_ERROR,
      KryptoError::InvalidBase64CipherText => StatusCode::BAD_REQUEST,
      KryptoError::InvalidCipher => StatusCode::BAD_REQUEST,
    }
  }
}

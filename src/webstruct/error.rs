use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum HttpStatusCode {
  Informational(u16),
  Success(u16),
  Redirection(u16),
  ClientError(u16),
  ServerError(u16),
}

#[derive(Serialize, Deserialize)]
pub struct ApiError {
  status_code: HttpStatusCode,
  message: String
}

impl ApiError {
  pub fn new(status_code: HttpStatusCode, message: String) -> ApiError {
    ApiError { status_code, message }
  }
}
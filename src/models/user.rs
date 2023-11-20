use argon2::Config;
use rand::Rng;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::{schema::users, webstruct::error::{ApiError, HttpStatusCode}};

#[derive(Debug, Serialize, Deserialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
  pub id: i32,
  pub username: String,
  #[serde(skip_serializing)]
  pub password: String,
}


impl User {
  pub fn verify_password(&self, password: &str) -> Result<bool, ApiError> {
    argon2::verify_encoded(&self.password, password.as_bytes()).map_err(|_e| {
        ApiError::new(
            HttpStatusCode::ClientError(401),
            "Password is not correct".to_string(),
        )
    })
  }
}


#[derive(Insertable, Deserialize, Clone)]
#[diesel(table_name = users)]
pub struct NewUser {
  pub username: String,
  pub password: String,
}


impl NewUser {
  pub fn hash_password(&mut self) -> Result<(), ApiError> {
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();

    self.password =
        argon2::hash_encoded(self.password.as_bytes(), &salt, &config).map_err(|_e| {
            ApiError::new(
                HttpStatusCode::ServerError(500),
                "Cannot hash password".to_string(),
            )
        })?;

    Ok(())
  }
}
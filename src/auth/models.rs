use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthData {
  pub username: String,
  pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
  pub sub: String,
  pub exp: usize,
}
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use std::time::{SystemTime, UNIX_EPOCH};

use super::models::Claims;

pub fn generate_token(username: &str) -> String {
  let expiration = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards")
    .as_secs() + 60 * 60;

  let claims = Claims {
    sub: username.to_owned(),
    exp: expiration as usize,
  };

  encode(&Header::default(), &claims, &EncodingKey::from_secret(b"secret")).unwrap()
}

pub fn verify_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
  decode::<Claims>(
    token,
    &DecodingKey::from_secret(b"secret"),
    &Validation::default(),
  )
}
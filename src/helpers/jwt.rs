use crate::schema::Claims;
use jsonwebtoken::{EncodingKey, Header, encode};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_jwt_token<T: serde::Serialize>(
  sub: T,
  secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
  let exp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs() as usize
    + 3600;

  let claims = Claims { sub, exp };

  encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(secret.as_bytes()),
  )
}

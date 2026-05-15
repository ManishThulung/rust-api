use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupSchema {
  pub name: String,
  pub password: String,
  pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninSchema {
  pub email: String,
  pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims<T> {
  pub sub: T,
  pub exp: usize,
}

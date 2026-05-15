use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserModel {
  pub id: Uuid,
  pub name: String,
  pub password: String,
  pub email: String,
  pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

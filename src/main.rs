use std::sync::Arc;

use dotenv::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};

mod config;
mod errors;
mod extractors;
mod handler;
mod helpers;
mod middleware;
mod models;
mod route;
mod schema;

use crate::route::create_router;

pub struct AppState {
  db: PgPool,
  jwt_secret: String,
}

#[tokio::main]
async fn main() {
  dotenv().ok();
  let db_url = std::env::var("DATABASE_URL").expect("Database url needed");
  let jwt_secret = std::env::var("JWT_SECRET").expect("jwt secret needed");
  println!("{}", db_url);

  let pool = match PgPoolOptions::new()
    .max_connections(10)
    .connect(&db_url)
    .await
  {
    Ok(pool) => {
      println!("db connection successfull");
      pool
    }
    Err(e) => {
      println!("db connection failed: {}", e);
      std::process::exit(1)
    }
  };

  let app = create_router(Arc::new(AppState {
    db: pool.clone(),
    jwt_secret: jwt_secret,
  }));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
  println!("server running at 0.0.0.0:4000");
  axum::serve(listener, app).await.unwrap()
}

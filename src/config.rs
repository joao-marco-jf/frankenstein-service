use std::env;
use sqlx::postgres::PgPoolOptions;

pub struct Config {
  pub database_url: String,
}

impl Config {
  pub fn from_env() -> Self {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Config { database_url }
  }

  pub async fn create_pool(&self) -> sqlx::PgPool {
    PgPoolOptions::new()
      .max_connections(5)
      .connect(&self.database_url)
      .await
      .expect("Failed to create pool")
  }
}
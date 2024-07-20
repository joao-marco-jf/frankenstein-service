use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
  pub id: i32,
  pub title: String,
  pub description: Option<String>,
  pub completed: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTask {
  pub title: String,
  pub description: Option<String>,
}
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

use crate::tasks::models::Task;

use super::models::CreateTask;

pub async fn get_tasks(pool: web::Data<PgPool>) -> impl Responder {
  match sqlx::query_as!(Task, "SELECT id, title, description, completed FROM tasks")
    .fetch_all(pool.get_ref())
    .await
  {
    Ok(tasks) => HttpResponse::Ok().json(json!({
      "status": "success",
      "message": "Tasks fetched successfully",
      "data": tasks
    })),
    Err(_) => HttpResponse::InternalServerError().json(json!({
      "status": "error",
      "message": "Failed to fetch tasks"
    }))
  }
}

pub async fn create_task(pool: web::Data<PgPool>, task: web::Json<CreateTask>) -> impl Responder {
  let task_title = task.title.clone();
  let task_description = task.description.clone();

  match sqlx::query_as!(
    Task,
    "INSERT INTO tasks (title, description) VALUES ($1, $2)",
    task_title,
    task_description
  )
  .fetch_all(pool.get_ref())
  .await
  {
    Ok(_) =>  HttpResponse::Created().json(json!({
      "status": "success",
      "message": "Task created successfully"
    })),
    Err(_) => HttpResponse::InternalServerError().json(json!({
      "status": "error",
      "message": "Failed to create task"
    }))
  }
}
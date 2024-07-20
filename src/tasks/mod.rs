pub mod handlers;
pub mod models;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
      web::resource("/tasks")
        .route(web::get().to(handlers::get_tasks))
    );
    cfg.service(
      web::resource("/tasks/1")
        .route(web::post().to(handlers::create_task))
    );
}
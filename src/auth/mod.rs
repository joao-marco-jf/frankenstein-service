pub mod handlers;
pub mod models;
pub mod services;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
      web::resource("/login")
        .route(web::post().to(handlers::login))
    );
    cfg.service(
      web::resource("/verify_token")
        .route(web::get().to(handlers::verify_token_handler))
    );
}
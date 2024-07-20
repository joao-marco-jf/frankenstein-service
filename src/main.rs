use actix_web::{web, get, App, HttpServer};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

mod auth;
mod tasks;
mod config;

#[get("/")]
async fn index() -> &'static str {
    info!("[index] Request received");

    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed");

    let config = config::Config::from_env();
    let pool = config::Config::create_pool(&config).await;

    info!("[main] Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(index)
            .configure(auth::init)
            .configure(tasks::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
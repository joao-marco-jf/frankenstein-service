use tracing::info;
use actix_web::{web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::auth::services::{generate_token, verify_token};
use serde_json::json;

use super::models::AuthData;

pub async fn login(auth_data: web::Json<AuthData>) -> impl Responder {
    info!("[login] Login attempt with username: {:?}", auth_data.username);

    if auth_data.username == "Admin" && auth_data.password == "123456" {
        let token = generate_token(&auth_data.username);
        HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Login successful",
            "data": {
                "token": token
            }
        }))
    } else {
        HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Invalid credentials"
        }))
    }
}

pub async fn verify_token_handler(auth: BearerAuth) -> impl Responder {
    info!("[verify_token] Verifying token: {:?}", auth.token());

    let token = auth.token();
    match verify_token(&token) {
        Ok(token_data) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Token is valid",
            "data": token_data.claims
        })),
        Err(_) => HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Invalid token"
        }))
    }
}
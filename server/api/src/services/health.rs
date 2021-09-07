use actix_web::{Responder, Scope};
use actix_web::web::{scope, to};

pub async fn health() -> impl Responder {
    "CodeMountainOJ API v1"
}

pub fn get_health_service() -> Scope {
    scope("/health")
        .route("/test", to(health))
}
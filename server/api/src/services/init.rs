use actix_web::web::{scope, ServiceConfig};
use crate::services::health::get_health_service;

pub fn init_v1api(cfg: &mut ServiceConfig) {
    cfg.service(scope("/v1")
        .service(get_health_service()));
}

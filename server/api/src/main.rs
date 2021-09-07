mod services;
mod config;

use crate::services::init::init_v1api;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use log::info;
use crate::config::run_config_check;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    run_config_check();

    info!("Listening on address: {}", config::get::<String>("LISTENING_URL"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(init_v1api)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

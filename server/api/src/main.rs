/*
 *  CodeMountain is a free and open source online judge open for everyone
 *  Copyright (C) 2021 MD Gaziur Rahman Noor and contributors
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#[macro_use]
extern crate diesel;

use crate::config::run_config_check;
use crate::db::create_pool;
use crate::services::init::init_v1api;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use log::info;
use actix_ratelimit::{MemoryStore, RateLimiter, MemoryStoreActor};
use std::time::Duration;

mod common;
mod config;
mod db;
mod errors;
mod jwt;
mod guards;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    run_config_check();

    let store = MemoryStore::new();

    info!(
        "Listening on address: {}",
        config::get::<String>("LISTENING_URL")
    );
    HttpServer::new(move || {
        App::new()
            .wrap(
                RateLimiter::new(MemoryStoreActor::from(store.clone()).start())
                    .with_interval(Duration::from_secs(60))
                    .with_max_requests(100)
            )
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .data(create_pool())
            .configure(init_v1api)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

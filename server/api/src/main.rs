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
pub mod db;
pub mod endpoints;
pub mod env;
pub mod errors;
pub mod guards;
pub mod jwt;
pub mod mailer;
pub mod redis;

#[cfg(test)]
mod tests;

use std::time::Duration;

use actix_ratelimit::MemoryStore;
use actix_ratelimit::MemoryStoreActor;
use actix_ratelimit::RateLimiter;
use actix_web::{web, App, HttpServer};
use db::create_pool;
use dotenv::dotenv;
use endpoints::auth;
use endpoints::user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::do_env_check();

    HttpServer::new(|| {
        App::new()
            .data(create_pool())
            .wrap(RateLimiter::new(
                MemoryStoreActor::from(MemoryStore::new()).start())
                        .with_interval(Duration::from_secs(60))
                        .with_max_requests(100)
            )
            .route("/auth/login", web::post().to(auth::login::login_handler))
            .route(
                "/auth/register",
                web::post().to(auth::register::registration_handler),
            )
            .route(
                "/auth/token/refresh",
                web::post().to(auth::refresh_accesstoken::refresh_accesstoken_handler),
            )
            .route(
                "/auth/get/passwordresettoken",
                web::post().to(auth::recovery::send_password_reset_email)
            )
            .route(
                "/auth/reset/password",
                web::post().to(auth::recovery::recover_password)
            )
            .route(
                "/auth/status",
                web::post().to(auth::authstatus::check_auth_status_handler)
            )
            .route(
                "/user/update/firstname",
                web::post().to(user::data_update::edit_firstname_handler),
            )
            .route(
                "/user/update/lastname",
                web::post().to(user::data_update::edit_lastname_handler),
            )
            .route(
                "/user/update/email",
                web::post().to(user::data_update::edit_email_handler)
            )
            .route(
                "/user/update/password",
                web::post().to(user::data_update::edit_password_handler)
            )
            .route(
                "/user/query/id",
                web::post().to(user::data_query::get_user_by_id_handler)
            )
            .route(
                "/user/query/username",
                web::post().to(user::data_query::get_user_by_username_handler)
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

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
pub mod errors;
pub mod jwt;
pub mod env;
pub mod extractors;

use actix_web::{ HttpServer, App, web };
use endpoints::auth;
use dotenv::dotenv;
use db::create_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::do_env_check();

    HttpServer::new(|| {
        App::new()
            .data(create_pool())
            .route("/auth/login", web::post().to(auth::login::login_handler))
            .route("/auth/register", web::post().to(auth::register::registration_handler))
            .route("/auth/token/refresh", web::post().to(auth::refresh_accesstoken::refresh_accesstoken))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
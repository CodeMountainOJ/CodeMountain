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
use actix_web::{test, web, App};
use crate::auth::login::login_handler;
use crate::auth::payload::LoginRequest;
use crate::db::create_pool;
use std::env::set_var;

#[actix_rt::test]
async fn test_login_successful() {
    set_var("JWT_SECRET_KEY", "beryberysecret");

    let pool = create_pool();
    let mut app = test::init_service(
        App::new()
        .data(pool.clone())
        .route("/", web::post().to(login_handler))
    ).await;

    let req = test::TestRequest::post()
    .set_json(&LoginRequest {
        email: String::from("john_doe@example.com"),
        password: String::from("password")
    })
    .to_request();

    let resp = test::call_service(&mut app, req).await;

    dbg!(resp.response());
    assert!(resp.status().is_success(), "This should be successful");
}

#[actix_rt::test]
async fn test_login_failure() {
    set_var("JWT_SECRET_KEY", "beryberysecret");

    let pool = create_pool();
    let mut app = test::init_service(
        App::new()
        .data(pool)
        .route("/", web::post().to(login_handler))
    ).await;

    let req = test::TestRequest::post()
    .set_json(&LoginRequest {
        email: String::from("john_doe@example.com"),
        password: String::from("password1")
    })
    .to_request();

    let resp = test::call_service(&mut app, req).await;

    dbg!(resp.response());
    assert!(resp.status().is_client_error(), "The request shouldn't be successful");
}
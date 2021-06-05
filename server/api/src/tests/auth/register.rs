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
use crate::auth::register::registration_handler;
use crate::auth::payload::RegisterRequest;
use crate::db::create_pool;
use std::env::set_var;

#[actix_rt::test]
async fn test_register_unsuccessful() {
    set_var("DATABASE_URL", "postgres://postgres:a3b2c100@127.0.0.1/codemountain_test");
    set_var("JWT_SECRET_KEY", "beryberysecret");

    let pool = create_pool();
    let mut app = test::init_service(
        App::new()
        .data(pool)
        .route("/", web::post().to(registration_handler))
    ).await;

    let req = test::TestRequest::post()
    .set_json(&RegisterRequest {
        firstname: String::from("John"),
        lastname: String::from("Doe"),
        username: String::from("johndoe"),
        email: String::from("john_doe@example.com"),
        password: String::from("password")
    })
    .to_request();

    let resp = test::call_service(&mut app, req).await;

    dbg!(resp.response());
    assert!(resp.status().is_client_error(), "The request shouldn't be successful");
}

#[actix_rt::test]
async fn test_register_successful() {
    use diesel::prelude::*;
    use crate::db::schema::users::dsl::*;

    set_var("DATABASE_URL", "postgres://postgres:a3b2c100@127.0.0.1/codemountain_test");
    set_var("JWT_SECRET_KEY", "beryberysecret");

    let pool = create_pool();
    let mut app = test::init_service(
        App::new()
        .data(pool.clone())
        .route("/", web::post().to(registration_handler))
    ).await;

    let req = test::TestRequest::post()
    .set_json(&RegisterRequest {
        firstname: String::from("test"),
        lastname: String::from("user"),
        username: String::from("testuser"),
        email: String::from("testuser@example.com"),
        password: String::from("password")
    })
    .to_request();
    

    let resp = test::call_service(&mut app, req).await;

    let one_con = pool.get().expect("Failed to get connection");

    diesel::delete(users.filter(username.eq("testuser")))
            .execute(&one_con)
            .expect("Failed to delete newly created user");

    dbg!(resp.response());
    assert!(resp.status().is_success(), "The request should be successful");
}


#[actix_rt::test]
async fn test_register_not_unique_data() {
    set_var("DATABASE_URL", "postgres://postgres:a3b2c100@127.0.0.1/codemountain_test");
    set_var("JWT_SECRET_KEY", "beryberysecret");

    let pool = create_pool();
    let mut app = test::init_service(
        App::new()
        .data(pool)
        .route("/", web::post().to(registration_handler))
    ).await;

    let req = test::TestRequest::post()
    .set_json(&RegisterRequest {
        firstname: String::from("test"),
        lastname: String::from("user"),
        username: String::from("testuser"),
        email: String::from("john_doe@example.com"),
        password: String::from("password")
    })
    .to_request();

    let resp = test::call_service(&mut app, req).await;

    dbg!(resp.response());
    assert!(resp.status().is_client_error(), "The request should be unsuccessful");
}
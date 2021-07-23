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
use crate::db::create_pool;
use crate::endpoints::user::payload::GetUserByUsernamePayload;
use actix_web::{test, web, App};
use std::env::set_var;
use crate::endpoints::user::data_query::get_user_by_username_handler;

static AUTHTOKEN: &'static str = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1aWQiOjI1LCJleHAiOjk5OTk5OTk5OTksInRva2VuX3R5cGUiOiJBY2Nlc3NUb2tlbiJ9.iIBHQu2ZT4rsdTR_wCTITcCERhOgzGswSt5wWB3sWio";

#[actix_rt::test]
async fn test_get_user_by_id_successful() {
    set_var("JWT_SECRET_KEY", "beryberysecret");

    let pool = create_pool();
    let mut app = test::init_service(
        App::new()
            .data(pool.clone())
            .route("/", web::post().to(get_user_by_username_handler)),
    )
        .await;

    let req = test::TestRequest::post()
        .header("authorization", AUTHTOKEN)
        .set_json(&GetUserByUsernamePayload {
            username: String::from("johndoe")
        })
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    dbg!(resp.response());
    assert!(resp.status().is_success(), "This should be successful");
}

#[actix_rt::test]
async fn test_edit_password_unsuccessful() {
    set_var("JWT_SECRET_KEY", "beryberysecret");

    let pool = create_pool();
    let mut app = test::init_service(
        App::new()
            .data(pool.clone())
            .route("/", web::post().to(get_user_by_username_handler)),
    )
        .await;

    let req = test::TestRequest::post()
        .header("authorization", AUTHTOKEN)
        .set_json(&GetUserByUsernamePayload {
            username: String::from("weeeeeeeeeeeeeeeeeeeeeeeeeeeeeee")
        })
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    dbg!(resp.response());
    assert!(
        resp.status().is_client_error(),
        "This should be unsuccessful"
    );
}

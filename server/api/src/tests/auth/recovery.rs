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
use crate::endpoints::auth::recovery::{send_password_reset_email, recover_password};
use crate::endpoints::auth::payload::{SendPasswordResetTokenPayload,ResetPasswordPayload};
use actix_web::{test, web, App};
use std::env::set_var;

static PASSWORD_RESET_TOKEN: &'static str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1aWQiOjI1LCJleHAiOjk5OTk5OTk5OTk5OSwidG9rZW5fdHlwZSI6IlBhc3N3b3JkUmVzZXRUb2tlbiJ9.WWd45y1Ue24m541Cujtz8iD1dSmtCxsnrVWjB9kJlzo";

#[actix_rt::test]
async fn test_send_password_reset_email() {
    set_var("JWT_SECRET_KEY", "beryberysecret");

    let pool = create_pool();
    let mut app = test::init_service(
        App::new()
            .data(pool.clone())
            .route("/", web::post().to(send_password_reset_email)),
    )
    .await;

    let req = test::TestRequest::post()
        .set_json(&SendPasswordResetTokenPayload {
            email: "john_doe@example.com".to_string(),
        })
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    dbg!(resp.response());
    assert!(resp.status().is_success(), "This should be successful");
}

#[actix_rt::test]
async fn test_edit_email_successful_with_repeatations() {
    set_var("JWT_SECRET_KEY", "beryberysecret");

    let pool = create_pool();
    let mut app = test::init_service(
        App::new()
            .data(pool.clone())
            .route("/", web::post().to(recover_password)),
    )
    .await;

    let req = test::TestRequest::post()
        .set_json(&ResetPasswordPayload {
            reset_token: PASSWORD_RESET_TOKEN.to_string(),
            password: "password".to_string()
        })
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    dbg!(resp.response());
    assert!(
        resp.status().is_success(),
        "This should be successful"
    );

    let req = test::TestRequest::post()
        .set_json(&ResetPasswordPayload {
            reset_token: PASSWORD_RESET_TOKEN.to_string(),
            password: "password".to_string()
        })
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    dbg!(resp.response());
    assert!(
        resp.status().is_client_error(),
        "Repeating reset tokens should be unsuccessful"
    );
}

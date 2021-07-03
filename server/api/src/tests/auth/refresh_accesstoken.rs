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
use crate::auth::refresh_accesstoken::refresh_accesstoken_handler;
use crate::auth::payload::RefreshAccessTokenPayload;
use crate::db::create_pool;
use std::env::set_var;

static REFRESH_TOKEN: &'static str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1aWQiOjI1LCJleHAiOjk5OTk5OTk5OTksInRva2VuX3R5cGUiOiJSZWZyZXNoVG9rZW4ifQ.fijR4UiUopM2V_DvA5XJ2wxa6bxY1ikMRsnwhiKmsvc";

#[actix_rt::test]
pub async fn refresh_accesstoken_ok() {

    set_var("JWT_SECRET_KEY", "beryberysecret");

    let pool = create_pool();
    let mut app = test::init_service(
        App::new()
        .data(pool)
        .route("/", web::post().to(refresh_accesstoken_handler))
    ).await;

    let req = test::TestRequest::post()
    .set_json(&RefreshAccessTokenPayload {
        refresh_token: REFRESH_TOKEN.to_string()
    })
    .to_request();

    let resp = test::call_service(&mut app, req).await;

    dbg!(resp.response());
    assert!(resp.status().is_success(), "The request should be successful");
}

#[actix_rt::test]
pub async fn refresh_accesstoken_not_ok() {

    set_var("JWT_SECRET_KEY", "beryberysecret");

    let pool = create_pool();
    let mut app = test::init_service(
        App::new()
        .data(pool)
        .route("/", web::post().to(refresh_accesstoken_handler))
    ).await;

    let req = test::TestRequest::post()
    .set_json(&RefreshAccessTokenPayload {
        refresh_token: "not_valid".to_string()
    })
    .to_request();

    let resp = test::call_service(&mut app, req).await;

    dbg!(resp.response());
    assert!(resp.status().is_client_error(), "The request should not be successful");
}
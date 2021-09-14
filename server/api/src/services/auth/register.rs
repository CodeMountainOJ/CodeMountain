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
use crate::common::StatusPayload;
use crate::db::users::mutations::insert_user;
use crate::db::users::query::is_unique_user;
use crate::db::Pool;
use crate::errors::Errors;
use actix_web::web::{Data, Json};
use actix_web::Responder;
use actix_web_validator::Json as validate_payload;
use rand::RngCore;
use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct RegisterRequest {
    #[validate(length(min = 1, max = 255))]
    firstname: String,

    #[validate(length(min = 1, max = 255))]
    nickname: String,

    #[validate(email)]
    email: String,

    #[validate(length(min = 1, max = 25))]
    username: String,

    #[validate(length(min = 8, max = 100))]
    password: String,
}

pub async fn register_handler(
    conn_pool: Data<Pool>,
    payload: validate_payload<RegisterRequest>,
) -> Result<impl Responder, Errors> {
    return if !is_unique_user(&conn_pool, &payload.username, &payload.email)? {
        Err(Errors::BadRequest(String::from(
            "Some fields aren't quite unique",
        )))
    } else {
        // make account and return statuspayload with success set to true
        let argon2_config = argon2::Config::default();
        let mut salt = [0u8; 30];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut salt);

        let hashed_password =
            argon2::hash_encoded(payload.password.as_bytes(), &salt, &argon2_config)
                .map_err(|_| Errors::InternalServerError)?;

        insert_user(
            &conn_pool,
            &payload.username,
            &payload.firstname,
            &payload.nickname,
            &payload.email,
            &hashed_password,
        )?;

        Ok(Json(StatusPayload {
            success: true,
            message: None,
        }))
    };
}

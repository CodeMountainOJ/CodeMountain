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
use actix_web::{
    web::{Data, Json},
    Responder,
};
use actix_web_validator::Json as validate_payload;
use argon2::verify_encoded;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    db::{users::query::get_user_by_email, Pool},
    errors::Errors,
    jwt::{accesstoken::generate_accesstoken, refreshtoken::generate_refreshtoken},
};

// payload structs
#[derive(Validate, Deserialize)]
pub struct LoginRequest {
    #[validate(email)]
    email: String,

    password: String,
}

#[derive(Serialize)]
pub struct AuthTokenPayload {
    pub refresh_token: String,
    pub access_token: String,
}

pub async fn login_handler(
    conn_pool: Data<Pool>,
    payload: validate_payload<LoginRequest>,
) -> Result<impl Responder, Errors> {
    let user = get_user_by_email(&conn_pool, &payload.email)
        .map_err(|_| Errors::BadRequest(String::from("Invalid creds")))?; // do not let the user know about errors
    let password_hash = user.password;

    // verify
    let matches = verify_encoded(&password_hash, payload.password.as_bytes())
        .map_err(|_| Errors::InternalServerError)?;

    if matches {
        // generate required tokens
        let access_token = generate_accesstoken(&user.id)?;
        let refresh_token = generate_refreshtoken(&user.id)?;

        Ok(Json(AuthTokenPayload {
            access_token,
            refresh_token,
        }))
    } else {
        return Err(Errors::BadRequest(String::from("Invalid creds")));
    }
}

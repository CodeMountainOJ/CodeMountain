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
use crate::common::{send_password_reset_email, StatusPayload};
use crate::config::get;
use crate::db::users::mutations::update_password;
use crate::db::users::query::{get_user, get_user_by_email};
use crate::db::Pool;
use crate::errors::Errors;
use crate::jwt::password_reset_token::{
    generate_password_reset_token, get_reset_token_data, verify_password_reset_token,
};
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::Responder;
use actix_web_validator::Json as validate;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct PasswordResetRequestPayload {
    #[validate(email)]
    pub email: String,
}

#[derive(Deserialize)]
pub struct ResetTokenVerifyPayload {
    pub reset_token: String,
}

#[derive(Serialize)]
pub struct ResetTokenValidationPayload {
    pub valid: bool,
}

#[derive(Deserialize, Validate)]
pub struct ResetPasswordPayload {
    pub reset_token: String,

    #[validate(length(min = 8, max = 100))]
    pub new_password: String,
}

pub async fn reset_password_request_handler(
    conn_pool: Data<Pool>,
    payload: validate<PasswordResetRequestPayload>,
) -> Result<impl Responder, Errors> {
    // get user
    let user = match get_user_by_email(&conn_pool, &payload.email) {
        Ok(u) => u,
        Err(Errors::InternalServerError) => return Err(Errors::InternalServerError),
        _ => {
            return Ok(Json(StatusPayload {
                success: true,
                message: Some(String::from("Check your email inbox")),
            }))
        }
    };

    // send email, that's it
    send_password_reset_email(
        &user.email,
        &get::<String>("SMTP_EMAIL"),
        "CodeMountainOJ: Password Recovery",
        &generate_password_reset_token(
            &user.id,
            &(user.password + &get::<String>("JWT_SECRET_KEY")),
        )?,
        &user.username,
        &user.nickname,
    );

    Ok(Json(StatusPayload {
        success: true,
        message: Some(String::from("Check your email inbox")),
    }))
}

pub async fn verify_reset_token_handler(
    conn_pool: Data<Pool>,
    payload: Json<ResetTokenVerifyPayload>,
) -> Result<impl Responder, Errors> {
    let token_data = get_reset_token_data(&payload.reset_token)?;

    // check if user exists
    match get_user(&conn_pool, token_data.user_id.parse::<Uuid>().unwrap()) {
        Ok(_) => Ok(Json(ResetTokenValidationPayload { valid: true })),
        Err(Errors::NotFound) => Ok(Json(ResetTokenValidationPayload { valid: false })),
        Err(e) => Err(e),
    }
}

pub async fn reset_password_handler(
    conn_pool: Data<Pool>,
    payload: validate<ResetPasswordPayload>,
) -> Result<impl Responder, Errors> {
    let token_data = get_reset_token_data(&payload.reset_token)?;

    // check if user exists
    let user = match get_user(&conn_pool, token_data.user_id.parse::<Uuid>().unwrap()) {
        Ok(u) => u,
        Err(Errors::NotFound) => {
            return Ok(Json(StatusPayload {
                success: false,
                message: Some(String::from("Invalid token!")),
            }))
        }
        Err(e) => return Err(e),
    };

    verify_password_reset_token(
        &payload.reset_token,
        &(user.password + &get::<String>("JWT_SECRET_KEY")),
    )?;

    // now set new password
    let argon2_config = argon2::Config::default();
    let mut salt = [0u8; 30];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut salt);

    let hashed_password =
        argon2::hash_encoded(payload.new_password.as_bytes(), &salt, &argon2_config)
            .map_err(|_| Errors::InternalServerError)?;

    update_password(&conn_pool, &hashed_password, &user.id).map(|_| {
        Json(StatusPayload {
            success: true,
            message: None,
        })
    })
}

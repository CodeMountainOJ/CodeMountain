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
use serde::{Serialize, Deserialize};
use validator::Validate;
use actix_web::web::{Data, Json};
use crate::db::Pool;
use actix_web_validator::Json as validate;
use actix_web::Responder;
use crate::errors::Errors;
use crate::db::users::mutations::{update_firstname, update_nickname, update_password, update_email};
use crate::guards::require_auth::RequireAuth;
use crate::common::StatusPayload;
use argon2::{verify_encoded, hash_encoded};
use rand::{thread_rng, RngCore};

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdateFirstnamePayload {
    #[validate(length(min = 1, max = 255))]
    firstname: String
}

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdateNicknamePayload {
    #[validate(length(min = 1, max = 255))]
    nickname: String
}

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdateEmailPayload {
    #[validate(email)]
    email: String
}

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdatePasswordPayload {
    #[validate(length(min = 8, max = 100))]
    old_password: String,

    #[validate(length(min = 8, max = 100))]
    new_password: String
}

pub async fn update_firstname_handler(conn_pool: Data<Pool>, payload: validate<UpdateFirstnamePayload>, user: RequireAuth) -> Result<impl Responder, Errors> {
    update_firstname(&conn_pool, &payload.firstname, &user.user.id).map(|_| Json(StatusPayload {
        success: true,
        message: None
    }))
}

pub async fn update_nickname_handler(conn_pool: Data<Pool>, payload: validate<UpdateNicknamePayload>, user: RequireAuth) -> Result<impl Responder, Errors> {
    update_nickname(&conn_pool, &payload.nickname, &user.user.id).map(|_| Json(StatusPayload {
        success: true,
        message: None
    }))
}

pub async fn update_password_handler(conn_pool: Data<Pool>, payload: validate<UpdatePasswordPayload>, user: RequireAuth) -> Result<impl Responder, Errors> {
    let is_correct_password = verify_encoded(&user.user.password, payload.old_password.as_bytes())
        .map_err(|_| Errors::InternalServerError)?;

    if !is_correct_password {
        return Err(Errors::BadRequest(String::from("Wrong Password!")));
    }

    let mut salt = [0u8; 30];
    let mut rng = thread_rng();
    rng.fill_bytes(&mut salt);

    let hashed_password = hash_encoded(payload.new_password.as_bytes(), &salt, &argon2::Config::default())
        .map_err(|_| Errors::InternalServerError)?;

    update_password(&conn_pool, &hashed_password, &user.user.id).map(|_| Json(StatusPayload {
        success: true,
        message: None
    }))
}

pub async fn update_email_handler(conn_pool: Data<Pool>, payload: validate<UpdateEmailPayload>, user: RequireAuth) -> Result<impl Responder, Errors> {
    update_email(&conn_pool, &payload.email, &user.user.id).map(|_| Json(StatusPayload {
        success: true,
        message: None
    }))
}
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
use super::payload::{LoginRequest, LoginTokens};
use crate::db::user::query::get_user_by_email;
use crate::db::Pool;
use crate::errors;
use crate::jwt::sign::{generate_accesstoken, generate_refreshtoken};
use actix_web::{web::Data, web::Json as actix_json, Responder};
use actix_web_validator::Json;
use bcrypt::verify;

pub async fn login_handler(
    conn_pool: Data<Pool>,
    req: Json<LoginRequest>,
) -> Result<impl Responder, errors::Errors> {
    let user = get_user_by_email(&req.email, conn_pool.as_ref())?;
    let raw_password = req.password.clone();
    let salted_password = user.password;

    let is_correct_password =
        verify(raw_password, &salted_password).map_err(|_| errors::Errors::InternalServerError)?;

    if is_correct_password {
        let access_token =
            generate_accesstoken(&user.id).map_err(|_| errors::Errors::InternalServerError)?;

        let refresh_token = match generate_refreshtoken(&user.id) {
            Ok(token) => token,
            Err(_) => return Err(errors::Errors::InternalServerError),
        };

        return Ok(actix_json(LoginTokens {
            access_token,
            refresh_token,
        }));
    }

    Err(errors::Errors::BadRequest("Wrong password!"))
}

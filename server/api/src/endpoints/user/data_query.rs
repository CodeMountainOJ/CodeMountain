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
use actix_web_validator::Json;
use actix_web::{web::Json as actix_json, web::Data, Responder};
use crate::endpoints::user::payload::{GetUserByIdPayload, GetUserByUsernamePayload};
use crate::db::Pool;
use crate::errors::Errors;
use crate::db::user::query::{get_user_by_uid, get_user_by_username};

pub async fn get_user_by_id_handler(
    payload: Json<GetUserByIdPayload>,
    conn_pool: Data<Pool>
) -> Result<impl Responder, Errors> {
    let user_id = payload.id;
    let mut user = get_user_by_uid(&user_id, &conn_pool)?;
    user.password = String::from("REDACTED");

    Ok(actix_json(user))
}

pub async fn get_user_by_username_handler(
    payload: Json<GetUserByUsernamePayload>,
    conn_pool: Data<Pool>
) -> Result<impl Responder, Errors> {
    let user_username = payload.username.clone();
    let mut user = get_user_by_username(&user_username, &conn_pool)?;
    user.password = String::from("REDACTED");

    Ok(actix_json(user))
}
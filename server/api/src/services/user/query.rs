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
use crate::db::users::models::SafeUser;
use crate::db::users::query::{get_user, get_users_from_query};
use crate::db::Pool;
use crate::errors::Errors;
use actix_web::web::{Data, Json};
use actix_web::Responder;
use serde::Deserialize;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct QueryPayload {
    query: String,
}

#[derive(Deserialize)]
pub struct QueryByIdPayload {
    id: String,
}

pub async fn get_user_by_query(
    conn_pool: Data<Pool>,
    payload: Json<QueryPayload>,
) -> Result<impl Responder, Errors> {
    Ok(Json(get_users_from_query(&conn_pool, &payload.query)?))
}

pub async fn get_user_by_id(
    conn_pool: Data<Pool>,
    payload: Json<QueryByIdPayload>,
) -> Result<impl Responder, Errors> {
    let user_id = Uuid::from_str(&payload.id);
    if user_id.is_err() {
        return Err(Errors::BadRequest(String::from("Invalid user id")));
    }

    Ok(Json(SafeUser::from(&get_user(
        &conn_pool,
        user_id.unwrap(),
    )?)))
}

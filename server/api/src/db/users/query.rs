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

use crate::{
    db::schema::users::dsl::*,
    db::{get_conn, Pool},
    errors::Errors,
};
use diesel::prelude::*;
use diesel::result::Error as diesel_error;
use uuid::Uuid;

use super::models::User;

fn map_err(err: diesel_error) -> Errors {
    match err {
        diesel_error::NotFound => Errors::NotFound,
        _ => Errors::InternalServerError,
    }
}

pub fn get_user(conn_pool: &Pool, user_id: Uuid) -> Result<User, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    users
        .filter(id.eq(user_id))
        .get_result(&conn)
        .map_err(|e| map_err(e))
}

pub fn get_user_by_email(conn_pool: &Pool, user_email: &str) -> Result<User, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    users
        .filter(email.eq(user_email))
        .get_result(&conn)
        .map_err(|e| map_err(e))
}

pub fn get_user_by_username(conn_pool: &Pool, user_username: &str) -> Result<User, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    users
        .filter(username.eq(user_username))
        .get_result(&conn)
        .map_err(|e| map_err(e))
}

pub fn get_users_from_query(conn_pool: &Pool, query: &str) -> Result<Vec<User>, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    users
        .filter(
            username
                .like(query)
                .or(email.like(query))
                .or(firstname.like(query))
                .or(nickname.like(query)),
        )
        .get_results(&conn)
        .map_err(|e| map_err(e))
}

pub fn is_unique_user(
    conn_pool: &Pool,
    user_username: &str,
    user_email: &str,
) -> Result<bool, Errors> {
    match get_user_by_username(conn_pool, user_username) {
        Err(e) => match e {
            Errors::NotFound => (),
            _ => return Err(e),
        },
        _ => return Ok(false),
    };

    match get_user_by_email(conn_pool, user_email) {
        Err(e) => match e {
            Errors::NotFound => (),
            _ => return Err(e),
        },
        _ => return Ok(false),
    };

    Ok(true)
}

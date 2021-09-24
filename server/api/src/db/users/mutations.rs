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
use super::models::NewUser;
use crate::{
    db::{get_conn, schema::users::dsl::*, Pool},
    errors::Errors,
};
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

pub fn insert_user(
    conn_pool: &Pool,
    new_username: &str,
    new_firstname: &str,
    new_nickname: &str,
    new_email: &str,
    new_password: &str,
) -> Result<usize, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    let new_user = NewUser {
        username: new_username,
        firstname: new_firstname,
        nickname: new_nickname,
        email: &String::from(new_email),
        password: new_password,
    };

    diesel::insert_into(users)
        .values(new_user)
        .execute(&conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn update_email(conn_pool: &Pool, new_email: &str, user_id: &Uuid) -> Result<usize, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    diesel::update(users.filter(id.eq(user_id)))
        .set(email.eq(String::from(new_email)))
        .execute(&conn)
        .map_err(|e| match e {
            Error::DatabaseError(de, _) => Errors::BadRequest(format!("{:?}", de)),
            _ => Errors::InternalServerError,
        })
}

pub fn update_firstname(
    conn_pool: &Pool,
    new_firstname: &str,
    user_id: &Uuid,
) -> Result<usize, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    diesel::update(users.filter(id.eq(user_id)))
        .set(firstname.eq(new_firstname))
        .execute(&conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn update_password(
    conn_pool: &Pool,
    new_password: &str,
    user_id: &Uuid,
) -> Result<usize, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    diesel::update(users.filter(id.eq(user_id)))
        .set(password.eq(new_password))
        .execute(&conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn update_nickname(
    conn_pool: &Pool,
    new_nickname: &str,
    user_id: &Uuid,
) -> Result<usize, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    diesel::update(users.filter(id.eq(user_id)))
        .set(nickname.eq(new_nickname))
        .execute(&conn)
        .map_err(|_| Errors::InternalServerError)
}

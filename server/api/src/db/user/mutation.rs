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
use super::model::{NewUser, User};
use crate::db::schema;
use crate::errors::Errors;
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;
use schema::users::dsl::*;

pub fn create_user<'a>(
    user_firstname: &'a str,
    user_lastname: &'a str,
    user_username: &'a str,
    user_email: &'a str,
    user_password: &'a str,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<User, Errors> {
    let new_user = NewUser {
        firstname: user_firstname,
        lastname: user_lastname,
        username: user_username,
        email: user_email,
        password: user_password,
    };

    match diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(conn)
    {
        Ok(u) => Ok(u),
        Err(_) => Err(Errors::InternalServerError),
    }
}

pub fn edit_firstname(
    user_id: i32,
    user_firstname: &str,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<User, Errors> {
    match diesel::update(users.filter(id.eq(user_id)))
        .set(firstname.eq(user_firstname))
        .get_result::<User>(conn)
    {
        Ok(u) => Ok(u),
        Err(_) => Err(Errors::InternalServerError),
    }
}
pub fn edit_lastname(
    user_id: i32,
    user_lastname: &str,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<User, Errors> {
    match diesel::update(users.filter(id.eq(user_id)))
        .set(lastname.eq(user_lastname))
        .get_result::<User>(conn)
    {
        Ok(u) => Ok(u),
        Err(_) => Err(Errors::InternalServerError),
    }
}

pub fn update_password(
    user_id: i32,
    new_password: &str,
    conn: &PooledConnection<ConnectionManager<PgConnection>>
) -> Result<User, Errors> {
    match diesel::update(users.filter(id.eq(user_id)))
        .set(password.eq(new_password))
        .get_result::<User>(conn) {
            Ok(u) => Ok(u),
            Err(_) => Err(Errors::InternalServerError)
        }
}
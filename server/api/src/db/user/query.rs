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
use super::model::User;
use crate::db::schema;
use crate::errors::Errors;
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;
use schema::users::dsl::*;

pub fn get_user_by_uid(
    user_id: &i32,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<User, Errors> {
    let results = match users.filter(id.eq_all(user_id)).limit(5).load::<User>(conn) {
        Ok(u) => u,
        Err(_) => return Err(Errors::InternalServerError),
    };
    if results.len() == 0 {
        return Err(Errors::BadRequest(String::from("No such user")));
    }

    let user = results[0].clone();
    Ok(user)
}

pub fn get_user_by_email(
    user_email: &String,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<User, Errors> {
    let results = match users
        .filter(email.like(user_email))
        .limit(5)
        .load::<User>(conn)
    {
        Ok(u) => u,
        Err(_) => return Err(Errors::InternalServerError),
    };
    if results.len() == 0 {
        return Err(Errors::BadRequest(String::from("No such user")));
    }

    let user = results[0].clone();
    Ok(user)
}

pub fn get_user_by_firstname(
    user_firstname: &String,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<User, Errors> {
    let results = match users
        .filter(firstname.like(user_firstname))
        .limit(5)
        .load::<User>(conn)
    {
        Ok(u) => u,
        Err(_) => return Err(Errors::InternalServerError),
    };
    if results.len() == 0 {
        return Err(Errors::BadRequest(String::from("No such user")));
    }

    let user = results[0].clone();
    Ok(user)
}

pub fn get_user_by_lastname(
    user_lastname: &String,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<User, Errors> {
    let results = match users
        .filter(lastname.like(user_lastname))
        .limit(5)
        .load::<User>(conn)
    {
        Ok(u) => u,
        Err(_) => return Err(Errors::InternalServerError),
    };
    if results.len() == 0 {
        return Err(Errors::BadRequest(String::from("No such user")));
    }

    let user = results[0].clone();
    Ok(user)
}

pub fn get_user_by_username(
    user_username: &String,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<User, Errors> {
    let results = match users
        .filter(username.like(user_username))
        .limit(5)
        .load::<User>(conn)
    {
        Ok(u) => u,
        Err(_) => return Err(Errors::InternalServerError),
    };
    if results.len() == 0 {
        return Err(Errors::BadRequest(String::from("No such user")));
    }

    let user = results[0].clone();
    Ok(user)
}

pub fn is_unique(
    user_firstname: &String,
    user_username: &String,
    user_email: &String,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<bool, Errors> {
    match get_user_by_firstname(user_firstname, conn) {
        Ok(_) => return Ok(false),
        Err(e) => match e {
            Errors::InternalServerError => return Err(Errors::InternalServerError),
            _ => (),
        },
    };

    match get_user_by_username(user_username, conn) {
        Ok(_) => return Ok(false),
        Err(e) => match e {
            Errors::InternalServerError => return Err(Errors::InternalServerError),
            _ => (),
        },
    };

    match get_user_by_email(user_email, conn) {
        Ok(_) => return Ok(false),
        Err(e) => match e {
            Errors::InternalServerError => return Err(Errors::InternalServerError),
            _ => (),
        },
    };

    Ok(true)
}

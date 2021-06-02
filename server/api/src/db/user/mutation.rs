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
use crate::db::schema;
use r2d2::PooledConnection;
use diesel::{ r2d2::ConnectionManager, PgConnection };
use diesel::prelude::*;
use super::model::{ User, NewUser };
use crate::errors::Errors;

pub fn create_user<'a>(
    user_firstname: &'a String,
    user_lastname: &'a String,
    user_username: &'a String,
    user_email: &'a String,
    user_password: &'a String,
    conn: &PooledConnection<ConnectionManager<PgConnection>>
) -> Result<User, Errors> {
    use schema::users;

    let new_user = NewUser {
        firstname: user_firstname,
        lastname: user_lastname,
        username: user_username,
        email: user_email,
        password: user_password
    };

    match diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(conn) {
            Ok(u) => Ok(u),
            Err(_) => Err(Errors::InternalServerError)
        }
}
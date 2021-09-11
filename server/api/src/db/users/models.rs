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

use crate::db::schema::users;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Queryable, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub firstname: String,
    pub nickname: String,
    pub email: String,
    pub profile_pic: Option<String>,
    pub password: String,
    pub joined: NaiveDateTime,
    pub banned: bool,
    pub admin: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub firstname: &'a str,
    pub nickname: &'a str,
    pub email: &'a String,
    pub password: &'a str,
}

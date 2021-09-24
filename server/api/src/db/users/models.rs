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
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Clone, Serialize)]
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

#[derive(Clone, Serialize)]
pub struct SafeUser {
    pub id: Uuid,
    pub username: String,
    pub firstname: String,
    pub nickname: String,
    pub email: String,
    pub profile_pic: Option<String>,
    pub joined: NaiveDateTime,
    pub banned: bool,
    pub admin: bool,
}

impl From<&User> for SafeUser {
    fn from(user: &User) -> Self {
        Self {
            id: user.id,
            username: user.username.clone(),
            firstname: user.firstname.clone(),
            nickname: user.nickname.clone(),
            email: user.email.clone(),
            profile_pic: user.profile_pic.clone(),
            joined: user.joined,
            banned: user.banned,
            admin: user.admin,
        }
    }
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

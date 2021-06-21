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
use chrono::NaiveDateTime;
use super::super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub rating: i32,
    pub joined: NaiveDateTime,
    pub banned: bool,
    pub avatar: String
}

impl Clone for User {
    fn clone(&self) -> User {
        User {
            id: self.id,
            firstname: self.firstname.clone(),
            lastname: self.lastname.clone(),
            username: self.username.clone(),
            password: self.password.clone(),
            email: self.email.clone(),
            rating: self.rating,
            joined: self.joined,
            banned: self.banned,
            avatar: self.avatar.clone()
        }
    }
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub email: &'a str
}
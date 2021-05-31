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
use actix_web_validator::{ Json };
use actix_web::{ web::Json as actix_json, Responder };
use super::payload;
use crate::errors;
use crate::db::user::query::is_unique;
use crate::db::user::mutation::create_user;
use bcrypt::{hash, DEFAULT_COST};

pub async fn registration_handler(req: Json<payload::RegisterRequest>) -> Result<impl Responder, errors::Errors> {
    let is_unique = match is_unique(
        &req.firstname,
        &req.username,
        &req.email
    ) {
        Ok(u) => u,
        Err(e) => return Err(e)
    };

    if !is_unique {
        return Err(errors::Errors::BadRequest(String::from("Some user data is not unique")));
    }

    let salted_password = match hash(&req.password, DEFAULT_COST) {
        Ok(pwd) => pwd,
        Err(_) => return Err(errors::Errors::InternalServerError)
    };

    let user = create_user(
        &req.firstname,
        &req.lastname, 
        &req.username, 
        &req.email, 
        &salted_password
    );

    match user {
        Ok(_) => Ok(actix_json(payload::RegisterReturnPayload {
            success: true
        })),
        Err(e) => Err(e)
    }
}
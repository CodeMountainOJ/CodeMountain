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
use actix_web_validator::Json;
use actix_web::{ web::Json as actix_json, web::Data, Responder };
use super::payload;
use crate::errors;
use crate::db::user::query::is_unique;
use crate::db::user::mutation::create_user;
use crate::db::Pool;
use bcrypt::hash;

pub async fn registration_handler(conn_pool: Data<Pool>, req: Json<payload::RegisterRequest>) -> Result<impl Responder, errors::Errors> {
    let req_firstname = req.firstname.trim_start().trim_end();
    let req_lastname = req.lastname.trim_start().trim_end();
    let req_username = req.username.trim_start().trim_end();
    let req_email = req.email.as_str();

    let is_unique = is_unique(
        req_firstname,
        req_username,
        req_email,
        conn_pool.as_ref()
    )?;

    if !is_unique {
        return Err(errors::Errors::BadRequest("Some user data is not unique"));
    }

    let salted_password = hash(&req.password, 7).map_err(|_| errors::Errors::InternalServerError)?;

    create_user(
        req_firstname,
        req_lastname, 
        req_username, 
        req_email, 
        &salted_password,
        &conn_pool
    ).map_or_else(Err, |_| Ok(actix_json(payload::ReturnStatus {
        success: true
    })))
}
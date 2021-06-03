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
use actix_web::{ web::Data, web::Json as actix_json, Responder };
use actix_web_validator::Json;
use crate::db::user::query::get_user_by_firstname;
use crate::errors::Errors;
use crate::db::Pool;
use crate::extractors::auth::AuthRequired;
use crate::db::user::mutation::edit_firstname;
use super::payload::FirstNamePayload;
use super::payload::SuccessPayload;

pub async fn edit_firstname_handler(conn_pool: Data<Pool>, user: AuthRequired, req: Json<FirstNamePayload>) -> Result<impl Responder, Errors> {
    let conn = match conn_pool.get() {
        Ok(p) => p,
        Err(_) => return Err(Errors::InternalServerError)
    };

    let user_id = user.user.id;
    let new_firstname = req.firstname.clone();

    // check if new firstname is unique
    match get_user_by_firstname(&new_firstname, &conn) {
        Ok(_) => return Err(Errors::BadRequest(String::from("Firstname is not unique!"))),
        Err(_) => () // unique
    }

    match edit_firstname(user_id, &new_firstname, &conn) {
        Ok(_) => return Ok(actix_json(SuccessPayload {
            success: true
        })),
        Err(_) => return Err(Errors::InternalServerError)
    }
}
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
use super::payload::{ LoginRequest, LoginTokens };
use crate::errors;
use crate::db::user::query::get_user_by_email;
use crate::db::Pool;
use bcrypt::verify;
use crate::jwt::sign::{ generate_accesstoken, generate_refreshtoken };

pub async fn login_handler(conn_pool: Data<Pool>, req: Json<LoginRequest>) -> Result<impl Responder, errors::Errors> {
    let conn = match conn_pool.get() {
        Ok(p) => p,
        Err(_) => return Err(errors::Errors::InternalServerError)
    };
    
    let user = match get_user_by_email(&req.email, &conn) {
        Ok(u) => u,
        Err(e) => return Err(e)
    };
    let raw_password = req.password.clone();
    let salted_password = user.password;

    let is_correct_password = match verify(raw_password, &salted_password) {
        Ok(s) => s,
        Err(_) => return Err(errors::Errors::InternalServerError)
    };

    if is_correct_password {
        let access_token = match generate_accesstoken(&user.id) {
            Ok(token) => token,
            Err(_) => return Err(errors::Errors::InternalServerError)
        };

        let refresh_token = match generate_refreshtoken(&user.id) {
            Ok(token) => token,
            Err(_) => return Err(errors::Errors::InternalServerError)
        };

        return Ok(actix_json(LoginTokens {
            access_token,
            refresh_token
        }));
    }

    Err(errors::Errors::BadRequest(String::from("Wrong password!")))
}
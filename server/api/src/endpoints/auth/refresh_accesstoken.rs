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
use actix_web::{ Responder, web::Json as actix_json, web::Data};
use super::payload::{ RefreshAccessTokenPayload, RefreshAccessTokenReturnPayload };
use crate::db::Pool;
use crate::db::user::query::get_user_by_uid;
use crate::jwt::claims::TokenType;
use crate::jwt::sign::generate_accesstoken;
use crate::jwt::verify::verify_token;
use crate::errors::Errors;

pub async fn refresh_accesstoken_handler(conn_pool: Data<Pool>, payload: actix_json<RefreshAccessTokenPayload>) -> Result<impl Responder, Errors> {
    let token = &payload.refresh_token;
    let payload = verify_token(token).map_err(|_| Errors::BadRequest("Invalid token"))?;

    match payload.token_type {
        TokenType::RefreshToken => (),
        _ => return Err(Errors::BadRequest("Not a refresh token"))
    }

    let uid = payload.uid;

    get_user_by_uid(&uid, conn_pool.as_ref()).map_err(|_| Errors::BadRequest("Invalid or malformed token"))?;

    let access_token = generate_accesstoken(&uid).map_err(|_| Errors::InternalServerError)?;

    Ok(actix_json(RefreshAccessTokenReturnPayload {
        access_token
    }))
}
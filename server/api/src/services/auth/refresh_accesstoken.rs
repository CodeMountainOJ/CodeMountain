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
use serde::{Serialize, Deserialize};
use crate::jwt::refreshtoken::verify_refreshtoken;
use crate::jwt::accesstoken::generate_accesstoken;
use uuid::Uuid;
use crate::errors::Errors;
use actix_web::Responder;
use actix_web::web::Json;

#[derive(Deserialize)]
pub struct RefreshAccessTokenPayload {
    pub refresh_token: String
}

#[derive(Serialize)]
struct AccessTokenPayload {
    pub access_token: String
}

pub async fn refresh_accesstoken_handler(payload: Json<RefreshAccessTokenPayload>) -> Result<impl Responder, Errors> {
    // no need to check for user's existence
    let claims = verify_refreshtoken(&payload.refresh_token)?;
    Ok(Json(AccessTokenPayload {
        access_token: generate_accesstoken(&claims.user_id.parse::<Uuid>().map_err(|e| Errors::BadRequest(e.to_string()))?)?
    }))
}

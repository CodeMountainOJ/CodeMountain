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

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use super::claims::{Token, TokenType};
use crate::{config::get, errors::Errors};

pub fn generate_refreshtoken(user_id: &Uuid) -> Result<String, Errors> {
    let claims = Token {
        user_id: user_id.to_string(),
        token_type: TokenType::RefreshToken,
        exp: (chrono::Utc::now() + chrono::Duration::days(7)).timestamp(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get::<String>("JWT_SECRET_KEY").as_ref()),
    )
    .map_err(|_| Errors::InternalServerError)
}

pub fn verify_refreshtoken(token: &str) -> Result<Token, Errors> {
    decode::<Token>(
        token,
        &DecodingKey::from_secret(get::<String>("JWT_SECRET_KEY").as_ref()),
        &Validation::default(),
    )
    .map_or_else(
        |e| Err(Errors::BadRequest(e.to_string())),
        |v| match v.claims.token_type {
            TokenType::RefreshToken => Ok(v.claims),
            _ => Err(Errors::BadRequest(String::from("Invalid token!"))),
        },
    )
}

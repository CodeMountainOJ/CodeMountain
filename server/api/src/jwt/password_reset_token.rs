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
use crate::errors::Errors;
use crate::jwt::claims::{Token, TokenType};
use jsonwebtoken::{
    dangerous_insecure_decode, decode, encode, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};
use uuid::Uuid;

pub fn generate_password_reset_token(user_id: &Uuid, secret: &str) -> Result<String, Errors> {
    let claims = Token {
        user_id: user_id.to_string(),
        token_type: TokenType::PasswordResetToken,
        exp: (chrono::Utc::now() + chrono::Duration::minutes(5)).timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| Errors::InternalServerError)
}

pub fn get_reset_token_data(token: &str) -> Result<Token, Errors> {
    dangerous_insecure_decode(token).map_or_else(
        |e| Err(Errors::BadRequest(e.to_string())),
        |v: TokenData<Token>| match v.claims.token_type {
            TokenType::PasswordResetToken => Ok(v.claims),
            _ => Err(Errors::BadRequest(String::from("Invalid token!"))),
        },
    )
}

pub fn verify_password_reset_token(token: &str, secret: &str) -> Result<Token, Errors> {
    decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_or_else(
        |e| Err(Errors::BadRequest(e.to_string())),
        |v: TokenData<Token>| match v.claims.token_type {
            TokenType::PasswordResetToken => Ok(v.claims),
            _ => Err(Errors::BadRequest(String::from("Invalid token!"))),
        },
    )
}

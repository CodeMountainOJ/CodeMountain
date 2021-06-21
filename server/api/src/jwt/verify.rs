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
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation, dangerous_insecure_decode, decode, errors::Result, errors::Error};
use super::claims::Token;
use std::env::var;

type TokenReturnType = std::result::Result<Token, Error>;

fn verify(token: &str) -> Result<TokenData<Token>> {
    decode::<Token>(token, &DecodingKey::from_secret(
        var("JWT_SECRET_KEY").unwrap().as_bytes()
    ), &Validation::new(Algorithm::HS256))
}

fn verify_wcustom_secret(token: &str, secret: &str) -> Result<TokenData<Token>> {
    decode::<Token>(token, &DecodingKey::from_secret(
        secret.as_bytes()
    ), &Validation::new(Algorithm::HS256))
}

pub fn verify_token(token: &str) -> TokenReturnType {
    match verify(token) {
        Ok(r) => Ok(r.claims),
        Err(e) => Err(e)
    }
}
pub fn verify_token_using_custom_secret(token: &str, secret: &str) -> TokenReturnType {
    match verify_wcustom_secret(token, secret) {
        Ok(r) => Ok(r.claims),
        Err(e) => Err(e)
    }
}

pub fn decode_without_secret(token: &str) -> TokenReturnType {
    match dangerous_insecure_decode(token) {
        Ok(t) => Ok(t.claims),
        Err(e) => Err(e)
    }
}
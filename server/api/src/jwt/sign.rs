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
use super::claims::Token;
use jsonwebtoken::{encode, errors::Result, EncodingKey, Header};

fn sign(claims: impl serde::Serialize) -> Result<String> {
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            std::env::var("JWT_SECRET_KEY")
                .expect("JWT_SECRET_KEY should be in the env")
                .as_bytes(),
        ),
    )
}

pub fn generate_accesstoken(uid: &i32) -> Result<String> {
    let claims = Token {
        uid: uid.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::minutes(5)).timestamp(),
    };

    sign(claims)
}

pub fn generate_refreshtoken(uid: &i32) -> Result<String> {
    let claims = Token {
        uid: uid.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::days(30)).timestamp(),
    };

    sign(claims)
}

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
use actix_web::{ FromRequest, HttpRequest, dev };
use crate::db::Pool;
use crate::db::user::query::get_user_by_uid;
use crate::errors::Errors;
use crate::db::user::model::User;
use crate::jwt::verify::verify_token;
use crate::jwt::claims::TokenType;
use futures_util::future::{ ok, err, Ready };

pub struct AuthRequired {
    pub user: User
}

impl FromRequest for AuthRequired {
    type Error = Errors;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();


    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let token_hv = match req.headers().get("authorization") {
            Some(t) => t,
            None => return err(Errors::BadRequest(String::from("No authorization headers included!"))),
        };

        let token_str = match token_hv.to_str() {
            Ok(t) => t.to_string(),
            Err(_) => return err(Errors::BadRequest(String::from("Invalid authorization header value!")))
        };

        let token = match verify_token(&token_str.to_string()) {
            Ok(t) => t,
            Err(_) => return err(Errors::AccessForbidden)
        };

        match token.token_type {
            TokenType::AccessToken => (),
            _ => return err(Errors::AccessForbidden)
        };

        let conn_pool = match req.app_data::<Pool>() {
            Some(p) => p,
            None => return err(Errors::InternalServerError)
        };

        let conn = match conn_pool.get() {
            Ok(c) => c,
            Err(_) => return err(Errors::InternalServerError)
        };

        let user = match get_user_by_uid(&token.uid, &conn) {
            Ok(u) => u,
            Err(e) => {
                match e {
                    Errors::BadRequest(_) => return err(Errors::AccessForbidden),
                    _ => return err(Errors::InternalServerError)
                }
            }
        };

        ok(AuthRequired {
            user
        })
    }

    fn extract(req: &HttpRequest) -> Self::Future {
        Self::from_request(req, &mut dev::Payload::None)
    }

    fn configure<F>(f: F) -> Self::Config
    where
        F: FnOnce(Self::Config) -> Self::Config,
    {
        f(Self::Config::default())
    }
}

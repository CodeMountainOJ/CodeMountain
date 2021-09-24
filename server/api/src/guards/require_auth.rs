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
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use crate::errors::Errors;
use futures_util::future::{err, ok, Ready};
use crate::jwt::accesstoken::verify_access_token;
use crate::config::get;
use actix_web::web::Data;
use crate::db::Pool;
use crate::db::users::query::get_user;
use crate::db::users::models::User;

pub struct RequireAuth {
    pub user: User,
}

impl FromRequest for RequireAuth {
    type Error = Errors;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let bearer = match req.headers().get("authorization") {
            Some(t) => t,
            None => return err(Errors::AccessForbidden)
        }
            .to_str()
            .unwrap();

        if bearer.is_empty() {
            return err(Errors::AccessForbidden)
        }

        let bearer = bearer.split_ascii_whitespace()
            .collect::<Vec<&str>>();
        if bearer.len() != 2 {
            return err(Errors::AccessForbidden);
        }

        let raw_token = bearer[1].clone();
        let token = match verify_access_token(raw_token, &get::<String>("JWT_SECRET_KEY")) {
            Ok(tv) => tv,
            Err(_) => return err(Errors::AccessForbidden)
        };

        let conn_pool = match req.app_data::<Data<Pool>>() {
            Some(c) => c,
            None => return err(Errors::InternalServerError)
        };

        let user = match get_user(conn_pool.as_ref(), token.user_id.parse().unwrap()) {
            Ok(u) => u,
            Err(_) => return err(Errors::AccessForbidden)
        };

        ok(RequireAuth {
            user
        })
    }
}
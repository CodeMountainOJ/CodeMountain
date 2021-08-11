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
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum Errors {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(&'static str),

    #[display(fmt = "Access Forbidden")]
    AccessForbidden,
}
impl ResponseError for Errors {
    fn status_code(&self) -> StatusCode {
        match *self {
            Errors::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Errors::BadRequest(_) => StatusCode::BAD_REQUEST,
            Errors::AccessForbidden => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> HttpResponse {
        #[derive(serde::Serialize)]
        struct Ret {
            pub success: bool,
            pub error: &'static str,
        }

        match self {
            Errors::InternalServerError => HttpResponse::InternalServerError().json(Ret {
                success: false,
                error: "Internal Server Error",
            }),

            Errors::BadRequest(message) => HttpResponse::BadRequest().json(Ret {
                success: false,
                error: message,
            }),
            Errors::AccessForbidden => HttpResponse::Forbidden().json(Ret {
                success: false,
                error: "Access forbidden",
            }),
        }
    }
}

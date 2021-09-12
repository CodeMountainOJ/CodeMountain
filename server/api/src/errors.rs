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
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::Serialize;

#[derive(Display, Debug)]
pub enum Errors {
    #[display(fmt = "Internal server error")]
    InternalServerError,

    #[display(fmt = "Bad request. Reason: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Requested resource was not found")]
    NotFound,

    #[display(fmt = "Access forbidden")]
    AccessForbidden,
}
impl ResponseError for Errors {
    fn status_code(&self) -> StatusCode {
        match *self {
            Errors::BadRequest(_) => StatusCode::BAD_REQUEST,
            Errors::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Errors::AccessForbidden => StatusCode::FORBIDDEN,
            Errors::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        #[derive(Serialize)]
        struct Ret {
            pub success: bool,
            pub error: String,
        }

        match self {
            Errors::InternalServerError => HttpResponse::InternalServerError().json(Ret {
                success: false,
                error: String::from("Internal server error"),
            }),
            Errors::AccessForbidden => HttpResponse::Forbidden().json(Ret {
                success: false,
                error: String::from("Access forbidden"),
            }),
            Errors::BadRequest(error) => HttpResponse::BadRequest().json(Ret {
                success: false,
                error: error.clone(),
            }),
            Errors::NotFound => HttpResponse::NotFound().json(Ret {
                success: false,
                error: String::from("Requested resource was not found"),
            }),
        }
    }
}

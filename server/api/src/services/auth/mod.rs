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
use actix_web::{
    web::{post, scope},
    Scope,
};

mod login;
mod password_reset;
mod refresh_accesstoken;
mod register;

pub fn get_auth_service() -> Scope {
    scope("/auth")
        .route("/login", post().to(login::login_handler))
        .route("/register", post().to(register::register_handler))
        .route(
            "/refresh_accesstoken",
            post().to(refresh_accesstoken::refresh_accesstoken_handler),
        )
        .route(
            "/forgot_password",
            post().to(password_reset::reset_password_request_handler),
        )
        .route(
            "/verify_reset_token",
            post().to(password_reset::verify_reset_token_handler),
        )
        .route(
            "/reset_password",
            post().to(password_reset::reset_password_handler),
        )
}

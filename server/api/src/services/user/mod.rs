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
use actix_web::web::{post, scope};
use actix_web::Scope;

mod query;
mod update;

pub fn get_user_service() -> Scope {
    scope("/user")
        .service(
            scope("/query")
                .route("/id", post().to(query::get_user_by_id))
                .route("/", post().to(query::get_user_by_query)), // f u
        )
        .service(
            scope("/update")
                .route("/firstname", post().to(update::update_firstname_handler))
                .route("/nickname", post().to(update::update_nickname_handler))
                .route("/email", post().to(update::update_email_handler))
                .route("/password", post().to(update::update_password_handler)),
        )
}

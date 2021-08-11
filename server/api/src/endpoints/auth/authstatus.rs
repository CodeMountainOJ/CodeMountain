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

use crate::endpoints::auth::payload::AuthStatusUserReturnPayload;
use crate::errors::Errors;
use crate::guards::auth::AuthRequired;
use actix_web::{web::Json, Responder};

pub async fn check_auth_status_handler(user: AuthRequired) -> Result<impl Responder, Errors> {
    Ok(Json(AuthStatusUserReturnPayload { user: user.user }))
}

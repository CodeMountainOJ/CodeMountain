/*
 *  CodeMountain is a free and open source online judge open for everyone
 *  Copyright (C) 2021 Uthsob Chakra Borty and contributors
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
use super::payload::LastNamePayload;
use super::payload::SuccessPayload;
use crate::db::user::mutation::edit_lastname;
use crate::db::Pool;
use crate::errors::Errors;
use crate::extractors::auth::AuthRequired;
use actix_web::{web::Data, web::Json as actix_json, Responder};
use actix_web_validator::Json;

pub async fn edit_lastname_handler(
    conn_pool: Data<Pool>,
    user: AuthRequired,    req: Json<LastNamePayload>,
) -> Result<impl Responder, Errors> {
    let conn = match conn_pool.get() {
        Ok(p) => p,
        Err(_) => return Err(Errors::InternalServerError),
    };

    let user_id = user.user.id;
    let new_lastname = req.lastname.clone();

    match edit_lastname(user_id, &new_lastname, &conn) {
        Ok(_) => return Ok(actix_json(SuccessPayload { success: true })),
        Err(_) => return Err(Errors::InternalServerError),
    }
}

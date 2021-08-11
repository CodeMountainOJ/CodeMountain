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

use crate::db::user::mutation::update_avatar;
use crate::db::Pool;
use crate::endpoints::user::payload::ReturnPayloadUpdateAvatar;
use crate::errors::Errors;
use crate::guards::auth::AuthRequired;
use crate::image_validation::validate_img;
use actix_multipart::Multipart;
use actix_web::web::Json;
use actix_web::Responder;
use futures_util::{StreamExt, TryStreamExt};
use std::io::Write;

// Handles avatar updating things
// NOTE: this handler does not have automated tests and have to be manually tested
pub async fn update_avatar_handler(
    mut payload: Multipart,
    user: AuthRequired,
    conn_pool: actix_web::web::Data<Pool>,
) -> Result<impl Responder, Errors> {
    let mut filename = String::new();
    let user_id = user.user.id;

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        filename = uuid::Uuid::new_v4().to_string()
            + &sanitize_filename::sanitize(&content_type.get_filename().unwrap());

        let filepath = std::path::Path::new(&std::env::var("USER_SUBMITTED_FILE_PATH").unwrap())
            .join(&sanitize_filename::sanitize(&filename));

        let mut file = actix_web::web::block(move || std::fs::File::create(filepath))
            .await
            .map_err(|_| Errors::InternalServerError)?;

        let mut final_buf: Vec<u8> = Vec::new();

        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|_| Errors::BadRequest("Invalid multipart chunk"))?;

            final_buf.extend(data);
        }

        if !validate_img(&final_buf) {
            return Err(Errors::BadRequest("Invalid file!"));
        }

        actix_web::web::block(move || file.write(&final_buf))
            .await
            .map_err(|_| Errors::InternalServerError)?;
    }

    // try removing the previous avatar and replacing it with the new one :^)
    let _ = std::fs::remove_file(
        std::path::Path::new(&std::env::var("USER_SUBMITTED_FILE_PATH").unwrap())
            .join(&user.user.avatar),
    );

    update_avatar(user_id, &filename, &conn_pool)?;

    Ok(Json(ReturnPayloadUpdateAvatar {
        new_avatar: filename,
    }))
}

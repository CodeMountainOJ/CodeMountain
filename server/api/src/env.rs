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
use std::env::var;

pub fn do_env_check() {
    var("REDIS_CON").expect("REDIS_CON is not set");
    var("SMTP_EMAIL").expect("SMTP_EMAIL is not set");
    var("SMTP_PASSWORD").expect("SMTP_PASSWORD is not set");
    var("SMTP_SERVER").expect("SMTP_SERVER is not set");
    var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY is not set");
    var("DATABASE_URL").expect("DATABASE_URL is not set");
    var("USER_SUBMITTED_FILE_PATH").expect("USER_SUBMITTED_FILE_PATH is not set");
}

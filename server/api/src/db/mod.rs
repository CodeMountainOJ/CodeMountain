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
pub mod schema;
pub mod user;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::Error;
use r2d2::PooledConnection;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_pool() -> Pool {
    let database_url = env::var("DATABASE_URL").unwrap();

    let connection_manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(connection_manager)
        .expect("Failed to create database connection pool")
}

pub fn get_conn(
    conn_pool: &Pool,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
    conn_pool.get()
}

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

use config::Config;
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    static ref CONFIGS: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings
            .merge(config::File::with_name("config.toml"))
            .unwrap();

        settings
    });
}

pub fn get<'a, T: serde::Deserialize<'a>>(key: &str) -> T {
    CONFIGS.read().unwrap().get::<T>(key).unwrap()
}

/// Checks config for required values
///
/// # Panics
/// Panics when required value is not found
pub fn run_config_check() {
    get::<String>("DATABASE_URL");
    get::<String>("JWT_SECRET_KEY");
    get::<String>("LISTENING_URL");
}

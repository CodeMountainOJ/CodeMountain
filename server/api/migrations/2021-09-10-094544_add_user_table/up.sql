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

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    "id"            UUID PRIMARY KEY            DEFAULT uuid_generate_v1mc(),
    "username"      VARCHAR(25)                 NOT NULL UNIQUE,
    "firstname"     VARCHAR(255)                NOT NULL,
    "nickname"      VARCHAR(255)                NOT NULL,
    "email"         VARCHAR(255)                NOT NULL UNIQUE,
    "profile_pic"   VARCHAR(255),
    "password"      VARCHAR(255)                NOT NULL,
    "joined"        TIMESTAMP NOT NULL          DEFAULT current_timestamp,
    "banned"        BOOLEAN   NOT NULL          DEFAULT false,
    "admin"         BOOLEAN   NOT NULL          DEFAULT false
) WITH (
    OIDS=false
);

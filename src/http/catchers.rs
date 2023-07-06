/*
 * Copyright (C) 2022 - This file is part of "japp".
 *
 * "japp" is free software: you can redistribute it and/or modify it under the
 *  terms of version 3 of the GNU Affero General Public License as published by the
 *  Free Software Foundation.
 *
 * "japp" is distributed in the hope that it will be useful, but WITHOUT ANY
 *  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 *  FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more
 *   details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with japp.  If not, see http://www.gnu.org/licenses/.
 */

use std::path::Path;
use rocket::fs::NamedFile;

#[catch(404)]
pub async fn not_found() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/404.html")).await.ok()
}

#[catch(401)]
pub fn unauthorized() -> &'static str {
    "Unauthorized 🎅"
}

#[catch(500)]
pub fn internal_server_error() -> &'static str {
    "Internal Server Error 😭"
}

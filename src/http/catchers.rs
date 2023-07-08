/*
 * Copyright (C) 2023 - This file is part of "JAPP".
 * "JAPP" is free software: you can redistribute it and/or modify it under the
 * terms of version 3 of the GNU Affero General Public License as published by the
 * Free Software Foundation.
 * "JAPP" is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more
 * details.
 * You should have received a copy of the GNU Affero General Public License
 * long with JAPP.  If not, see http://www.gnu.org/licenses/.
 */

use crate::application::static_folder::StaticFolder;
use rocket::fs::NamedFile;
use rocket::Request;

#[catch(404)]
pub async fn not_found(req: &Request<'_>) -> Option<NamedFile> {
    let static_folder = req
        .rocket()
        .state::<StaticFolder>()
        .expect("StaticFolder is not in state");
    NamedFile::open(static_folder.0.join("404.html")).await.ok()
}

#[catch(401)]
pub fn unauthorized() -> &'static str {
    "Unauthorized 🎅"
}

#[catch(500)]
pub fn internal_server_error() -> &'static str {
    "Internal Server Error 😭"
}

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
use rocket::fs::{FileServer, NamedFile};
use rocket::{get, routes, Route, State};
use std::path::PathBuf;

#[get("/")]
async fn index(static_folder: &State<StaticFolder>) -> Option<NamedFile> {
    NamedFile::open(static_folder.0.join("landing.html"))
        .await
        .ok()
}

#[get("/imprint")]
async fn imprint(static_folder: &State<StaticFolder>) -> Option<NamedFile> {
    NamedFile::open(static_folder.0.join("imprint.html"))
        .await
        .ok()
}

#[get("/data-privacy")]
async fn data_privacy(static_folder: &State<StaticFolder>) -> Option<NamedFile> {
    NamedFile::open(static_folder.0.join("data-privacy.html"))
        .await
        .ok()
}

#[get("/app/<_..>")]
async fn app(static_folder: &State<StaticFolder>) -> Option<NamedFile> {
    NamedFile::open(static_folder.0.join("index.html"))
        .await
        .ok()
}

pub struct Static {}

impl Static {
    pub fn routes() -> Vec<Route> {
        routes![index, imprint, data_privacy, app]
    }

    pub fn files(path: &PathBuf) -> FileServer {
        FileServer::from(path).rank(2)
    }
}

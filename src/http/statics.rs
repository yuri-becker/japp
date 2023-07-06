/*
 * Copyright (C) 2022 - This file is part of "JAPP".
 *
 * "JAPP" is free software: you can redistribute it and/or modify it under the
 *  terms of version 3 of the GNU Affero General Public License as published by the
 *  Free Software Foundation.
 *
 * "JAPP" is distributed in the hope that it will be useful, but WITHOUT ANY
 *  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 *  FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more
 *   details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with JAPP.  If not, see http://www.gnu.org/licenses/.
 */
use rocket::{fs::NamedFile, fs::relative, get, routes, Route};
use std::path::Path;
use rocket::fs::FileServer;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/landing.html")).await.ok()
}

#[get("/imprint")]
async fn imprint() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/imprint.html")).await.ok()
}

#[get("/data-privacy")]
async fn data_privacy() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/data-privacy.html"))
        .await
        .ok()
}

#[get("/app/<_..>")]
async fn app() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

pub struct Static {}

impl Static {
    pub fn routes() -> Vec<Route> {
        routes![index, imprint, data_privacy, app]
    }

    pub fn files() -> FileServer {
        FileServer::from(relative!("static")).rank(2)
    }
}


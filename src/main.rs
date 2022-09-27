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

#[macro_use]
extern crate rocket;
extern crate core;

use crate::application::database::MongoDb;
use api::routes as api_routes;
use rocket::fs::NamedFile;
use rocket::{catch, fs::relative, fs::FileServer};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use std::path::Path;
use web_router::routes as web_routes;

mod api;
mod application;
mod domain;
mod usecase;
mod web_router;

#[catch(404)]
async fn not_found() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/404.html")).await.ok()
}

#[catch(401)]
fn unauthorized() -> &'static str {
    "Unauthorized 🎅"
}

#[catch(500)]
fn internal_server_error() -> &'static str {
    "Internal Server Error 😭"
}

fn docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/api/openapi.json".to_string(),
        ..Default::default()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(MongoDb)
        .mount("/api/swagger", make_swagger_ui(&docs()))
        .mount("/api", api_routes())
        .mount("/", web_routes())
        .mount("/", FileServer::from(relative!("static")).rank(-1))
        .register(
            "/",
            catchers![not_found, internal_server_error, unauthorized],
        )
}

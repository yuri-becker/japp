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
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket::tokio::sync::broadcast::channel;
use crate::http::catchers::{not_found, unauthorized, internal_server_error};
use crate::http::events::{Events, Message};
use crate::http::session::SessionApi;
use crate::http::statics::Static;

mod application;
mod domain;
mod http;
mod usecase;


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
        .manage(channel::<Message>(1024).0)
        .mount("/", Static::files())
        .mount("/", Static::routes())
        .mount("/api/swagger", make_swagger_ui(&docs()))
        .mount("/api/session", SessionApi::routes())
        .mount("/api/events", Events::routes())
        .register("/", catchers![not_found, internal_server_error, unauthorized])
}

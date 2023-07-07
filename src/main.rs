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

use crate::http::catchers::{internal_server_error, not_found, unauthorized};
use crate::http::events::{Events, Message};
use crate::http::session::SessionApi;
use crate::http::statics::Static;
use application::static_folder::StaticFolder;
use domain::session::SessionRepository;
use std::path::PathBuf;
use mongodb::Database;
use rocket::Config;
use rocket::config::SecretKey;
use rocket::tokio::sync::broadcast::channel;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use shuttle_rocket::ShuttleRocket;
use shuttle_secrets::SecretStore;

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

#[shuttle_runtime::main]
async fn rocket(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::MongoDb(local_uri = "mongodb://localhost/japp")] mongo_db: Database,
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> ShuttleRocket {
    let config = Config {
        secret_key: SecretKey::from(secret_store.get("ROCKET_SECRET").expect("Please define secret ROCKET_SECRET").as_bytes()),
        ..Config::default()
    };
    let rocket = rocket::custom(config)
        .manage(SessionRepository::from(&mongo_db))
        .manage(channel::<Message>(1024).0)
        .manage(StaticFolder(static_folder.clone()))
        .mount("/", Static::files(&static_folder))
        .mount("/", Static::routes())
        .mount("/api/swagger", make_swagger_ui(&docs()))
        .mount("/api/session", SessionApi::routes())
        .mount("/api/events", Events::routes())
        .register(
            "/",
            catchers![not_found, internal_server_error, unauthorized],
        );
    Ok(rocket.into())
}

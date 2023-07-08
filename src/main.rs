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

#[macro_use]
extern crate rocket;
extern crate core;

use crate::http::catchers::{internal_server_error, not_found, unauthorized};
use crate::http::statics::Static;
use application::static_folder::StaticFolder;
use domain::session::SessionRepository;
use events::message::Message;
use http::api::session::SessionController;
use mongodb::Database;
use rocket::config::SecretKey;
use rocket::tokio::sync::broadcast::channel;
use rocket::Config;
use rocket_okapi::mount_endpoints_and_merged_docs;
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use shuttle_rocket::ShuttleRocket;
use shuttle_secrets::SecretStore;
use std::path::PathBuf;

mod application;
mod domain;
mod events;
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
        secret_key: SecretKey::from(
            secret_store
                .get("ROCKET_SECRET")
                .expect("Please define secret ROCKET_SECRET")
                .as_bytes(),
        ),
        ..Config::default()
    };
    let rocket = rocket::custom(config);
    let rocket = rocket.manage(SessionRepository::from(&mongo_db));
    let rocket = rocket.manage(channel::<Message>(1024).0);
    let rocket = rocket.manage(StaticFolder(static_folder.clone()));
    let rocket = rocket.mount("/", Static::files(&static_folder));
    let rocket = rocket.mount("/", Static::routes());
    let mut rocket = rocket.mount("/api/session", SessionController::event_stream_routes());
    let openapi_settings = OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
        rocket,
        "/api".to_owned(),
        openapi_settings,
        "/session" => SessionController::restful_routes(),
    };
    let rocket = rocket.mount("/api/swagger", make_swagger_ui(&docs()));
    let rocket = rocket.register(
        "/",
        catchers![not_found, internal_server_error, unauthorized],
    );
    Ok(rocket.into())
}

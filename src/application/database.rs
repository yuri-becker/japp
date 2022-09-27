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

use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use mongodb::{Client, Database};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::serde::Deserialize;
use rocket::{Build, Rocket};

static mut DATABASE: Option<Database> = None;

pub fn database() -> &'static Database {
    unsafe { DATABASE.as_ref().unwrap() }
}

#[derive(Deserialize, Debug, PartialEq)]
struct MongoConfig {
    mongo_url: String,
    mongo_database: String,
}

pub struct MongoDb;
#[rocket::async_trait]
impl Fairing for MongoDb {
    fn info(&self) -> Info {
        Info {
            name: "Connect to MongoDB",
            kind: Kind::Ignite,
        }
    }
    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        let config: MongoConfig = rocket.figment().extract().expect("Configuration missing");
        let options = ClientOptions::parse(config.mongo_url).await.unwrap();
        let client = Client::with_options(options).expect("Could not connect to MongoDB");
        unsafe {
            DATABASE = Some(client.database(&config.mongo_database));
        }
        Ok(rocket)
    }
}

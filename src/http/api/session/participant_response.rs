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

use crate::domain::session::Participant;
use rocket::serde::Serialize;
use rocket_okapi::JsonSchema;
use serde::Deserialize;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct ParticipantResponse {
    pub id: String,
    pub name: Option<String>,
    pub estimating: Option<bool>,
    pub away: Option<bool>,
}

impl From<Participant> for ParticipantResponse {
    fn from(it: Participant) -> Self {
        ParticipantResponse {
            id: it.id.unwrap().to_hex(),
            name: it.name,
            estimating: it.estimating,
            away: it.away,
        }
    }
}

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

use super::participant_response::ParticipantResponse;
use crate::domain::session::Session;
use rocket::serde::Serialize;
use rocket_okapi::JsonSchema;
use serde::Deserialize;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct SessionResponse {
    pub name: String,
    pub participant: Vec<ParticipantResponse>,
    pub scale: Vec<String>,
}

impl From<Session> for SessionResponse {
    fn from(value: Session) -> Self {
        SessionResponse {
            name: value.name,
            participant: value.participants.into_iter().map(|it| it.into()).collect(),
            scale: value.scale,
        }
    }
}

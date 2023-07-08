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

use crate::http::api::session::session_response::SessionResponse;
use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Receiver {
    Participant {
        session: ObjectId,
        participant: ObjectId,
    },
    Session(ObjectId),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Payload {
    SessionInit { session: SessionResponse },
    ParticipantJoined { id: String, name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub receiver: Receiver,
    pub payload: Payload,
}

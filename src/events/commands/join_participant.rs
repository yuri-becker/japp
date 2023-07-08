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

use crate::domain::session::{Participant, SessionRepository};
use crate::events::command::Command;
use crate::events::message::{Message, Payload, Receiver};
use mongodb::bson::oid;
use mongodb::error::Error;

pub struct JoinParticipant<'a> {
    pub participant_id: String,
    pub participant_name: String,
    pub session_id: String,
    pub session_repository: &'a SessionRepository,
}

#[async_trait]
impl Command for JoinParticipant<'_> {
    async fn domain_update(&self) -> Result<(), Error> {
        self.session_repository
            .update_participant(
                self.session_id.as_str(),
                &Participant {
                    id: Option::Some(
                        oid::ObjectId::parse_str(self.participant_id.to_string()).unwrap(),
                    ),
                    name: Option::Some(self.participant_name.to_string()),
                    ..Participant::default()
                },
            )
            .await
    }

    fn event(&self) -> Message {
        Message {
            receiver: Receiver::Session(
                oid::ObjectId::parse_str(self.session_id.to_string()).unwrap(),
            ),
            payload: Payload::ParticipantJoined {
                id: self.participant_id.to_string(),
                name: self.participant_name.to_string(),
            },
        }
    }
}

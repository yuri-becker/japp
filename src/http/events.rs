/*
 * Copyright (C) 2022 - This file is part of "japp".
 *
 * "japp" is free software: you can redistribute it and/or modify it under the
 *  terms of version 3 of the GNU Affero General Public License as published by the
 *  Free Software Foundation.
 *
 * "japp" is distributed in the hope that it will be useful, but WITHOUT ANY
 *  WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 *  FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more
 *   details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with japp.  If not, see http://www.gnu.org/licenses/.
 */

use rocket::tokio::sync::broadcast::{Sender, error::RecvError};
use rocket::tokio::select;
use mongodb::bson::oid::ObjectId;
use rocket::response::stream::{EventStream, Event};
use rocket::{Route, Shutdown, State, routes};
use rocket::futures::Stream;
use crate::application::participant_id::ParticipantId;
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
    ParticipantJoined {
        id: String,
        name: String,
        estimating: Option<bool>

    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    receiver: Receiver,
    payload: Payload,
}

pub type MessageStream = Sender<Message>;

#[get("/<session_id>")]
async fn events<'a>(
    session_id: &'a str,
    participant_id: ParticipantId<'a>,
    message_stream: &'a State<MessageStream>,
    mut end: Shutdown,
) -> EventStream<impl Stream<Item = Event> + 'a> {
    let mut rx = message_stream.subscribe();
    EventStream! {
        loop {
            let event = select! {
                event = rx.recv() => match event {
                    Ok(message) => message,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue
                },
                _ = &mut end => break
            };
            match event.receiver {
                Receiver::Participant {session, participant} => {
                    if (session.to_hex() == session_id && participant.to_hex() == participant_id.0) {
                        yield Event::json(&event.payload)
                    }
                }
                Receiver::Session(session) => {
                    if session.to_hex() == session_id {
                        yield Event::json(&event.payload)
                    }
                }
            }
        }
    }
}

pub struct Events {}

impl Events {
    pub fn routes() -> Vec<Route> {
        routes![events]
    }
}

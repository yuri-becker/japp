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

use okapi::openapi3::OpenApi;
use rocket_okapi::openapi_get_routes_spec;

pub mod create_session_request;
pub mod create_session_response;
pub mod log_into_session_response;
pub mod participant_response;
pub mod session_response;
use self::create_session_request::CreateSessionRequest;
use self::create_session_response::CreateSessionResponse;
use self::log_into_session_response::LogIntoSessionResponse;
use self::participant_response::ParticipantResponse;
use crate::application::error::ErrorResponse;
use crate::application::participant_id::ParticipantId;
use crate::domain::session::SessionRepository;
use crate::events::commands::join_participant::JoinParticipant;
use crate::events::message::{Payload, Receiver};
use crate::events::message_stream::MessageStream;
use crate::usecase::command_runner::CommandRunner;
use crate::usecase::generate_session_name::generate_session_name;
use crate::usecase::secrets::{generate_secret, verify_secret};
use rocket::futures::Stream;
use rocket::http::{Cookie, CookieJar};
use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::error::RecvError;
use rocket::{post, Route, Shutdown, State};
use rocket_okapi::openapi;

#[openapi]
#[post("/", data = "<req>")]
async fn create_session(
    req: Json<CreateSessionRequest<'_>>,
    session_repostitory: &State<SessionRepository>,
) -> Result<Json<CreateSessionResponse>, ErrorResponse> {
    let secret = generate_secret();
    session_repostitory
        .create(
            req.name
                .filter(|name| !name.trim().is_empty())
                .map(|name| name.trim().to_string())
                .unwrap_or_else(generate_session_name),
            secret.db_safe_encrypted,
        )
        .await
        .map_err(ErrorResponse::from)
        .map(|session| {
            Json(CreateSessionResponse {
                id: session.id.unwrap().to_hex(),
                secret: secret.clear_text,
            })
        })
}

#[openapi]
#[post("/<id>/participant?<secret>")]
async fn log_into_session(
    id: &str,
    secret: &str,
    cookies: &CookieJar<'_>,
    session_repository: &State<SessionRepository>,
) -> Result<Json<LogIntoSessionResponse>, ErrorResponse> {
    let db_result = session_repository.find_by_id(id).await;
    if db_result.is_err() {
        return Result::Err(ErrorResponse::Mongo(db_result.unwrap_err()));
    }
    let optional_session = db_result.unwrap();
    if optional_session.is_none() {
        return Result::Err(ErrorResponse::NotFound());
    }
    let session = optional_session.unwrap();
    let is_secret_valid = verify_secret(&crate::usecase::secrets::Secret {
        db_safe_encrypted: session.secret,
        clear_text: String::from(secret),
    });
    if !is_secret_valid {
        return Result::Err(ErrorResponse::Unauthorized());
    }
    let cookie_name = format!("participant_id__{}", id);
    if cookies.get_private(&cookie_name).is_none() {
        let participant = session_repository.create_participant(id).await;
        if participant.is_err() {
            return Result::Err(ErrorResponse::Mongo(participant.unwrap_err()));
        }
        cookies.add_private(Cookie::new(
            cookie_name,
            participant.unwrap().id.unwrap().to_hex(),
        ));
    }
    Ok(Json(LogIntoSessionResponse { name: session.name }))
}

#[openapi]
#[put("/<session_id>/participant/<name>")]
async fn join_participant(
    session_id: &str,
    name: &str,
    participant_id: ParticipantId<'_>,
    command_runner: CommandRunner<'_>,
    session_repository: &State<SessionRepository>,
) -> Result<(), ErrorResponse> {
    command_runner
        .run(JoinParticipant {
            participant_id: participant_id.0.to_string(),
            participant_name: name.to_string(),
            session_id: session_id.to_string(),
            session_repository: session_repository,
        })
        .await
}

#[openapi]
#[get("/<session_id>/participant/me")]
async fn get_me(
    session_id: &str,
    participant_id: ParticipantId<'_>,
    session_repostitory: &State<SessionRepository>,
) -> Result<Json<ParticipantResponse>, ErrorResponse> {
    session_repostitory
        .find_participant_by_id(session_id, participant_id.0.as_ref())
        .await
        .map_err(ErrorResponse::Mongo)
        .and_then(|it| it.ok_or(ErrorResponse::NotFound()))
        .map(ParticipantResponse::from)
        .map(Json)
}

#[get("/<session_id>/events")]
async fn events<'a>(
    session_id: &'a str,
    participant_id: ParticipantId<'a>,
    message_stream: &'a State<MessageStream>,
    session_repostitory: &State<SessionRepository>,
    mut end: Shutdown,
) -> EventStream<impl Stream<Item = Event> + 'a> {
    let mut rx = message_stream.subscribe();
    let init = Payload::SessionInit {
        session: session_repostitory
            .find_by_id(session_id)
            .await
            .unwrap_or(Option::None)
            .expect("Request should have already aborted if session does not exist")
            .into(),
    };
    EventStream! {
      yield Event::json(&init);
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
pub struct SessionController {}

impl SessionController {
    pub fn restful_routes() -> (Vec<Route>, OpenApi) {
        openapi_get_routes_spec![create_session, log_into_session, join_participant, get_me]
    }

    pub fn event_stream_routes() -> Vec<Route> {
        routes![events]
    }
}

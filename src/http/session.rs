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

use std::borrow::Borrow;
use crate::application::error::ErrorResponse;
use crate::application::participant_id::ParticipantId;
use crate::domain::session::{session_repository, Participant};
use crate::usecase::generate_session_name::generate_session_name;
use crate::usecase::secrets::{generate_secret, verify_secret};
use mongodb::bson::oid;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{post, Route, State};
use rocket_okapi::{openapi, openapi_get_routes, JsonSchema};
use crate::http::events::{Message, MessageStream, Payload, Receiver};

#[derive(Serialize, JsonSchema)]
struct CreateSessionResponse {
    id: String,
    secret: String,
}

#[derive(Deserialize, JsonSchema)]
struct CreateSessionRequest<'r> {
    name: Option<&'r str>,
}

#[derive(Serialize, JsonSchema)]
struct LogIntoSessionResponse {
    name: String,
}

#[derive(Serialize, JsonSchema)]
struct GetSessionResponse {
    name: String,
}

#[derive(Serialize, JsonSchema)]
struct ParticipantResponse {
    id: String,
    name: Option<String>,
    estimating: Option<bool>,
    away: Option<bool>,
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

#[openapi]
#[post("/", data = "<req>")]
async fn create_session(
    req: Json<CreateSessionRequest<'_>>,
) -> Result<Json<CreateSessionResponse>, ErrorResponse> {
    let secret = generate_secret();
    session_repository::create(
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
) -> Result<Json<LogIntoSessionResponse>, ErrorResponse> {
    let db_result = session_repository::find_by_id(id).await;
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
        let participant = session_repository::create_participant(id).await;
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
async fn set_name(
    session_id: &str,
    name: &str,
    participant_id: ParticipantId<'_>,
    message_stream: &State<MessageStream>
) -> Result<(), ErrorResponse> {
    session_repository::update_participant(
        session_id,
        &Participant {
            id: Option::Some(oid::ObjectId::parse_str(participant_id.0.to_string()).unwrap()),
            name: Option::Some(name.to_string()),
            ..Participant::default()
        },
    )
        .await
        .and_then(|_| Result::Ok(message_stream.send(Message {
            receiver: Receiver::Session(oid::ObjectId::parse_str(session_id).unwrap()),
            payload: Payload::ParticipantJoined {id: participant_id.0.to_string(), name: name.to_string() }
        })))
        .map(|_| ())
        .map_err(ErrorResponse::Mongo)
}

#[openapi]
#[get("/<session_id>")]
async fn get_session(
    session_id: &str,
    _participant: ParticipantId<'_>,
) -> Result<Json<GetSessionResponse>, ErrorResponse> {
    session_repository::find_by_id(session_id)
        .await
        .map_err(ErrorResponse::Mongo)
        .and_then(|it| it.ok_or(ErrorResponse::NotFound()))
        .map(|session| GetSessionResponse { name: session.name })
        .map(Json)
}

#[openapi]
#[get("/<session_id>/participant/me")]
async fn get_me(
    session_id: &str,
    participant_id: ParticipantId<'_>,
) -> Result<Json<ParticipantResponse>, ErrorResponse> {
    session_repository::find_participant_by_id(session_id, participant_id.0.as_ref())
        .await
        .map_err(ErrorResponse::Mongo)
        .and_then(|it| it.ok_or(ErrorResponse::NotFound()))
        .map(ParticipantResponse::from)
        .map(Json)
}

pub struct SessionApi {}

impl SessionApi {
    pub fn routes() -> Vec<Route> {
        openapi_get_routes![
        create_session,
        log_into_session,
        set_name,
        get_session,
        get_me
    ]
    }
}


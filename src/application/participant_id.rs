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

use crate::domain::session::SessionRepository;
use mongodb::bson::oid;
use okapi::openapi3::{Responses, SecurityScheme, SecuritySchemeData};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use schemars::Map;
use std::borrow::Cow;

pub struct ParticipantId<'r>(pub(crate) Cow<'r, str>);

impl<'r> OpenApiFromRequest<'r> for ParticipantId<'r> {
    fn from_request_input(
        gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let scheme = SecurityScheme {
            description: Option::Some(
                "Requires a Cookie. Can be acquired via /session/{id}/participant".to_string(),
            ),
            data: SecuritySchemeData::ApiKey {
                name: "Cookie".to_string(),
                location: "cookie".to_string(),
            },
            extensions: Map::default(),
        };
        gen.add_security_scheme("Cookie".to_string(), scheme.clone());
        Result::Ok(RequestHeaderInput::Security(
            "Cookie".to_string(),
            scheme,
            Map::default(),
        ))
    }
    fn get_responses(_gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        Result::Ok(Responses::default())
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for ParticipantId<'r> {
    type Error = Status;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if request.route().is_none() {
            debug!("{:?} is not handled", request);
            return Outcome::Forward(());
        }
        let segment_index_of_session_id = request
            .route()
            .unwrap()
            .uri
            .as_str()
            .split('/')
            .enumerate()
            .find(|&it| it.1.eq("<session_id>"));
        if segment_index_of_session_id.is_none() {
            panic!("SessionGuard is attached to {} {} but has no path parameters. The first path parameter should be the seesion id", request.method(), request.uri());
        }
        let uri = request.uri().to_string();
        let session_id = uri
            .split('/')
            .nth(segment_index_of_session_id.unwrap().0)
            .unwrap();
        debug!("session_id is {}", session_id);
        let cookie = request
            .cookies()
            .get_private(format!("participant_id__{}", session_id).as_str());
        if cookie.is_none() {
            debug!("No cookie with the session id present. Returnung Unauthorized.");
            return Outcome::Failure((Status::Unauthorized, Status::Unauthorized));
        }
        let session_repository = request
            .rocket()
            .state::<SessionRepository>()
            .expect("SessionRepository is not in state");
        let session = session_repository.find_by_id(session_id).await;
        if session.is_err() {
            return Outcome::Failure((Status::InternalServerError, Status::InternalServerError));
        }
        if session.as_ref().unwrap().as_ref().is_none() {
            debug!("Session with ID {} does not exist", session_id);
            return Outcome::Failure((Status::NotFound, Status::NotFound));
        }
        let cookie = cookie.unwrap();
        let cookie = cookie.value();
        debug!("{}", cookie);
        if cookie.is_empty() {
            debug!("Cookie has no value. Returning Unauthorized.");
            return Outcome::Failure((Status::Unauthorized, Status::Unauthorized));
        }
        let session = session.unwrap().unwrap();
        let participant_id = oid::ObjectId::parse_str(&cookie);
        if participant_id.is_err() {
            debug!("Could not parse cookie as ObjectId. Returning Unauthorized.");
            return Outcome::Failure((Status::Unauthorized, Status::Unauthorized));
        }
        let participant_id = participant_id.unwrap();
        let participant_exists = session
            .participants
            .iter()
            .any(|it| it.id.unwrap().eq(&participant_id));
        if !participant_exists {
            debug!(
                "Participant with id {} not present in session with id {}. Returning Unauthorized.",
                participant_id, session_id
            );
            return Outcome::Failure((Status::Unauthorized, Status::Unauthorized));
        }
        Outcome::Success(ParticipantId(Cow::Owned(cookie.to_string())))
    }
}

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

use crate::application::error::ErrorResponse;
use crate::events::command::Command;
use crate::events::message_stream::MessageStream;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket_okapi::request::OpenApiFromRequest;

pub struct CommandRunner<'a> {
    pub message_stream: &'a MessageStream,
}

impl<'r> CommandRunner<'r> {
    pub async fn run(&self, command: impl Command) -> Result<(), ErrorResponse> {
        command
            .domain_update()
            .await
            .and_then(|_| Result::Ok(self.message_stream.send(command.event())))
            .map(|_| ())
            .map_err(ErrorResponse::Mongo)
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for CommandRunner<'r> {
    type Error = Status;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        request
            .rocket()
            .state::<MessageStream>()
            .map(|it| Outcome::Success(CommandRunner { message_stream: it }))
            .unwrap_or(Outcome::Failure((
                Status::InternalServerError,
                Status::InternalServerError,
            )))
    }
}

impl<'r> OpenApiFromRequest<'r> for CommandRunner<'r> {
    fn get_responses(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
    ) -> rocket_okapi::Result<okapi::openapi3::Responses> {
        Ok(okapi::openapi3::Responses::default())
    }

    fn from_request_input(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<rocket_okapi::request::RequestHeaderInput> {
        Ok(rocket_okapi::request::RequestHeaderInput::None)
    }
}

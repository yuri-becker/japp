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

use okapi::openapi3::{RefOr, Response, Responses};
use rocket::http::Status;
use rocket::response::Responder;
use rocket::{response, warn, Request};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::response::OpenApiResponderInner;

#[derive(Debug)]
pub enum ErrorResponse {
    Mongo(mongodb::error::Error),
    NotFound(),
    Unauthorized(),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ErrorResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        // log `self` to your favored error tracker, e.g.
        // sentry::capture_error(&self);
        match self {
            ErrorResponse::Mongo(it) => {
                warn!("MongoDB Operation failed: {:?}", it);
                Status::InternalServerError.respond_to(req)
            }
            ErrorResponse::NotFound() => Status::NotFound.respond_to(req),
            ErrorResponse::Unauthorized() => Status::Unauthorized.respond_to(req),
        }
    }
}

impl From<mongodb::error::Error> for ErrorResponse {
    fn from(it: mongodb::error::Error) -> Self {
        ErrorResponse::Mongo(it)
    }
}

impl OpenApiResponderInner for ErrorResponse {
    fn responses(_: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        let mut responses = Responses::default();
        responses
            .responses
            .entry("404".to_string())
            .or_insert(RefOr::Object(Response {
                description: "The requested document was not found".to_string(),
                ..Response::default()
            }));
        responses
            .responses
            .entry("401".to_string())
            .or_insert(RefOr::Object(Response {
                description: "The endpoint needs authorization and the callee has not authorized"
                    .to_string(),
                ..Response::default()
            }));
        responses
            .responses
            .entry("500".to_string())
            .or_insert(RefOr::Object(Response {
                description: "Something went wrong".to_string(),
                ..Response::default()
            }));
        Result::Ok(responses)
    }
}

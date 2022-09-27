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

use crate::domain::domain::DomainObject;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, to_bson, Bson};
use mongodb::options::CreateIndexOptions;
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Participant {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimating: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub away: Option<bool>,
    pub estimates: Vec<Option<usize>>,
}

impl Default for Participant {
    fn default() -> Self {
        Participant {
            id: Some(ObjectId::default()),
            name: None,
            estimating: None,
            away: None,
            estimates: vec![],
        }
    }
}

impl From<Participant> for Bson {
    fn from(it: Participant) -> Self {
        to_bson(&it).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub secret: String,
    pub participants: Vec<Participant>,
    pub issues: Vec<String>,
    pub scale: Vec<String>,
}

pub struct Scale;

impl Scale {
    pub fn fibonacci() -> Vec<&'static str> {
        vec!["1", "2", "3", "5", "8", "13", "21", "34", "55", "89"]
    }
}

impl DomainObject for Session {
    fn collection_name() -> &'static str {
        "sessions"
    }
    fn indexes() -> Vec<(IndexModel, Option<CreateIndexOptions>)> {
        vec![]
    }
}

pub mod session_repository {
    use crate::application::database::database;
    use crate::domain::domain::to_oid;
    use crate::domain::session::{Participant, Scale, Session};
    use mongodb::bson::doc;
    use mongodb::bson::oid;
    use mongodb::bson::to_document;
    use mongodb::error::Error;
    use mongodb::Collection;

    fn collection() -> Collection<Session> {
        database().collection::<Session>("sessions")
    }

    pub async fn create(name: String, secret: String) -> Result<Session, Error> {
        let doc = Session {
            id: None,
            name: name.to_string(),
            participants: vec![],
            secret: secret.to_string(),
            issues: vec![],
            scale: Scale::fibonacci()
                .iter()
                .map(|&it| it.to_string())
                .collect(),
        };
        collection()
            .insert_one(&doc, None)
            .await
            .map(|result| Session {
                id: Some(to_oid(result.inserted_id)),
                ..doc
            })
    }

    pub async fn find_by_id(id: &str) -> Result<Option<Session>, Error> {
        collection()
            .find_one(doc! {"_id": oid::ObjectId::parse_str(id).unwrap()}, None)
            .await
    }

    pub async fn create_participant(session_id: &str) -> Result<Participant, Error> {
        let participant = Participant::default();
        collection()
            .update_one(
                doc! {"_id": oid::ObjectId::parse_str(session_id).unwrap()},
                doc! {"$push": {"participants": &participant}},
                None,
            )
            .await
            .map(|_| participant)
    }

    pub async fn find_participant_by_id(
        session_id: &str,
        participant: &str,
    ) -> Result<Option<Participant>, Error> {
        let participant = oid::ObjectId::parse_str(participant);
        if participant.is_err() {
            return Result::Ok(Option::None);
        }
        let participant = participant.unwrap();
        find_by_id(session_id)
            .await
            .map(|it| {
                it.map(|session| {
                    session
                        .participants
                        .iter()
                        .find(|part| part.id.unwrap().eq(&participant))
                        .cloned()
                })
            })
            .map(|it| it.flatten())
    }

    pub async fn update_participant(
        session_id: &str,
        participant: &Participant,
    ) -> Result<(), Error> {
        debug!(
            "Updating Participant in Session {} with object {:?}",
            session_id,
            to_document(participant).unwrap()
        );
        let mut update = doc! {};
        let doc = to_document(participant).unwrap();
        doc.keys().for_each(|key| {
            update.insert(format!("participants.$.{}", key), doc.get(key).unwrap());
        });
        collection()
        .update_one(
          doc! {"$and": [{"_id": oid::ObjectId::parse_str(session_id).unwrap()}, {"participants._id": participant.id.unwrap()}]},
          doc! {"$set": update},
          Option::None,
        )
        .await
        .map(|_| ())
    }
}

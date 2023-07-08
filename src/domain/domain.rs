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

use mongodb::bson::oid::ObjectId;
use mongodb::bson::Bson;
use mongodb::options::CreateIndexOptions;
use mongodb::IndexModel;

pub trait DomainObject {
    fn collection_name() -> &'static str;
    fn indexes() -> Vec<(IndexModel, Option<CreateIndexOptions>)>;
}

pub fn to_oid(bson: Bson) -> ObjectId {
    match bson {
        Bson::ObjectId(oid) => oid,
        _ => panic!("Not an objectId"),
    }
}

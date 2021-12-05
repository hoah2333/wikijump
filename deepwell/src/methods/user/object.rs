/*
 * methods/user/object.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2021 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use super::prelude::*;
use std::convert::TryFrom;

#[derive(Deserialize, Debug)]
struct UserGetQuery {
    #[serde(default)]
    detail: UserDetails,

    #[serde(default = "bool_true")]
    avatars: bool,
}

pub async fn user_get(req: ApiRequest) -> ApiResponse {
    let _reference = Reference::try_from(&req)?;
    let _query: UserGetQuery = req.query()?;
    // returns UserResponse
    todo!()
}

pub async fn user_reset(req: ApiRequest) -> ApiResponse {
    let _reference = Reference::try_from(&req)?;
    let _query: UserGetQuery = req.query()?;
    // returns ()
    todo!()
}

pub async fn user_avatar_get(req: ApiRequest) -> ApiResponse {
    let _reference = Reference::try_from(&req)?;
    let _query: UserGetQuery = req.query()?;
    // returns Vec<u8>
    todo!()
}

pub async fn user_avatar_delete(req: ApiRequest) -> ApiResponse {
    let _reference = Reference::try_from(&req)?;
    let _query: UserGetQuery = req.query()?;
    // returns ()
    todo!()
}
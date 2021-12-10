/*
 * methods/locales.rs
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MessageRequest {
    locale: String,
    message_key: String,
}

pub async fn message_get(mut req: ApiRequest) -> ApiResponse {
    let MessageRequest {
        locale,
        message_key,
    } = req.body_json().await?;

    let message = req
        .state()
        .localizations
        .translate(&locale, &message_key)
        .into();

    Ok(message)
}
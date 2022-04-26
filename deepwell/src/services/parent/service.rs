/*
 * services/parent/service.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2022 Wikijump Team
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
use crate::models::page_parent::{self, Entity as PageParent};
use crate::services::PageService;

#[derive(Debug)]
pub struct ParentService;

impl ParentService {
    /// Adds a parental relationship with the two given pages.
    ///
    /// Both pages must be extant and on the same site.
    ///
    /// # Returns
    /// Returns `true` if the relationship was created, and
    /// `false` if it already existed.
    pub async fn add_child(
        ctx: &ServiceContext<'_>,
        site_id: i64,
        parent_page_ref: Reference<'_>,
        child_page_ref: Reference<'_>,
    ) -> Result<bool> {
        let txn = ctx.transaction();

        let (parent_page, child_page) = try_join!(
            PageService::get(ctx, site_id, parent_page_ref),
            PageService::get(ctx, site_id, child_page_ref),
        )?;

        let relationship =
            PageParent::find_by_id((parent_page.page_id, child_page.page_id))
                .one(txn)
                .await?;

        match relationship {
            // Create new parent relationship
            None => {
                let relationship = page_parent::ActiveModel {
                    parent_page_id: Set(parent_page.page_id),
                    child_page_id: Set(child_page.page_id),
                    ..Default::default()
                };

                relationship.insert(txn).await?;
                Ok(true)
            }

            // Parent relationship already exists
            Some(_) => Ok(false),
        }
    }

    pub async fn delete_children() -> Result<()> {
        // TODO
        Ok(())
    }
}

// TODO

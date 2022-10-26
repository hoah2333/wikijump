//! SeaORM Entity. Generated by sea-orm-codegen 0.10.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "page")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub page_id: i64,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub site_id: i64,
    pub page_category_id: i64,
    #[sea_orm(column_type = "Text")]
    pub slug: String,
    pub discussion_thread_id: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::page_category::Entity",
        from = "Column::PageCategoryId",
        to = "super::page_category::Column::CategoryId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    PageCategory,
    #[sea_orm(
        belongs_to = "super::site::Entity",
        from = "Column::SiteId",
        to = "super::site::Column::SiteId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Site,
    #[sea_orm(has_many = "super::page_revision::Entity")]
    PageRevision,
    #[sea_orm(has_many = "super::page_attribution::Entity")]
    PageAttribution,
    #[sea_orm(has_many = "super::page_lock::Entity")]
    PageLock,
    #[sea_orm(has_many = "super::page_link::Entity")]
    PageLink,
    #[sea_orm(has_many = "super::page_vote::Entity")]
    PageVote,
    #[sea_orm(has_many = "super::file::Entity")]
    File,
    #[sea_orm(has_many = "super::file_revision::Entity")]
    FileRevision,
}

impl Related<super::page_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageCategory.def()
    }
}

impl Related<super::site::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Site.def()
    }
}

impl Related<super::page_revision::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageRevision.def()
    }
}

impl Related<super::page_attribution::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageAttribution.def()
    }
}

impl Related<super::page_lock::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageLock.def()
    }
}

impl Related<super::page_link::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageLink.def()
    }
}

impl Related<super::page_vote::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageVote.def()
    }
}

impl Related<super::file::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::File.def()
    }
}

impl Related<super::file_revision::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FileRevision.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

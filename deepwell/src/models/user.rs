//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i64,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "Text", unique)]
    pub name: String,
    #[sea_orm(column_type = "Text", unique)]
    pub slug: String,
    pub name_changes_left: i16,
    pub last_renamed_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "Text", unique)]
    pub email: String,
    pub email_verified_at: Option<DateTimeWithTimeZone>,
    pub is_system: bool,
    pub is_bot: bool,
    #[sea_orm(column_type = "Text")]
    pub password: String,
    #[sea_orm(column_type = "Text")]
    pub locale: String,
    pub avatar_s3_hash: Option<Vec<u8>>,
    #[sea_orm(column_type = "Text")]
    pub real_name: String,
    #[sea_orm(column_type = "Text")]
    pub gender: String,
    pub birthday: Option<Date>,
    #[sea_orm(column_type = "Text")]
    pub biography: String,
    #[sea_orm(column_type = "Text")]
    pub user_page: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::page_revision::Entity")]
    PageRevision,
    #[sea_orm(has_many = "super::page_attribution::Entity")]
    PageAttribution,
    #[sea_orm(has_many = "super::page_lock::Entity")]
    PageLock,
    #[sea_orm(has_many = "super::file_revision::Entity")]
    FileRevision,
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

impl Related<super::file_revision::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FileRevision.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

//! SeaORM Entity. Generated by sea-orm-codegen 0.10.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "site")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub site_id: i64,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "Text")]
    pub slug: String,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub tagline: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    #[sea_orm(column_type = "Text")]
    pub locale: String,
    #[sea_orm(column_type = "Text")]
    pub default_page: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::page_category::Entity")]
    PageCategory,
    #[sea_orm(has_many = "super::page::Entity")]
    Page,
    #[sea_orm(has_many = "super::page_revision::Entity")]
    PageRevision,
}

impl Related<super::page_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageCategory.def()
    }
}

impl Related<super::page::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Page.def()
    }
}

impl Related<super::page_revision::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageRevision.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

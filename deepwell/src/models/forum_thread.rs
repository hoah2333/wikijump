//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "forum_thread")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub thread_id: i64,
    pub user_id: Option<i32>,
    pub user_string: Option<String>,
    pub category_id: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub number_posts: i32,
    pub date_started: Option<DateTimeUtc>,
    pub site_id: Option<i32>,
    pub last_post_id: Option<i32>,
    pub page_id: Option<i32>,
    pub sticky: bool,
    pub blocked: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::forum_category::Entity",
        from = "Column::CategoryId",
        to = "super::forum_category::Column::CategoryId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    ForumCategory,
    #[sea_orm(
        belongs_to = "super::forum_post::Entity",
        from = "Column::LastPostId",
        to = "super::forum_post::Column::PostId",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    ForumPost,
    #[sea_orm(
        belongs_to = "super::page::Entity",
        from = "Column::PageId",
        to = "super::page::Column::PageId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Page,
    #[sea_orm(
        belongs_to = "super::site::Entity",
        from = "Column::SiteId",
        to = "super::site::Column::SiteId",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Site,
}

impl Related<super::forum_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ForumCategory.def()
    }
}

impl Related<super::forum_post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ForumPost.def()
    }
}

impl Related<super::page::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Page.def()
    }
}

impl Related<super::site::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Site.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

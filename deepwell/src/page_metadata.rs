//! SeaORM Entity. Generated by sea-orm-codegen 0.4.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "page_metadata")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub metadata_id: i64,
    pub parent_page_id: Option<i32>,
    pub title: Option<String>,
    pub unix_name: Option<String>,
    pub owner_user_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
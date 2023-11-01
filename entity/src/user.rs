use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(DeriveEntityModel, Debug, Clone, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(unique)]
    pub email: String,

    pub password: String,
}

#[derive(Debug, Copy, Clone, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

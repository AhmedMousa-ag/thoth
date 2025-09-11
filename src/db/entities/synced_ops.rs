use crate::err;
use chrono::{DateTime, Utc};
use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "synced_ops")]
pub struct Model {
    #[sea_orm(index, unique, auto_increment = false)]
    pub synced_id: String,
    #[sea_orm(default = false)]
    pub is_finished: bool,
    #[sea_orm(primary_key)]
    pub from_date: DateTime<Utc>,
    #[sea_orm(primary_key)]
    pub to_date: DateTime<Utc>,
    #[sea_orm(nullable, default = None)]
    pub ops_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        err!("No RelationDef"; panic = true);
        unreachable!()
    }
}

impl ActiveModelBehavior for ActiveModel {}

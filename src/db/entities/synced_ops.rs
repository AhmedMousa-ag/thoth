use chrono::{DateTime, Utc};
use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "synced_ops")]
pub struct Model {
    #[sea_orm(index, unique, auto_increment = true)]
    pub synced_id: String,
    #[sea_orm(default = false)]
    pub is_finished: bool,
    #[sea_orm(primary_key)]
    pub from_date: DateTime<Utc>,
    #[sea_orm(primary_key)]
    pub to_date: DateTime<Utc>,
    pub ops_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Operation,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Operation => Entity::belongs_to(super::operations::Entity)
                .from(Column::OpsId)
                .to(super::operations::Column::OpId)
                .into(),
        }
    }
}

impl Related<super::operations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Operation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

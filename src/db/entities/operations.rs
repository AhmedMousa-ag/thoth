use chrono::{DateTime, Utc};
use sea_orm::prelude::*;
use tonic::async_trait;

use crate::info;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "operations")]
pub struct Model {
    #[sea_orm(primary_key, unique, auto_increment = false)]
    op_id: String, // Instead of using UUID, we are already converting everything into string and UUID several times.
    exec_date: DateTime<Utc>,
    #[sea_orm(default_value = false)]
    is_finished: bool,
    //TODO you might put the result here.
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::steps::Entity")]
    Step,
}

impl Related<super::steps::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Step.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn after_save<C>(model: Model, _db: &C, _insert: bool) -> Result<Model, DbErr>
    where
        C: ConnectionTrait,
    {
        info!("Successfull insertion of an operation model into the database");
        Ok(model)
    }
}

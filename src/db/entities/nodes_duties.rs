use chrono::{DateTime, Utc};
use sea_orm::prelude::*;

use crate::{err, operations::planner::charts::structs::OperationInfo};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "nodes_duties")]
pub struct Model {
    #[sea_orm(indexed)]
    pub node_id: String, // Instead of using UUID, we are already converting everything into string and UUID several times.
    pub op_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub step_id: String,
    #[sea_orm(default_value = "false")]
    pub is_finished: bool,
    pub created_at: DateTime<Utc>,
    //TODO you might put the result here.
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        err!("No RelationDef"; panic=true);
        unreachable!()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for OperationInfo {
    fn from(model: Model) -> Self {
        OperationInfo {
            operation_id: model.op_id,
            step_id: model.step_id,
        }
    }
}

impl From<&Model> for OperationInfo {
    fn from(model: &Model) -> Self {
        OperationInfo {
            operation_id: model.op_id.clone(),
            step_id: model.step_id.clone(),
        }
    }
}

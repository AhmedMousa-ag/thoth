use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::{self, Set};
use tokio::spawn;

use crate::{
    db::{
        controller::traits::{
            SQLiteDBTraits, SqlNodesDuties, SqlOperations, SqlSteps, SqlSyncedOps,
        },
        entities::nodes_duties::ActiveModel as NodesDutiesActiveModel,
    },
    err,
    errors::thot_errors::ThothErrors,
    operations::planner::charts::structs::NodesOpsMsg,
};

pub struct DbOpsRegisterer {}
impl DbOpsRegisterer {
    pub fn new_syncer(date_from: DateTime<Utc>, date_to: DateTime<Utc>) {
        spawn(async move {
            if let Err(e) = SqlSyncedOps::insert_row(SqlSyncedOps::new(date_from, date_to)) {
                err!("new synced operation {}", ThothErrors::from(e))
            };
        });
    }
    pub fn new_operation(operation_id: String) {
        spawn(async move {
            if let Err(e) = SqlOperations::insert_row(SqlOperations::new(operation_id)) {
                err!("new operations {}", ThothErrors::from(e))
            };
        });
    }
    pub fn new_step(operation_id: String, step_id: String, use_prev_res: bool) {
        spawn(async move {
            if let Err(e) = {
                let mut model = SqlSteps::new(step_id.clone(), operation_id.clone());
                model.use_prev_res = Set(use_prev_res);
                SqlSteps::insert_row(model)
            } {
                err!("new step {}", ThothErrors::from(e))
            };
        });
    }
    pub fn finished_step() {}
    pub fn new_duty(node_id: String, operation_id: String, step_id: String) {
        spawn(async move {
            if let Err(e) = SqlNodesDuties::insert_row(SqlNodesDuties::new(
                operation_id.clone(),
                node_id.clone(),
                step_id,
            )) {
                err!("New node duty {}", ThothErrors::from(e))
            };
        });
    }
    pub fn finished_duty(step_id: String) {
        spawn(async move {
            let mut sql_duty = NodesDutiesActiveModel {
                step_id: ActiveValue::Unchanged(step_id),
                ..Default::default()
            };
            sql_duty.is_finished = Set(true);
            if let Err(e) = SqlNodesDuties::update_row(sql_duty) {
                err!("Marking duty as finished: {}", ThothErrors::from(e));
            };
        });
    }
    /// Register both a step and a duty in one funciton
    pub fn new_step_duty(
        node_id: String,
        operation_id: String,
        step_id: String,
        use_prev_res: bool,
    ) {
        DbOpsRegisterer::new_step(operation_id.clone(), step_id.clone(), use_prev_res);
        DbOpsRegisterer::new_duty(node_id, operation_id, step_id);
    }
    pub fn new_duties(duties: NodesOpsMsg) {
        for (node_id, ops_info) in duties.nodes_duties {
            for ops_infos in ops_info.try_read().unwrap().clone() {
                DbOpsRegisterer::new_duty(
                    node_id.clone(),
                    ops_infos.operation_id,
                    ops_infos.step_id,
                );
            }
        }
    }
}

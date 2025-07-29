use sea_orm::ActiveValue::Set;
use tokio::spawn;

use crate::{
    db::controller::traits::{SQLiteDBTraits, SqlNodesDuties, SqlSteps},
    err,
    errors::thot_errors::ThothErrors,
    operations::planner::charts::structs::NodesOpsMsg,
};

pub struct DbOpsRegisterer {}
impl DbOpsRegisterer {
    pub fn new_step(operation_id: String, step_id: String) {
        spawn(async move {
            if let Err(e) =
                SqlSteps::insert_row(SqlSteps::new(step_id.clone(), operation_id.clone()))
            {
                err!("Error new step: {}", ThothErrors::from(e))
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
                err!("Error new node duty: {}", ThothErrors::from(e))
            };
        });
    }
    pub fn finished_duty(node_id: String, operation_id: String, step_id: String) {
        spawn(async move {
            let mut sql_duty = SqlNodesDuties::new(operation_id, node_id, step_id);
            sql_duty.is_finished = Set(true);
            if let Err(e) = SqlNodesDuties::update_row(sql_duty) {
                err!("Error marking duty as finished: {}", e);
            };
        });
    }
    /// Register both a step and a duty in one funciton
    pub fn new_step_duty(node_id: String, operation_id: String, step_id: String) {
        DbOpsRegisterer::new_step(operation_id.clone(), step_id.clone());
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

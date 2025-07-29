use tokio::spawn;

use crate::{
    db::controller::traits::{SQLiteDBTraits, SqlNodesDuties, SqlSteps},
    err,
    errors::thot_errors::ThothErrors,
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
    pub fn finished_duties() {}
    /// Register both a step and a duty in one funciton
    pub fn new_step_duty(node_id: String, operation_id: String, step_id: String) {
        DbOpsRegisterer::new_step(operation_id.clone(), step_id.clone());
        DbOpsRegisterer::new_duty(node_id, operation_id, step_id);
    }
}

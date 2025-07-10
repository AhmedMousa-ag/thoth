use crate::{
    db::controller::traits::{SQLiteDBTraits, SqlSteps},
    operations::{
        executer::types::Executer,
        planner::charts::structs::{NodesOpsMsg, Steps},
    },
};
use std::{cell::RefCell, rc::Rc};

impl Executer {
    pub fn execute_step(step: Rc<RefCell<Steps>>) -> Rc<RefCell<Steps>> {
        // Register into Sqlite the operation.
        let sql_step = SqlSteps::new(
            step.borrow().step_id.to_owned(),
            step.borrow().operation_id.to_owned(),
        );
        SqlSteps::insert_row(sql_step).unwrap();

        // Async Write it into operations file with serialization.

        // Translate the steps into a result.
        // Update steps files.
        step
    }
    pub fn execute_duties(duties: NodesOpsMsg) {
        // Check for every step result, do calculate what is pending.
        // Return the final result to all nodes.
        // Invoke Returning final result to the user. Maybe in another function or something.
    }
}

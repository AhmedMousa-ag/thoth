use crate::{
    db::controller::traits::{SQLiteDBTraits, SqlSteps},
    err,
    operations::{
        executer::types::Executer,
        planner::charts::structs::{NodesOpsMsg, Steps},
    },
};
use std::{cell::RefCell, rc::Rc};

impl Executer {
    pub fn execute_step(&mut self, step: Rc<RefCell<Steps>>) -> Rc<RefCell<Steps>> {
        // Register into Sqlite the operation.
        let sql_step = SqlSteps::new(
            step.borrow().step_id.to_owned(),
            step.borrow().operation_id.to_owned(),
        );
        SqlSteps::insert_row(sql_step).unwrap();
        err!("{}", self.op_file_manager.write(step.clone()).unwrap_err());
        // Translate the steps into a result.

        // Update steps files.
        err!("{}", self.op_file_manager.write(step.clone()).unwrap_err());
        step
    }
    pub fn execute_duties(&mut self, duties: Box<NodesOpsMsg>) {
        // Check for every step result, do calculate what is pending.
        // Return the final result to all nodes.
        // Invoke Returning final result to the user. Maybe in another function or something.
    }
}

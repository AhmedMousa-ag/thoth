use std::{cell::RefCell, rc::Rc};

use crate::operations::{
    executer::types::Executer,
    planner::charts::structs::{NodesOpsMsg, Steps},
};

impl Executer {
    pub fn execute_step(step: Rc<RefCell<Steps>>) -> Rc<RefCell<Steps>> {
        // Register into Sqlite the operation.
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

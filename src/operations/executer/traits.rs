use crate::{
    connections::channels_node_info::get_current_node_cloned,
    db::controller::traits::{SQLiteDBTraits, SqlOperations, SqlSteps},
    err,
    operations::{
        executer::types::Executer,
        planner::charts::structs::{NodesOpsMsg, Steps},
        translator::translate::DutiesTranslator,
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
        let step = DutiesTranslator::translate_step(step); //I think we don't need to return it as it's mutable by reference.
        // Update steps files.
        err!("{}", self.op_file_manager.write(step.clone()).unwrap_err());
        step
    }
    pub fn execute_duties(&mut self, duties: Box<NodesOpsMsg>) {
        // Check for every step result, do calculate what is pending.
        if let Some(node_duties) = duties.nodes_duties.get(&get_current_node_cloned().id) {
            let _sql_ops = SqlOperations::insert_row(SqlOperations::new(
                node_duties.borrow()[0].operation_id.clone(),
            ))
            .unwrap();
            for duty in node_duties.borrow().iter() {
                // DutiesTranslator::new(node_duty)
            }

            // SqlOperations::update_row(sql_ops);
        }
        // Return the final result to all nodes.
        // Invoke Returning final result to the user. Maybe in another function or something.
    }
}

use crate::{
    connections::channels_node_info::get_current_node_cloned,
    db::controller::traits::{SQLiteDBTraits, SqlOperations, SqlSteps},
    debug, err,
    operations::{
        executer::types::Executer,
        planner::charts::structs::{NodesOpsMsg, Steps},
        translator::translate::DutiesTranslator,
    },
};
use std::sync::{Arc, RwLock};

impl Executer {
    pub fn execute_step(&mut self, step: Arc<RwLock<Steps>>) -> Arc<RwLock<Steps>> {
        // Register into Sqlite the operation.

        let step_id = step.try_read().unwrap().step_id.clone();
        let op_id = step.try_read().unwrap().operation_id.clone();

        let is_op_exists = SqlOperations::find_by_id(op_id.clone()).is_some();
        debug!(" Is Ops exists: {:?}", is_op_exists);
        if !is_op_exists {
            let _ = SqlOperations::insert_row(SqlOperations::new(op_id.clone()));
        }
        SqlSteps::insert_row(SqlSteps::new(step_id.clone(), op_id.clone())).unwrap();
        self.op_file_manager.write(Arc::clone(&step)).unwrap();
        // err!("{}", self.op_file_manager.write(step.clone()).unwrap_err());
        // Translate the steps into a result.
        let step = DutiesTranslator::translate_step(Arc::clone(&step)); //I think we don't need to return it as it's mutable by reference.
        // Update steps files.
        self.op_file_manager.write(Arc::clone(&step)).unwrap();
        // err!("{}", self.op_file_manager.write(step.clone()).unwrap_err());
        step
    }
    pub fn execute_duties(&mut self, duties: Box<NodesOpsMsg>) {
        // Check for every step result, do calculate what is pending.
        if let Some(node_duties) = duties.nodes_duties.get(&get_current_node_cloned().id) {
            let _sql_ops = SqlOperations::insert_row(SqlOperations::new(
                node_duties.try_read().unwrap()[0].operation_id.clone(),
            ))
            .unwrap();
            for duty in node_duties.try_read().unwrap().iter() {
                // DutiesTranslator::new(node_duty)
            }

            // SqlOperations::update_row(sql_ops);
        }
        // Return the final result to all nodes.
        // Invoke Returning final result to the user. Maybe in another function or something.
    }
}

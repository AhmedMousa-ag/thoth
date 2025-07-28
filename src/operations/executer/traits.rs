use crate::{
    connections::channels_node_info::get_current_node_cloned,
    db::controller::traits::{SQLiteDBTraits, SqlOperations, SqlSteps},
    err,
    logger::writters::writter::OperationsFileManager,
    operations::{
        executer::types::Executer,
        planner::charts::structs::{NodesOpsMsg, Steps},
        translator::translate::DutiesTranslator,
    },
    warn,
};
use sea_orm::ActiveValue;
use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};
impl Executer {
    pub fn execute_step(&mut self, step: Arc<RwLock<Steps>>) -> Arc<RwLock<Steps>> {
        let step_id = step
            .try_read()
            .map_err(|e| err!("Faild to read lock step due to: {}.", e))
            .unwrap()
            .step_id
            .clone();
        let op_id = step.try_read().unwrap().operation_id.clone();

        let is_op_exists = SqlOperations::find_by_id(op_id.clone()).is_some();
        if !is_op_exists {
            let _ = SqlOperations::insert_row(SqlOperations::new(op_id.clone()));
        }
        SqlSteps::insert_row(SqlSteps::new(step_id.clone(), op_id.clone())).unwrap();
        let _ = self.op_file_manager.write(Arc::clone(&step), false); // Ignoring this error as it's not critical.
        // Translate the steps into a result.
        let step = DutiesTranslator::translate_step(Arc::clone(&step)); //I think we don't need to return it as it's mutable by reference.
        // Update steps files.
        self.op_file_manager
            .write(Arc::clone(&step), false)
            .unwrap();
        // err!("{}", self.op_file_manager.write(step.clone()).unwrap_err());
        step
    }
    pub fn execute_duties(&mut self, duties: Box<NodesOpsMsg>) {
        // Check for every step result.
        if let Some(node_duties) = duties.nodes_duties.get(&get_current_node_cloned().id) {
            let op_id = node_duties.try_read().unwrap()[0].operation_id.clone();
            let mut sql_ops_model = SqlOperations::new(op_id.clone());
            if SqlOperations::find_by_id(op_id).is_none() {
                SqlOperations::insert_row(sql_ops_model.clone()).unwrap();
            }
            for duty in node_duties.try_read().unwrap().iter() {
                // DutiesTranslator::new(node_duty)
                while SqlSteps::find_by_id(duty.step_id.clone()).is_none() {
                    //Will wait for one second, maybe not all messages weren't processed. //TODO There's a potential an error happened and might cause the process to keep waiting.
                    thread::sleep(Duration::from_secs(1));
                }
                let step = OperationsFileManager::load_step_file(&duty.operation_id, &duty.step_id)
                    .unwrap(); //You might get it from the sqlite, possible you should not use the sqlite, it feels limited to it's one thread access in it's nature.
                if step.result.is_none() {
                    self.execute_step(Arc::new(RwLock::new(step)));
                }
            }
            sql_ops_model.is_finished = ActiveValue::Set(true);
            let _ = SqlOperations::update_row(sql_ops_model);
        }
    }
}

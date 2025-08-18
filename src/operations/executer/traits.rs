use crate::{
    connections::channels_node_info::get_current_node_cloned, db::controller::registerer::DbOpsRegisterer, debug, err, errors::thot_errors::ThothErrors, info, logger::writters::writter::OperationsFileManager, operations::{
        checker::decrease_running_operation,
        executer::types::Executer,
        planner::charts::structs::{NodesOpsMsg, Steps},
        translator::translate::DutiesTranslator,
    }, warn
};

use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

impl Executer {
    pub fn execute_step(&mut self, step: Arc<RwLock<Steps>>) {
        let step_id = step
            .try_read()
            .map_err(|e| err!("Faild to read lock step due to: {}.", e))
            .unwrap()
            .step_id
            .clone();
        let op_id = step.try_read().unwrap().operation_id.clone();

        let is_op_exists = DbOpsRegisterer::get_operation_file(&op_id).is_some();
        if !is_op_exists {
            debug!("Operation id doesn't exists in SQL, will insert new one");
            DbOpsRegisterer::new_operation(op_id.clone(), true);
        }

        let file_step = DbOpsRegisterer::get_step_file(&op_id, &step_id);
        if file_step.is_some_and(|stp| !stp.result.is_none()) {
            info!("Step id {} already has a result, skipping execution.", step_id);
            return;
        }

        DbOpsRegisterer::new_step(Arc::clone(&step), true); // Ignoring this error as it's not critical.
        let step = DutiesTranslator::translate_step(Arc::clone(&step)); //I think we don't need to return it as it's mutable by reference.
        DbOpsRegisterer::new_step(Arc::clone(&step), true);

        decrease_running_operation(op_id);
    }

    fn get_result_string(&self, step: Arc<RwLock<Steps>>) -> Option<String> {
        let step = step.try_read().unwrap();
        if let Some(result) = &step.result {
            let res = match serde_json::to_string(result) {
                Ok(res) => Some(res),
                Err(e) => {
                    err!(
                        "Faild to encode step result into string: {}",
                        ThothErrors::from(e)
                    );
                    None
                }
            };
            res
        } else {
            None
        }
    }

    pub fn execute_duties(&mut self, duties: Box<NodesOpsMsg>) {
        // Check for every step result.
        if let Some(node_duties) = duties.nodes_duties.get(&get_current_node_cloned().id) {
            let op_id = node_duties.try_read().unwrap()[0].operation_id.clone();
            // let mut sql_ops_model = SqlOperations::new(op_id.clone());
            if DbOpsRegisterer::get_operation_file(&op_id).is_none() {
                DbOpsRegisterer::new_operation(op_id.clone(),true);
            }
            for duty in node_duties.try_read().unwrap().iter() {
                // DutiesTranslator::new(node_duty)
                while DbOpsRegisterer::get_step_file(&duty.operation_id, &duty.step_id).is_none() {
                    //Will wait for one second, maybe not all messages weren't processed. //TODO There's a potential an error happened and might cause the process to keep waiting.
                    thread::sleep(Duration::from_secs(1));
                }
                match OperationsFileManager::load_step_file(
                    &duty.operation_id,
                    &duty.step_id.clone(),
                ) {
                    //You might get it from the sqlite, possible you should not use the sqlite, it feels limited to it's one thread access in it's nature.
                    Ok(step) => {
                        if step.result.is_none() {
                            self.execute_step(Arc::new(RwLock::new(step)));
                        }
                    }
                    Err(e) => {
                        warn!(
                            "Faild to load step file for step id {}: {}",
                            duty.step_id,
                            ThothErrors::from(e)
                        );
                        continue;
                    }
                }
                DbOpsRegisterer::finished_duty(duty.step_id.clone(), true);
            }
            // DbOpsRegisterer::new_operation(operation_id, thread);
            // sql_ops_model.is_finished = ActiveValue::Set(true);
            // let _ = SqlOperations::update_row(sql_ops_model);
        }
    }
}

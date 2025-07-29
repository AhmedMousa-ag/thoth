use crate::{
    connections::channels_node_info::get_current_node_cloned,
    db::controller::{
        registerer::DbOpsRegisterer,
        traits::{SQLiteDBTraits, SqlOperations, SqlSteps},
    },
    debug, err,
    errors::thot_errors::ThothErrors,
    logger::writters::writter::OperationsFileManager,
    operations::{
        executer::types::Executer,
        planner::charts::structs::{NodesOpsMsg, Steps},
        translator::translate::DutiesTranslator,
    },
    warn,
};
use sea_orm::{
    ActiveValue::{self, Set},
    IntoActiveModel,
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

        let is_op_exists = SqlOperations::find_by_id(op_id.clone()).is_some();
        if !is_op_exists {
            debug!("Operation doesn't exists will create a new one");
            if let Err(e) = SqlOperations::insert_row(SqlOperations::new(op_id.clone())) {
                warn!(
                    "Inserting sqlite Operations possibly already exists: {}",
                    ThothErrors::from(e)
                );
            };
        }

        let sql_step = SqlSteps::find_by_id(step_id.clone());
        if sql_step.clone().is_none_or(|stp| !stp.is_finished) {
            let mut sql_step_model = match sql_step {
                Some(sql_mod) => sql_mod.into_active_model(),
                None => {
                    let inserted_stp =
                        SqlSteps::insert_row(SqlSteps::new(step_id.clone(), op_id.clone()));
                    match inserted_stp {
                        Ok(stp) => stp.into_active_model(),
                        Err(e) => {
                            err!("Inserting step, possibly already exists: {}", e);
                            SqlSteps::find_by_id(step_id.clone())
                                .unwrap()
                                .into_active_model()
                        }
                    }
                } // {
                  //     warn!("Inserting Step, possibly already exists: {}",e);
                  //     return SqlSteps::find_by_id(step_id.clone()).unwrap().into_active_model();

                  // }
            };

            let _ = self.op_file_manager.write(Arc::clone(&step), false); // Ignoring this error as it's not critical.
            // Translate the steps into a result.
            let step = DutiesTranslator::translate_step(Arc::clone(&step)); //I think we don't need to return it as it's mutable by reference.
            // Update steps files.
            self.op_file_manager
                .write(Arc::clone(&step), false)
                .unwrap();
            let extra_info = step.try_read().unwrap().extra_info.clone();
            if extra_info.is_some() {
                let extra_info = extra_info.unwrap();
                let res_pos: Option<String> = match serde_json::to_string(&extra_info.res_pos) {
                    Ok(pos) => Some(pos),
                    Err(e) => {
                        warn!(
                            "Faild to encode extra info res position into string: {}",
                            ThothErrors::from(e)
                        );
                        None
                    }
                };
                sql_step_model.res_pos = Set(res_pos);

                let res_type: Option<String> = match serde_json::to_string(&extra_info.res_type) {
                    Ok(rs_type) => Some(rs_type),
                    Err(e) => {
                        warn!(
                            "Faild to encode extra info res type into string: {}",
                            ThothErrors::from(e)
                        );
                        None
                    }
                };
                sql_step_model.res_type = Set(res_type);
            }
            sql_step_model.is_finished = Set(true);
            SqlSteps::update_row(sql_step_model).unwrap();
        }
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
                let step = OperationsFileManager::load_step_file(
                    &duty.operation_id,
                    &duty.step_id.clone(),
                )
                .unwrap(); //You might get it from the sqlite, possible you should not use the sqlite, it feels limited to it's one thread access in it's nature.
                if step.result.is_none() {
                    self.execute_step(Arc::new(RwLock::new(step)));
                }
                DbOpsRegisterer::finished_duty(duty.step_id.clone());
            }
            sql_ops_model.is_finished = ActiveValue::Set(true);
            let _ = SqlOperations::update_row(sql_ops_model);
        }
    }
}

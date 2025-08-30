use crate::{
    connections::channels_node_info::get_current_node_cloned,
    db::controller::registerer::DbOpsRegisterer,
    debug, info,
    operations::{
        checker::decrease_running_operation,
        executer::types::Executer,
        gatherer::structs::{GatheredMessage, GatheredResponse},
        planner::charts::structs::{NodesOpsMsg, Steps},
        translator::translate::DutiesTranslator,
    },
    router::post_offices::nodes_info::post_office::reply_gather_res,
    warn,
};
use tokio::sync::RwLock;

use std::sync::Arc;

impl Executer {
    pub async fn execute_step(&mut self, step: Arc<RwLock<Steps>>) {
        let step_id = step.read().await.step_id.clone();
        let op_id = step.read().await.operation_id.clone();
        debug!("Executing step id: {}", step_id);
        let is_op_exists = DbOpsRegisterer::get_operation_file(&op_id).is_some();
        if !is_op_exists {
            debug!("Operation id doesn't exists in SQL, will insert new one");
            DbOpsRegisterer::new_operation(op_id.clone(), true);
        }

        let file_step = DbOpsRegisterer::get_step_file(&op_id, &step_id);
        if file_step.is_some_and(|stp| !stp.result.is_none()) {
            info!(
                "Step id {} already has a result, skipping execution.",
                step_id
            );
            return;
        }
        // let ev_hand = EventsHandler::new(&format!("write_{}",step_id)).add_event(true);
        DbOpsRegisterer::new_step(Arc::clone(&step), false).await; // Ignoring this error as it's not critical.
        let step = DutiesTranslator::translate_step(Arc::clone(&step)).await; //I think we don't need to return it as it's mutable by reference.
        DbOpsRegisterer::new_step(Arc::clone(&step), false).await;

        decrease_running_operation(&op_id);
        let read_guard = step.read().await;
        let res = match read_guard.result.as_ref() {
            Some(rs) => Some(rs.clone()),
            None => None,
        };
        let use_prev_res = read_guard.use_prev_res.clone();
        let extra_info = read_guard.extra_info.clone();
        drop(read_guard);
        debug!(
            "Step Executer, will send response back to gatherer: {:?}",
            res
        );
        // ev_hand.listener.wait_for_event();
        reply_gather_res(GatheredMessage {
            operation_id: op_id,
            step_id,
            respond: Some(GatheredResponse {
                use_prev_res: use_prev_res,
                extra_info: extra_info,
                result: res,
            }),
        }); // Returning it now in case it finished before.
    }

    pub async fn execute_duties(&mut self, duties: Box<NodesOpsMsg>) {
        debug!("Will execute duties assigned to this node.");
        // Check for every step result.
        if let Some(node_duties) = duties.nodes_duties.get(&get_current_node_cloned().id) {
            DbOpsRegisterer::new_duties(&duties, false);
            debug!("Started executing duties assigned to this node.");
            let op_id = node_duties[0].operation_id.clone();
            // let mut sql_ops_model = SqlOperations::new(op_id.clone());
            if DbOpsRegisterer::get_operation_file(&op_id).is_none() {
                DbOpsRegisterer::new_operation(op_id.clone(), true);
            }
            for duty in node_duties.iter() {
                // DutiesTranslator::new(node_duty)
                // while DbOpsRegisterer::get_step_file(&duty.operation_id, &duty.step_id).is_none() {
                //     //Will wait for one second, maybe not all messages weren't processed. //TODO There's a potential an error happened and might cause the process to keep waiting.
                //     thread::sleep(Duration::from_secs(1));
                // }
                match DbOpsRegisterer::get_step_file(&duty.operation_id, &duty.step_id.clone()) {
                    //You might get it from the sqlite, possible you should not use the sqlite, it feels limited to it's one thread access in it's nature.
                    Some(step) => {
                        if step.result.is_none() {
                            self.execute_step(Arc::new(RwLock::new(step))).await;
                        }
                    }
                    None => {
                        warn!("Faild to load step file for step id: {}", duty.step_id);
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

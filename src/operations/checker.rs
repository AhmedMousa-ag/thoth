/*
his module checks if an operations or a step have been done before or not and return the result.
*/
use crate::{
    db::controller::traits::{SQLiteDBTraits, SqlNodesDuties, SqlOperations},
    errors::thot_errors::ThothErrors,
    operations::planner::charts::structs::{NodesOpsMsg, OperationInfo},
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub struct PlanChecker {}
impl PlanChecker {
    pub fn is_planned_before(operation_id: String) -> bool {
        let model = match SqlOperations::find_by_id(operation_id.clone()) {
            Some(model) => model,
            None => return false,
        };

        model.is_finished
    }

    pub fn get_planned_duties_db(operation_id: String) -> Result<Box<NodesOpsMsg>, ThothErrors> {
        let mut node_msgs: HashMap<String, Arc<RwLock<Vec<OperationInfo>>>> = HashMap::new();
        for duty in SqlNodesDuties::find_all_duties(operation_id.clone()).iter() {
            let ops_info = OperationInfo {
                operation_id: duty.op_id.clone(),
                step_id: duty.step_id.clone(),
            };
            match node_msgs.get(&duty.node_id) {
                Some(ops) => ops.try_write()?.push(ops_info.clone()),
                None => {
                    node_msgs.insert(duty.node_id.clone(), Arc::new(RwLock::new(vec![ops_info])));
                }
            };
        }

        return Ok(Box::new(NodesOpsMsg {
            nodes_duties: node_msgs,
        }));
    }
}

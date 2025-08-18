/*
his module checks if an operations or a step have been done before or not and return the result.
*/
use crate::{
    db::controller::{registerer::DbOpsRegisterer, traits::SqlNodesDuties},
    debug,
    errors::thot_errors::ThothErrors,
    operations::planner::charts::structs::{NodesOpsMsg, OperationInfo},
};
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

lazy_static!(
    /// This is a global checker for the operations.
    pub static ref RUNNING_OPERATIONS: Arc<RwLock<HashMap<String, u64>>> = Arc::new(RwLock::new(HashMap::new()));
);

pub fn increase_running_operation(operation_id: String) {
    debug!("Increase running operation: {}", operation_id);
    let mut running_operations = RUNNING_OPERATIONS.write().unwrap();
    let num_operations = running_operations.get(&operation_id).unwrap_or(&0).clone();
    running_operations.insert(operation_id, num_operations + 1);
}

pub fn decrease_running_operation(operation_id: String) {
    let mut running_operations = RUNNING_OPERATIONS.write().unwrap();
    let num_operations = running_operations.get(&operation_id).unwrap_or(&0).clone();
    if num_operations > 0 {
        running_operations.insert(operation_id, num_operations - 1);
    }
}
pub fn is_internal_ops_finished(operation_id: String) -> bool {
    RUNNING_OPERATIONS
        .read()
        .unwrap()
        .get(&operation_id)
        .is_some_and(|&num| num == 0)
}
pub fn get_num_running_operations(operation_id: String) -> u64 {
    RUNNING_OPERATIONS
        .read()
        .unwrap()
        .get(&operation_id)
        .cloned()
        .unwrap_or(0)
}

pub struct PlanChecker {}
impl PlanChecker {
    pub fn is_planned_before(operation_id: String) -> bool {
        DbOpsRegisterer::get_operation_file(&operation_id).is_some_and(|op| op.result.is_some())
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

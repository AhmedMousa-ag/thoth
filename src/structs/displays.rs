use crate::operations::{
    executer::types::OperationTypes,
    planner::charts::structs::{NodesOpsMsg, Numeric, OperationInfo, Steps},
};

use super::structs::{Message, NodeInfo, RequestsTypes};
use std::fmt;
impl fmt::Display for NodeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(
            f,
            "Id: {}, Available Threads: {}, Available Ram: {}",
            self.id, self.av_threads, self.av_ram
        )
    }
}

impl fmt::Display for RequestsTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Request Type: {}, ", self.as_str())
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Topic Name: {}, Request Type: {}",
            self.topic_name, self.request
        )
    }
}

impl fmt::Display for OperationTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl fmt::Display for Numeric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
impl fmt::Display for Steps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut msg = String::new();
        if let Some(x) = &self.x {
            msg.push_str(&format!("X: {}, ", x));
        }
        if let Some(y) = &self.y {
            msg.push_str(&format!("Y: {}, ", y));
        }
        if let Some(next_step) = &self.next_step {
            msg.push_str(&format!("Next Step: {}", next_step));
        }
        if let Some(prev_step) = &self.prev_step {
            msg.push_str(&format!("\n<= Previous Step => {}", prev_step));
        }

        write!(f, "\nOperation Type: {}, {}", self.op_type, msg)
    }
}
impl fmt::Display for OperationInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Operation Id: {}, Step Id: {}",
            self.operation_id, self.step_id
        )
    }
}

impl fmt::Display for NodesOpsMsg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut msg = String::new();
        for node_id in self.nodes_duties.keys() {
            let mut node_duties = String::new();
            for node_duty in self.nodes_duties[node_id].try_read().unwrap().iter() {
                node_duties.push_str(&format!("{} ", node_duty));
            }
            msg.push_str(&format!("Node Id: {}, With Duty: {}", node_id, node_duties));
        }
        write!(f, "{}", msg)
    }
}

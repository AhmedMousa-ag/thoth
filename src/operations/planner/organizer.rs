use super::charts::plans::{NodesOpsMsg, Steps};
use crate::operations::executer::base_operations::OperationTypes;
use crate::operations::planner::charts::plans::Numeric;
use crate::structs::structs::NodeInfo;
use crate::{connections::channels_node_info::get_nodes_info_cloned, info};
use std::collections::HashMap;

pub struct Planner {
    nodes_info: HashMap<std::string::String, NodeInfo>,
}

impl Planner {
    pub fn new() -> Self {
        info!("Started new planer");
        Self {
            nodes_info: get_nodes_info_cloned(),
        }
    }
    pub fn send_message(&self, msg: &NodesOpsMsg) {}
    pub fn plan_matrix_multiply(&self, x: Vec<Vec<f64>>, y: Vec<Vec<f64>>) {
        //TODO
        // debug!("Will start multiply");
        // let n = x.len();
        // if n==1{
        //     x[0][0]*y[0][0];
        // }
        // let mut new_size = n /2;
        // while new_size>2 {

        //     new_size/=2;
        // }
    }

    pub fn plan_average(&self, x: Vec<f64>) {
        let data_size = x.len();
        let nodes_num = self.nodes_info.keys().len(); //It shall never be zero as the current node is one.
        let ops_slice_size = data_size / nodes_num;
        let mut idx = 0;
        let mut nodes_keys = self.nodes_info.keys();
        let mut node_idx = 0;
        let mut nodes_msgs = HashMap::new();

        while idx < data_size {
            if node_idx > nodes_num {
                node_idx = 0;
            }
            let node_data = x[idx..ops_slice_size].to_vec();
            let mut step_two = Box::new(Steps {
                x: None,
                y: Some(Numeric::Number(node_data.len() as f64)),
                op_type: OperationTypes::DIVIDE,
                result: None,
                next_step: None,
                prev_step: None,
                use_prev_res: true,
            });
            let step_one = Box::new(Steps {
                x: Some(Numeric::Vector(node_data)),
                y: None,
                op_type: OperationTypes::SUM,
                result: None,
                use_prev_res: false,
                prev_step: None,
                next_step: Some(Box::clone(&step_two)),
            });
            step_two.prev_step = Some(Box::clone(&step_one));

            match nodes_keys.next() {
                Some(key) => {
                    let node_id = self.nodes_info.get(key).unwrap().id.clone();
                    nodes_msgs.insert(node_id, step_one);

                    idx += ops_slice_size;
                    node_idx += 1;
                }
                None => break,
            }
        }
        info!("Finished planning: {:?}", nodes_msgs);
        self.send_message(&NodesOpsMsg {
            nodes_duties: nodes_msgs,
        });
    }
}

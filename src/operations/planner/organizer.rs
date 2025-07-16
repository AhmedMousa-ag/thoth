use super::charts::structs::{NodesOpsMsg, Steps};
use crate::connections::configs::topics::TopicsEnums;
use crate::logger::writters::writter::{FileTypes, OperationsFileManager};
use crate::operations::executer::types::{Executer, OperationTypes};
use crate::operations::planner::charts::structs::{ExtraInfo, Numeric, OperationInfo};
use crate::operations::utils::util;
use crate::router::post_offices::external_com_ch::ExternalComm;
use crate::router::post_offices::nodes_info::channel::InternalCommunications;
use crate::router::post_offices::nodes_info::post_office::{
    OperationStepExecuter, OperationsExecuterOffice,
};
use crate::router::traits::{PostOfficeTrait, SenderReciverTrait};
use crate::structs::structs::{Message, NodeInfo, RequestsTypes};
use crate::structs::traits::EncodingDecoding;
use crate::{connections::channels_node_info::get_nodes_info_cloned, info};
use crate::{debug, warn};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

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

    pub fn plan_matrix_naive_multiply(
        &self,
        x: Vec<Vec<Box<f64>>>,
        mut y: Vec<Vec<Box<f64>>>,
        operation_id: String,
    ) {
        info!("Will start planning naive multiply");
        let nodes_keys: Vec<String> = self.nodes_info.keys().map(|s| s.clone()).collect();
        let nodes_num = nodes_keys.len();
        let mut executer: Option<Executer> = if nodes_num >= 1 {
            warn!(
                "Only one node available which is considered usesless for Thoth to handle this operation"
            );
            Some(Executer {
                op_file_manager: OperationsFileManager::new(operation_id.clone()).unwrap(),
            })
        } else {
            None
        };
        let mut node_idx = 0;
        let mut nodes_duties: HashMap<String, Rc<RefCell<Vec<OperationInfo>>>> = HashMap::new();

        if x.is_empty() || y.is_empty() {
            warn!("Empty Vectors");
            // TODO handle this situation with the error system.
        }
        let y_row_len = y.len();
        let x_col_len = x.get(0).unwrap_or(&vec![]).len();
        debug!("Y before: {:?}", y);
        if x_col_len != y_row_len {
            debug!("Is transposing");
            // TODO check if transponsing will be beneficial in terms of the deminsions then throw an error if it doesn't.
            y = util::transpose(y);
        }
        let mut prev_step: Option<Rc<RefCell<Steps>>> = None;

        for (irow, row) in x.iter().enumerate() {
            //Every row by every column
            debug!("Will do  row: {}", irow);
            for icol in 0..y_row_len {
                //Iterate every column
                debug!("Will get the columns vectors");
                let col: Vec<Box<f64>> = y.iter().map(|yrow| Box::new(*yrow[icol])).collect();
                debug!("Finished the columns vectors");
                debug!("Will multiply: {:?} by {:?}", row, col);
                let node_id = util::get_node_id(&mut node_idx, nodes_num, &nodes_keys);
                let step_id = Uuid::new_v4().to_string();
                let step: Rc<RefCell<Steps>> = Rc::new(RefCell::new(Steps {
                    node_id: node_id.to_string(),
                    operation_id: operation_id.clone(),
                    step_id: step_id.clone(),
                    x: Some(Numeric::Vector(row.to_vec())),
                    y: Some(Numeric::Vector(col)),
                    op_type: OperationTypes::DOT,
                    result: None,
                    next_step: None,
                    prev_step: None,
                    use_prev_res: false,
                    extra_info: Some(ExtraInfo {
                        res_pos: Some(vec![irow as u64, icol as u64]),
                    }),
                }));

                if let Some(prev) = prev_step {
                    step.borrow_mut().prev_step = Some(prev.borrow().step_id.to_string());
                    prev.borrow_mut().next_step = Some(step.borrow().step_id.to_string());
                }

                prev_step = Some(Rc::clone(&step));
                let op_msg = OperationInfo {
                    operation_id: operation_id.clone(),
                    step_id,
                };

                if let Some(exec) = &mut executer {
                    exec.execute_step(step.clone());
                    continue;
                } else {
                    OperationStepExecuter::send_message(step);
                }
                match nodes_duties.get(&node_id) {
                    Some(msg_vec) => msg_vec.borrow_mut().push(op_msg),
                    None => {
                        nodes_duties.insert(node_id, Rc::new(RefCell::new(vec![op_msg])));
                    }
                }
                debug!("Finished row: {} and col: {}", irow, icol);
            }
            debug!("Finished row: {}", irow);
        }
        debug!("Finished all rows and stuff");

        let nodes_ops_msg = Box::new(NodesOpsMsg { nodes_duties });
        info!("Finished planning: {}", nodes_ops_msg);
        if let Some(exec) = &mut executer {
            exec.execute_duties(nodes_ops_msg);
            return;
        }
        OperationsExecuterOffice::send_message(nodes_ops_msg);
    }

    pub fn plan_average(&self, x: Vec<Box<f64>>, operation_id: String) {
        let data_size = x.len();
        let nodes_keys: Vec<String> = self.nodes_info.keys().map(|s| s.clone()).collect();
        let nodes_num = nodes_keys.len(); //It shall never be zero as the current node is one.
        let mut executer: Option<Executer> = if nodes_num >= 1 {
            warn!(
                "Only one node available which is considered usesless for Thoth to handle this operation"
            );

            Some(Executer {
                op_file_manager: OperationsFileManager::new(operation_id.clone()).unwrap(),
            })
        } else {
            None
        };
        let ops_slice_size = data_size / nodes_num;
        let mut idx = 0;
        let mut node_idx = 0;
        let mut nodes_duties: HashMap<String, Rc<RefCell<Vec<OperationInfo>>>> = HashMap::new();

        while idx < data_size {
            let first_step_node_id = util::get_node_id(&mut node_idx, nodes_num, &nodes_keys);
            let first_step_id = Uuid::new_v4().to_string();
            let node_data = x[idx..ops_slice_size].to_vec();
            let data_len = node_data.len() as f64;
            let step_one = Rc::new(RefCell::new(Steps {
                operation_id: operation_id.clone(),
                step_id: first_step_id.clone(),
                node_id: first_step_node_id.to_string(),
                x: Some(Numeric::Vector(node_data)),
                y: None,
                op_type: OperationTypes::SUM,
                result: None,
                use_prev_res: false,
                prev_step: None,
                next_step: None,
                extra_info: None,
            }));
            let step_two: Rc<RefCell<Steps>> = Rc::new(RefCell::new(Steps {
                node_id: util::get_node_id(&mut node_idx, nodes_num, &nodes_keys),
                operation_id: operation_id.clone(),
                step_id: Uuid::new_v4().to_string(),
                x: None,
                y: Some(Numeric::Scaler(Box::new(data_len))),
                op_type: OperationTypes::DIVIDE,
                result: None,
                next_step: None,
                prev_step: None,
                use_prev_res: true,
                extra_info: None,
            }));
            step_one.borrow_mut().next_step = Some(step_two.borrow().step_id.to_string());
            step_two.borrow_mut().prev_step = Some(step_one.borrow().step_id.to_string());

            let op_msg = OperationInfo {
                operation_id: operation_id.clone(),
                step_id: first_step_id,
            };
            if let Some(exec) = &mut executer {
                exec.execute_step(step_one);
            } else {
                OperationStepExecuter::send_message(step_one);
            }
            match nodes_duties.get(&first_step_node_id) {
                Some(msg_vec) => msg_vec.borrow_mut().push(op_msg),
                None => {
                    nodes_duties.insert(first_step_node_id, Rc::new(RefCell::new(vec![op_msg])));
                }
            }

            idx += ops_slice_size;
        }

        info!("Finished planning: {:?}", nodes_duties);
        let nodes_ops_msg = Box::new(NodesOpsMsg { nodes_duties });
        if let Some(exec) = &mut executer {
            exec.execute_duties(nodes_ops_msg);
            return;
        }
        OperationsExecuterOffice::send_message(nodes_ops_msg);
    }
}

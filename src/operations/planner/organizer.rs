use super::charts::plans::{NodesOpsMsg, Steps};
use crate::connections::configs::topics::TopicsEnums;
use crate::operations::executer::base_operations::OperationTypes;
use crate::operations::planner::charts::plans::{ExtraInfo, Numeric};
use crate::operations::utils::util;
use crate::router::post_offices::external_com_ch::ExternalComm;
use crate::structs::structs::{Message, NodeInfo, RequestsTypes};
use crate::structs::traits::EncodingDecoding;
use crate::{connections::channels_node_info::get_nodes_info_cloned, info};
use crate::{debug, warn};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
    pub fn send_message(&self, msg: Box<NodesOpsMsg>) {
        let nodes_msg = Box::new(Message {
            topic_name: TopicsEnums::OPERATIONS.to_string(),
            request: RequestsTypes::PlansToExecute,
            message: Some(msg.encode_bytes()),
        });
        ExternalComm::send_message(nodes_msg);
        info!("Sent plans to be executed.")
    }
    pub fn plan_matrix_naive_multiply(&self, x: Vec<Vec<Box<f64>>>, mut y: Vec<Vec<Box<f64>>>) {
        info!("Will start planning naive multiply");
        let nodes_keys: Vec<String> = self.nodes_info.keys().map(|s| s.clone()).collect();
        let nodes_num = nodes_keys.len();
        let mut node_idx = 0;
        let mut nodes_msgs = HashMap::new();

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

                let step: Rc<RefCell<Steps>> = Rc::new(RefCell::new(Steps {
                    x: Some(Numeric::Vector(row.to_vec())),
                    y: Some(Numeric::Vector(col)),
                    op_type: OperationTypes::DOT,
                    result: None,
                    next_step: None,
                    prev_step: None, //if prev_step.is_some() { Some(Rc::clone(&prev_step.unwrap())) } else { None } ,
                    use_prev_res: false,
                    extra_info: Some(ExtraInfo {
                        res_pos: Some(vec![irow as u64, icol as u64]),
                    }),
                }));

                if let Some(prev) = prev_step {
                    step.borrow_mut().prev_step = Some(Rc::clone(&prev));
                    prev.borrow_mut().next_step = Some(Rc::clone(&step));
                }

                prev_step = Some(Rc::clone(&step));
                let node_id = util::get_node_id(&mut node_idx, nodes_num, &nodes_keys);
                nodes_msgs.insert(node_id, step);
                debug!("Finished row: {} and col: {}", irow, icol);
            }
            debug!("Finished row: {}", irow);
        }
        debug!("Finished all rows and stuff");

        let nodes_ops_msg = Box::new(NodesOpsMsg {
            nodes_duties: nodes_msgs,
        });
        info!("Finished planning: {}", nodes_ops_msg);
        self.send_message(nodes_ops_msg);
    }

    pub fn plan_average(&self, x: Vec<Box<f64>>) {
        let data_size = x.len();
        let nodes_keys: Vec<String> = self.nodes_info.keys().map(|s| s.clone()).collect();
        let nodes_num = nodes_keys.len(); //It shall never be zero as the current node is one.
        //TODO if nodes_num==1 then send it to the executre and return.
        let ops_slice_size = data_size / nodes_num;
        let mut idx = 0;
        let mut node_idx = 0;
        let mut nodes_msgs = HashMap::new();

        while idx < data_size {
            let node_data = x[idx..ops_slice_size].to_vec();
            let step_two: Rc<RefCell<Steps>> = Rc::new(RefCell::new(Steps {
                x: None,
                y: Some(Numeric::Number(Box::new(node_data.len() as f64))),
                op_type: OperationTypes::DIVIDE,
                result: None,
                next_step: None,
                prev_step: None,
                use_prev_res: true,
                extra_info: None,
            }));
            let step_one = Rc::new(RefCell::new(Steps {
                x: Some(Numeric::Vector(node_data)),
                y: None,
                op_type: OperationTypes::SUM,
                result: None,
                use_prev_res: false,
                prev_step: None,
                next_step: Some(Rc::clone(&step_two)),
                extra_info: None,
            }));
            step_two.borrow_mut().prev_step = Some(Rc::clone(&step_one));

            let node_id = util::get_node_id(&mut node_idx, nodes_num, &nodes_keys);
            nodes_msgs.insert(node_id, step_one);

            idx += ops_slice_size;
        }
        info!("Finished planning: {:?}", nodes_msgs);
        let nodes_ops_msg = Box::new(NodesOpsMsg {
            nodes_duties: nodes_msgs,
        });
        self.send_message(nodes_ops_msg);
    }
}

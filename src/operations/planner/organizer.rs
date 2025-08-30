use crate::{
    connections::channels_node_info::{NodeInfoTrait, get_nodes_info_cloned},
    db::controller::registerer::DbOpsRegisterer,
    debug,
    errors::thot_errors::ThothErrors,
    info,
    logger::writters::writter::OperationsFileManager,
    operations::{
        checker::{PlanChecker, increase_running_operation},
        executer::types::{Executer, OperationTypes},
        planner::charts::structs::{ExtraInfo, NodesOpsMsg, OperationInfo, Steps},
        utils::util,
    },
    router::{
        post_offices::nodes_info::post_office::{OperationStepExecuter, OperationsExecuterOffice},
        traits::PostOfficeTrait,
    },
    structs::{numerics::structs::Numeric, structs::NodeInfo},
    warn,
};
use std::{collections::HashMap, sync::Arc, vec};
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct Planner {
    nodes_info: HashMap<std::string::String, NodeInfo>,
    operation_id: String,
}

impl Planner {
    pub fn new(operation_id: String) -> Self {
        info!("Started new planer");
        NodeInfo::request_other_nodes_info(); // I think it's useless, it takes time to respond, TODO consider event driven or get the reference of the node (I don't like it, the guard will stay for long time.)
        Self {
            nodes_info: get_nodes_info_cloned(),
            operation_id,
        }
    }

    pub async fn plan_matrix_naive_multiply(
        &self,
        x: Vec<Vec<f64>>,
        mut y: Vec<Vec<f64>>,
    ) -> Result<Box<NodesOpsMsg>, ThothErrors> {
        if PlanChecker::is_planned_before(self.operation_id.clone()) {
            return PlanChecker::get_planned_duties_db(self.operation_id.clone());
        }
        info!("Will start planning naive multiply");
        let nodes_keys: Vec<String> = self.nodes_info.keys().map(|s| s.clone()).collect();
        debug!("Available Nodes: {:?}", nodes_keys);
        let nodes_num = nodes_keys.len();
        info!("Available Nodes number is: {:?}", nodes_num);
        let mut executer: Option<Executer> = if nodes_num <= 1 {
            warn!(
                "Only one node available which is considered usesless for Thoth to handle this operation"
            );
            Some(Executer {
                op_file_manager: OperationsFileManager::new(&self.operation_id),
            })
        } else {
            None
        };
        let mut node_idx = 0;
        let mut nodes_duties: HashMap<String, Vec<OperationInfo>> = HashMap::new();

        if x.is_empty() || y.is_empty() {
            warn!("Empty Vectors");
            // TODO handle this situation with the error system.
        }
        let y_row_len = y.len();
        let x_col_len = x.get(0).unwrap_or(&vec![]).len();
        if x_col_len != y_row_len {
            // TODO check if transponsing will be beneficial in terms of the deminsions then throw an error if it doesn't.
            y = util::transpose(y);
        }
        let mut prev_step: Option<Arc<RwLock<Steps>>> = None;

        for (irow, row) in x.iter().enumerate() {
            //Every row by every column
            for icol in 0..y_row_len - 1 {
                //Iterate every column
                let col: Vec<f64> = y.iter().map(|yrow| yrow[icol]).collect();
                let node_id = util::get_node_id(&mut node_idx, nodes_num, &nodes_keys);
                let step_id = Uuid::new_v4().to_string();
                let step: Arc<RwLock<Steps>> = Arc::new(RwLock::new(Steps {
                    node_id: node_id.to_string(),
                    operation_id: self.operation_id.clone(),
                    step_id: step_id.clone(),
                    x: Some(Numeric::Vector(row.to_vec())),
                    y: Some(Numeric::Vector(col)),
                    op_type: OperationTypes::DOT,
                    result: None,
                    next_step: None,
                    prev_step: None,
                    use_prev_res: false,
                    extra_info: Some(ExtraInfo {
                        res_pos: Some(vec![irow, icol]),
                        res_type: Some(Numeric::Matrix(vec![vec![]])),
                    }),
                }));

                if let Some(prev) = prev_step {
                    step.write().await.prev_step = Some(prev.read().await.step_id.to_string());
                    prev.write().await.next_step = Some(step.read().await.step_id.to_string());
                }

                prev_step = Some(Arc::clone(&step));
                let op_msg = OperationInfo {
                    operation_id: self.operation_id.clone(),
                    step_id: step_id.clone(),
                };

                if let Some(exec) = &mut executer {
                    warn!("Will execute step internally");
                    // DbOpsRegisterer::new_step(step, false);
                    increase_running_operation(self.operation_id.clone());
                    exec.execute_step(Arc::clone(&step)).await;
                } else {
                    info!("Will send an execution step");
                    OperationStepExecuter::send_message(Arc::clone(&step));
                }
                match nodes_duties.get_mut(&node_id) {
                    Some(msg_vec) => msg_vec.push(op_msg),
                    None => {
                        nodes_duties.insert(node_id, vec![op_msg]);
                    }
                }
            }
        }
        let nodes_ops_msg = Box::new(NodesOpsMsg { nodes_duties });
        info!("Finished planning: {}", nodes_ops_msg);
        if let Some(exec) = &mut executer {
            exec.execute_duties(nodes_ops_msg.clone()).await;
        } else {
            info!("Will send an execution message");
            OperationsExecuterOffice::send_message(nodes_ops_msg.clone());
        }
        Ok(nodes_ops_msg)
    }

    pub async fn plan_average(&self, x: Vec<f64>) -> Result<Box<NodesOpsMsg>, ThothErrors> {
        if PlanChecker::is_planned_before(self.operation_id.clone()) {
            info!("Already planned, will return.");
            return PlanChecker::get_planned_duties_db(self.operation_id.clone());
        }
        let data_size = x.len();
        let nodes_keys: Vec<String> = self.nodes_info.keys().map(|s| s.clone()).collect();
        debug!("Available Nodes: {:?}", nodes_keys);
        let nodes_num = nodes_keys.len(); //It shall never be zero as the current node is one.
        debug!("Available nodes number: {}", nodes_num);
        let mut executer: Option<Executer> = if nodes_num <= 1 {
            warn!(
                "Only one node available which is considered usesless for Thoth to handle this operation"
            );
            Some(Executer {
                op_file_manager: OperationsFileManager::new(&self.operation_id),
            })
        } else {
            None
        };
        let ops_slice_size = data_size / nodes_num;
        let mut idx = 0;
        let mut node_idx = 0;
        let mut nodes_duties: HashMap<String, Vec<OperationInfo>> = HashMap::new();

        while idx < data_size {
            let node_id = util::get_node_id(&mut node_idx, nodes_num, &nodes_keys);
            // let second_step_node_id = util::get_node_id(&mut node_idx, nodes_num, &nodes_keys);
            let step_id = Uuid::new_v4().to_string();

            // [1,2,3,4,5]
            // [1,2],[3,4],[5]
            let node_data = if idx + ops_slice_size < data_size {
                x[idx..idx + ops_slice_size].to_vec()
            } else {
                x[idx..].to_vec()
            };

            let step_one = Arc::new(RwLock::new(Steps {
                operation_id: self.operation_id.clone(),
                step_id: step_id.clone(),
                node_id: node_id.to_string(),
                x: Some(Numeric::Vector(node_data)),
                y: None,
                // Will not use AVG as it will be calculated in the gatherer. Also dividing several operations across several nodes losses important fractions between the operations.
                op_type: OperationTypes::SUM,
                result: None,
                use_prev_res: false,
                prev_step: None,
                next_step: None,
                extra_info: None,
            }));
            debug!("Planning--->Step one: {:?}", step_one);

            let op_msg = OperationInfo {
                operation_id: self.operation_id.clone(),
                step_id: step_id.clone(),
            };
            if let Some(exec) = &mut executer {
                increase_running_operation(self.operation_id.clone());
                exec.execute_step(step_one).await;
            } else {
                OperationStepExecuter::send_message(step_one.clone());
                DbOpsRegisterer::new_step(step_one, true).await;
            }
            match nodes_duties.get_mut(&node_id) {
                Some(msg_vec) => msg_vec.push(op_msg),
                None => {
                    nodes_duties.insert(node_id, vec![op_msg]);
                }
            }

            idx += ops_slice_size;
        }

        info!("Finished planning: {:?}", nodes_duties);
        let nodes_ops_msg = Box::new(NodesOpsMsg { nodes_duties });
        if let Some(exec) = &mut executer {
            exec.execute_duties(nodes_ops_msg.clone()).await;
            // return;
        } else {
            OperationsExecuterOffice::send_message(nodes_ops_msg.clone());
        }
        Ok(nodes_ops_msg)
    }
}

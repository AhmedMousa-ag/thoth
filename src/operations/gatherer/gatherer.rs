use crate::{
    connections::channels_node_info::get_nodes_info_cloned, db::controller::registerer::DbOpsRegisterer, debug, err, errors::thot_errors::ThothErrors, grpc::grpc_server::mathop::{Matrix, MatrixRow}, info, logger::writters::writter::OperationsFileManager, operations::{
        checker::{get_num_running_operations, is_internal_ops_finished},
        gatherer::{
            channels::{add_ch_sender, get_opened_ch_sender},
            structs::{GatheredMessage, GatheredResponse, Gatherer},
        },
        planner::charts::structs::NodesOpsMsg,
        // utils::util::load_sql_step_to_gatherer_res,
    }, router::{post_offices::nodes_info::post_office::GathererOffice, traits::PostOfficeTrait}, structs::numerics::structs::Numeric
};
use tokio::{
    select, spawn,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
};

impl Gatherer {
    pub fn new(operation_id: String) -> Self {
        let channels: (
            UnboundedSender<GatheredMessage>,
            UnboundedReceiver<GatheredMessage>,
        ) = mpsc::unbounded_channel();
        add_ch_sender(operation_id.clone(), channels.0);
        Self {
            reciever_ch: channels.1,
        }
    }
    pub fn reply_gathered_msg(mut message: GatheredMessage) -> Option<GatheredMessage> {
        let res =
            match OperationsFileManager::load_step_file(&message.operation_id, &message.step_id) {
                Ok(stp) => {
                    if stp.result.is_some() {
                        GatheredResponse {
                            result: stp.result.unwrap(),
                            use_prev_res: stp.use_prev_res,
                            extra_info: stp.extra_info,
                        }
                    } else {
                        return None;
                    }
                }
                Err(_) => return None,
            };
        message.respond = Some(res);
        Some(message)
    }
    // TODO you might move it outside of this struct, but I don't see it worth it.
    fn ask_nodes_their_results(plan: Box<NodesOpsMsg>) -> Result<usize, ThothErrors> {
        let mut num_sent_message = 0;
        debug!("Plan Nodes Duties: {:?}", plan);
        //TODO Keep track of execution steps, then get the number of nodes, if only this one available, then wait until all of the steps are done.
        let num_nodes = get_nodes_info_cloned().len();
        for (_, op_infos) in plan.nodes_duties {
            for info in op_infos.try_read()?.clone() {
                num_sent_message += 1;
                if num_nodes == 1 && !is_internal_ops_finished(info.operation_id.clone()) {
                    debug!(
                        "Only one node available, waiting for the operation to finish: {}",
                        info.operation_id
                    );
                    loop {
                        if is_internal_ops_finished(info.operation_id.clone()) {
                            debug!("Operation finished, Will Break: {}", info.operation_id);
                            break;
                        }
                        // Wait until the operation is finished, then continue.
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        debug!(
                            "Waiting for the operation to finish: {}, num of operations: {}",
                            info.operation_id,
                            get_num_running_operations(info.operation_id.clone())
                        );
                    }
                }
                let step = DbOpsRegisterer::get_step_file(&info.operation_id,&info.step_id);
                debug!("SQL Step: {:?}", step);
                if !step.as_ref().is_none_or(|stp| stp.result.is_none()) {
                    // If it's one node or the result already exists on this node, then the step is already done. and exists. However it doesn't comply if there were multiple nodes.
                    info!(
                        "Step already done on this node, sending the result: {:?}",
                        info
                    );
                    let sql_step = step.unwrap();
                    let result = &sql_step.result;
                    let sender = get_opened_ch_sender(&info.operation_id);
                    if sender.is_some() && result.is_some() {
                        let gther_res = GatheredResponse { result: sql_step.result.unwrap(), use_prev_res: sql_step.use_prev_res, extra_info: sql_step.extra_info };
                        info!("Sending Gathered Message Internally: {:?}", gther_res);
                        if let Err(e) = sender.unwrap().send(GatheredMessage {
                            operation_id: info.operation_id,
                            step_id: info.step_id,
                            respond: Some(gther_res),
                        }) {
                            err!("Error sending Gathered Message: {}", ThothErrors::from(e));
                        };
                    };
                    // If succesfully sent the message then continue to the next element, otherwise get back to the main loop and ask the other nodes.
                    continue;
                }
                //Send the operation to all nodes, even if the result is None. which it's asking for the result.
                spawn(async {
                    info!(
                        "Sending message to gatherer channel to all nodes: {:?}",
                        info
                    );
                    //TODO handle a situation when the operation was executed on another node, but the node is not available anymore.
                    // let num_nodes = get_nodes_info_cloned().len();
                    // if num_nodes == 0 {

                    //     warn!("Gatherer: No nodes available to gather results from. Operation ID: {}", info.operation_id);
                    //     return;
                    // };
                    GathererOffice::send_message(GatheredMessage {
                        operation_id: info.operation_id,
                        step_id: info.step_id,
                        respond: None,
                    });
                });
            }
        }
        Ok(num_sent_message)
    }

    pub async fn gather_list_average(
        &mut self,
        plan: Box<NodesOpsMsg>,
    ) -> Result<f64, ThothErrors> {
        let num_duties = Self::ask_nodes_their_results(plan)?;
        let mut left_to_gather = num_duties.clone();
        let mut res = 0.0;
        let mut num_divide = 0.0;
        while left_to_gather > 0 {
            select! {
             result = self.reciever_ch.recv() => {
                 match result {
                     Some(value) => {
                        info!("Received: {:?}", value);
                        match value.respond{
                        Some(gath_res)=>{
                            let num:f64= gath_res.result.clone().into();
                            debug!("Gathered Result: {}", num);
                            if gath_res.use_prev_res{
                                // let prev_res= gath_res.prev_step_res.unwrap_or(Numeric::from(0.0)).get_scaler_value();
                                res +=  num;
                                num_divide += 1.0;
                            }
                            left_to_gather -= 1;
                        },
                        None=>continue,
                        }
                     }
                     None => {
                         // Channel closed
                         break;
                     }
                 }
             }
            }
        }
        debug!("Gathered List Average: {}, num_divide: {}", res, num_divide);
        res /= if num_divide != 0.0 { num_divide } else { 1.0 }; // Each step had it's own average, now after we gather each step average, we need to define the last average for all gathered steps.
        Ok(res)
    }

    pub async fn gather_matrix_multiply(
        &mut self,
        plan: Box<NodesOpsMsg>,
        (rows_dim, cols_dim): (usize, usize),
    ) -> Result<Matrix, ThothErrors> {
        let mut left_to_gather = Self::ask_nodes_their_results(plan)?;

        let mut res: Matrix = Matrix {
            rows: vec![
                MatrixRow {
                    values: vec![0.0; cols_dim]
                };
                rows_dim
            ],
        };
        // let mut left_to_gather = rows_dim * cols_dim;
        while left_to_gather > 0 {
            select! {
             result = self.reciever_ch.recv() => {
                 match result {
                     Some(value) => {
                        info!("Received: {:?}", value);
                        match value.respond{
                        Some(gath_res)=>{
                            let num = gath_res.result.get_scaler_value();
                            if let Some(extra_infos)=gath_res.extra_info{
                                let poses= extra_infos.res_pos.unwrap_or(vec![0,0]); // It shall never be None in case of metrics.
                                let (x_pos,y_pos) = (poses[0],poses[1]);
                                res.rows[x_pos].values[y_pos]= num;

                            }
                            left_to_gather-=1;

                        },
                        None=>continue,
                        }
                     }
                     None => {
                         // Channel closed
                         break;
                     }
                 }
             }
            }
        }
        Ok(res)
    }
}

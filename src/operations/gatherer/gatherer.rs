use std::{collections::HashMap, sync::Arc};

use crate::{
    connections::channels_node_info::{get_current_node_cloned, get_nodes_info_cloned},
    db::controller::registerer::DbOpsRegisterer,
    err,
    errors::thot_errors::ThothErrors,
    grpc::grpc_server::mathop::{Matrix, MatrixRow},
    info,
    operations::{
        checker::is_internal_ops_finished,
        executer::types::OperationsHelper,
        gatherer::{
            channels::{add_ch_sender, get_opened_ch_sender},
            structs::{GatheredMessage, GatheredResponse, Gatherer},
        },
        planner::charts::structs::{NodesOpsMsg, OperationInfo, Steps}, // utils::util::load_sql_step_to_gatherer_res,
    },
    router::{post_offices::nodes_info::post_office::GathererOffice, traits::PostOfficeTrait},
    warn,
};
use tokio::{
    select, spawn,
    sync::{
        RwLock,
        mpsc::{self, UnboundedReceiver, UnboundedSender},
    },
};

async fn get_result_internally(info: OperationInfo) {
    loop {
        //TODO make it event based rather than polling based.
        if is_internal_ops_finished(info.operation_id.clone()).await {
            info!("Operation finished, Will Break: {}", info.operation_id);
            break;
        }
        // Wait until the operation is finished, then continue.
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    let step = DbOpsRegisterer::get_step_file(&info.operation_id, &info.step_id);
    if !step.as_ref().is_none_or(|stp| stp.result.is_none()) {
        // If it's one node or the result already exists on this node, then the step is already done. and exists. However it doesn't comply if there were multiple nodes.
        info!(
            "Step already done on this node, sending the result: {:?}",
            info
        );
        let sql_step = step.unwrap();
        let result = &sql_step.result;
        let sender = get_opened_ch_sender(&info.operation_id).await;
        if sender.is_some() && result.is_some() {
            let gther_res = GatheredResponse {
                result: sql_step.result,
                use_prev_res: sql_step.use_prev_res,
                extra_info: sql_step.extra_info,
            };
            info!("Sending Gathered Message Internally: {:?}", gther_res);
            if let Err(e) = sender.unwrap().send(GatheredMessage {
                operation_id: info.operation_id,
                step_id: info.step_id,
                respond: Some(gther_res),
            }) {
                err!("Error sending Gathered Message: {}", ThothErrors::from(e));
            };
        };
    }
}

impl Gatherer {
    pub async fn new(operation_id: String) -> Self {
        let channels: (
            UnboundedSender<GatheredMessage>,
            UnboundedReceiver<GatheredMessage>,
        ) = mpsc::unbounded_channel();
        add_ch_sender(operation_id.clone(), channels.0).await;
        Self {
            reciever_ch: channels.1,
        }
    }
    pub async fn reply_gathered_msg_step(step: Arc<RwLock<Steps>>) -> Option<GatheredMessage> {
        let read_guard = step.read().await;
        let message = GatheredMessage {
            operation_id: read_guard.operation_id.clone(),
            step_id: read_guard.step_id.clone(),
            respond: Some(GatheredResponse {
                result: read_guard.result.clone(),
                use_prev_res: read_guard.use_prev_res,
                extra_info: read_guard.extra_info.clone(),
            }),
        };
        drop(read_guard);
        Some(message)
    }
    pub fn reply_gathered_msg(mut message: GatheredMessage) -> Option<GatheredMessage> {
        let res = match DbOpsRegisterer::get_step_file(&message.operation_id, &message.step_id) {
            Some(stp) => {
                if stp.result.is_none() {
                    warn!("Step result is None, cannot reply gathered message");
                    return None;
                };
                GatheredResponse {
                    result: stp.result,
                    use_prev_res: stp.use_prev_res,
                    extra_info: stp.extra_info,
                }
            }
            None => {
                warn!("Step is not found, cannot reply gathered message");
                return None;
            }
        };
        message.respond = Some(res);
        Some(message)
    }
    // TODO you might move it outside of this struct, but I don't see it worth it.
    async fn ask_nodes_their_results(
        plan: Box<NodesOpsMsg>,
    ) -> Result<HashMap<String, bool>, ThothErrors> {
        let mut sent_messages = HashMap::new();
        let current_node_id = get_current_node_cloned().id;
        //TODO Keep track of execution steps, then get the number of nodes, if only this one available, then wait until all of the steps are done.
        let num_nodes = get_nodes_info_cloned().len();
        for (node_id, op_infos) in plan.nodes_duties {
            for info in op_infos {
                sent_messages.insert(info.step_id.clone(), true);
                if num_nodes == 1 && !is_internal_ops_finished(info.operation_id.clone()).await {
                    get_result_internally(info).await;
                    // If succesfully sent the message then continue to the next element, otherwise get back to the main loop and ask the other nodes.
                    continue;
                }
                if node_id == current_node_id {
                    info!(
                        "Operation is for the current node, getting the result internally: {:?}",
                        info
                    );
                    get_result_internally(info).await;
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
        Ok(sent_messages)
    }

    pub async fn gather_list_average(
        &mut self,
        plan: Box<NodesOpsMsg>,
    ) -> Result<f64, ThothErrors> {
        let mut duties_maps = Self::ask_nodes_their_results(plan).await?;
        let mut res: f64 = 0.0;
        let mut num_divide = 0.0;
        while duties_maps.len() > 0 {
            select! {
             result = self.reciever_ch.recv() => {
                 match result {
                     Some(value) => {
                        info!("Received: {:?}", value);
                        if duties_maps.get(&value.step_id).is_none(){
                            warn!("Received a step_id that is not in duties map: {}", value.step_id);
                            continue;
                        }
                        match value.respond{
                        Some(gath_res)=>{

                            if gath_res.result.is_some(){
                                let num:f64= gath_res.result.unwrap().clone().0.read().await.get_scaler_value();
                                res +=  num;
                                num_divide+=gath_res.extra_info.unwrap().helper_number.unwrap().0.read().await.get_scaler_value();
                            }
                            duties_maps.remove(&value.step_id);
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
        info!("Gathered List Average: {}, num_divide: {}", res, num_divide);
        res /= if num_divide != 0.0 { num_divide } else { 1.0 }; // Each step had it's own average, now after we gather each step average, we need to define the last average for all gathered steps.
        Ok(res)
    }

    pub async fn gather_matrix_multiply(
        &mut self,
        plan: Box<NodesOpsMsg>,
        (rows_dim, cols_dim): (usize, usize),
    ) -> Result<Matrix, ThothErrors> {
        let mut duties_maps = Self::ask_nodes_their_results(plan).await?;

        let mut res: Matrix = Matrix {
            rows: vec![
                MatrixRow {
                    values: vec![0.0; cols_dim]
                };
                rows_dim
            ],
        };
        // let mut left_to_gather = rows_dim * cols_dim;
        while duties_maps.len() > 0 {
            select! {
             result = self.reciever_ch.recv() => {
                 match result {
                     Some(value) => {
                        info!("Received: {:?}", value);
                        if duties_maps.get(&value.step_id).is_none(){
                            warn!("Received a step_id that is not in duties map: {}", value.step_id);
                            continue;
                        }
                        match value.respond{
                        Some(gath_res)=>{
                            let num = gath_res.result.unwrap().0.read().await.get_scaler_value();
                            if let Some(extra_infos)=gath_res.extra_info{
                                let poses= extra_infos.res_pos.unwrap_or(vec![0,0]); // It shall never be None in case of metrics.
                                let (x_pos,y_pos) = (poses[0],poses[1]);
                                res.rows[x_pos].values[y_pos]= num;

                            }
                            duties_maps.remove(&value.step_id);

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

    pub async fn gather_order_list(
        &mut self,
        plan: Box<NodesOpsMsg>,
    ) -> Result<Vec<f64>, ThothErrors> {
        let mut duties_maps = Self::ask_nodes_their_results(plan).await?;
        let mut res_list: Vec<Vec<f64>> = vec![];
        let mut total_list_len = 0;
        let mut order_type: Option<OperationsHelper> = None;
        while duties_maps.len() > 0 {
            select! {
             result = self.reciever_ch.recv() => {
                 match result {
                     Some(value) => {
                        info!("Received: {:?}", value);
                        if duties_maps.get(&value.step_id).is_none(){
                            warn!("Received a step_id that is not in duties map: {}", value.step_id);
                            continue;
                        }
                        match value.respond{
                        Some(gath_res)=>{
                            if order_type.is_none(){
                                order_type = Some(gath_res.extra_info.unwrap().helper_string.unwrap().into());
                            }
                            if gath_res.result.is_some(){
                                let num = gath_res.result.unwrap().clone().0.read().await.get_vector_value().clone();
                                //TODO push or extend based on the ordering type.
                                total_list_len+= num.len();
                                if res_list.is_empty(){
                                    res_list.push(num);
                                } else {
                                    let is_num_bigger= res_list[0][0]<num[0];
                                    match order_type.as_ref().unwrap() {
                                        OperationsHelper::ASCENDING => {
                                            if is_num_bigger{
                                                res_list.push(num);
                                            } else {
                                                res_list.insert(0,num);
                                            };
                                        },
                                        OperationsHelper::DESCENDING => {
                                            if is_num_bigger{
                                                res_list.insert(0,num);
                                            } else {
                                                res_list.push(num);
                                            };
                                        }
                                    };


                            };

                            duties_maps.remove(&value.step_id);
                            }
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

        // Sort res_list using insertion sort based on order_type
        let mut sorted: Vec<f64> = Vec::with_capacity(total_list_len);
        for num in res_list.into_iter() {
            let mut inserted = false;
            for i in 0..sorted.len() {
                // 'cmp' is short for 'compare'.
                let cmp = match order_type.as_ref().unwrap() {
                    OperationsHelper::ASCENDING => &num[0] < &sorted[i],
                    OperationsHelper::DESCENDING => &num[0] > &sorted[i],
                };
                if cmp {
                    sorted.splice(i..i, num.clone());
                    inserted = true;
                    break;
                }
            }
            if !inserted {
                sorted.extend(num);
            }
        }

        Ok(sorted)
    }
    pub async fn gather_list_max(&mut self, plan: Box<NodesOpsMsg>) -> Result<f64, ThothErrors> {
        let mut duties_maps = Self::ask_nodes_their_results(plan).await?;
        let mut res: f64 = std::f64::MIN;
        while duties_maps.len() > 0 {
            select! {
             result = self.reciever_ch.recv() => {
                 match result {
                     Some(value) => {
                        info!("Received: {:?}", value);
                        if duties_maps.get(&value.step_id).is_none(){
                            warn!("Received a step_id that is not in duties map: {}", value.step_id);
                            continue;
                        }
                        match value.respond{
                        Some(gath_res)=>{

                            if gath_res.result.is_some(){
                                let num:f64= gath_res.result.unwrap().clone().0.read().await.get_scaler_value();
                                if num > res {
                                    res = num;
                                }
                            }
                            duties_maps.remove(&value.step_id);
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
        info!("Gathered List Max: {}", res);
        Ok(res)
    }
    pub async fn gather_list_min(&mut self, plan: Box<NodesOpsMsg>) -> Result<f64, ThothErrors> {
        let mut duties_maps = Self::ask_nodes_their_results(plan).await?;
        let mut res: f64 = std::f64::MAX;
        while duties_maps.len() > 0 {
            select! {
             result = self.reciever_ch.recv() => {
                 match result {
                     Some(value) => {
                        info!("Received: {:?}", value);
                        if duties_maps.get(&value.step_id).is_none(){
                            warn!("Received a step_id that is not in duties map: {}", value.step_id);
                            continue;
                        }
                        match value.respond{
                        Some(gath_res)=>{

                            if gath_res.result.is_some(){
                                let num:f64= gath_res.result.unwrap().clone().0.read().await.get_scaler_value();
                                if num < res {
                                    res = num;
                                }
                            }
                            duties_maps.remove(&value.step_id);
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
        info!("Gathered List Max: {}", res);
        Ok(res)
    }
}

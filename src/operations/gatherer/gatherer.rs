use crate::{
    errors::thot_errors::ThothErrors,
    grpc::grpc_server::mathop::{Matrix, MatrixRow},
    info,
    logger::writters::writter::OperationsFileManager,
    operations::{
        gatherer::{
            channels::add_ch_sender,
            structs::{GatheredMessage, GatheredResponse, Gatherer},
        },
        planner::charts::structs::NodesOpsMsg,
    },
    router::{post_offices::nodes_info::post_office::GathererOffice, traits::PostOfficeTrait},
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
        for (_, op_infos) in plan.nodes_duties {
            for info in op_infos.try_read()?.clone() {
                spawn(async {
                    GathererOffice::send_message(GatheredMessage {
                        operation_id: info.operation_id,
                        step_id: info.step_id,
                        respond: None,
                    });
                });
                num_sent_message += 1
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

        while left_to_gather > 0 {
            select! {
             result = self.reciever_ch.recv() => {
                 match result {
                     Some(value) => {
                        info!("Received: {:?}", value);
                        match value.respond{
                        Some(gath_res)=>{
                            let num = gath_res.result.get_scaler_value();
                            res+=num;
                            // if let Some(extra_infos)=gath_res.extra_info{


                            // }
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
        res /= num_duties as f64; // Each step had it's own average, now after we gather each step average, we need to define the last average for all gathered steps.
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

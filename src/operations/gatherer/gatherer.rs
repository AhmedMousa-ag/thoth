use crate::{
    grpc::grpc_server::mathop::{Matrix, MatrixRow},
    info,
    operations::{
        gatherer::{channels::get_reciver_rx, structs::Gatherer},
        planner::charts::structs::NodesOpsMsg,
    },
};
use std::fmt::Error;
use tokio::select;

impl Gatherer {
    pub async fn gather_matrix_multiply(plan: Box<NodesOpsMsg>) -> Result<Matrix, Error> {
        let res: Vec<Vec<f64>> = Vec::new();
        let mut receiver = get_reciver_rx().try_lock().unwrap();

        //Ask
        loop {
            select! {
             result = receiver.recv() => {
                 match result {
                     Some(value) => {
                         // Handle received value
                         info!("Received: {:?}", value);
                     }
                     None => {
                         // Channel closed
                         break;
                     }
                 }
             }
            }
        }
        Ok(Matrix {
            rows: vec![MatrixRow { values: vec![] }],
        })
    }
}

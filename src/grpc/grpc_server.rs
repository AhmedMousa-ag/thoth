use mathop::{
    ListAverageOperationReply, ListAverageOperationRequest, MatrixOperationReply,
    MatrixOperationRequest,
    math_ops_server::{MathOps, MathOpsServer},
};
use tonic::{Request, Response, Status, transport::Server};

use crate::{
    err,
    errors::thot_errors::ThothErrors,
    grpc::{grpc_server::mathop::Matrix, utils::extract_matrix},
    info,
    operations::{
        gatherer::structs::Gatherer,
        planner::{charts::structs::NodesOpsMsg, organizer::Planner},
    },
};
pub mod mathop {
    tonic::include_proto!("mathop");
}

#[derive(Debug, Default)]
pub struct MathOperations {}

#[tonic::async_trait]
impl MathOps for MathOperations {
    async fn matrix_multiply(
        &self,
        request: Request<MatrixOperationRequest>,
    ) -> Result<Response<MatrixOperationReply>, Status> {
        info!(
            "gRPC: got matrix multiplication request from: {:?}",
            request.remote_addr()
        );
        let req_data: MatrixOperationRequest = request.into_inner();
        let operation_id: String = req_data.operation_id;
        let pln = Planner::new(operation_id.clone());

        let x_matrix: mathop::Matrix = match req_data.matrix_a {
            Some(s) => s,
            None => {
                return Ok(Response::new(MatrixOperationReply {
                    result_matrix: None,
                    status_message: format!(
                        "Matrix X wasn't provided, please provide two matrixes"
                    ),
                }));
            }
        };

        let (x, x_rows_dim, _x_cols_dim) = extract_matrix(x_matrix);
        let y_matrix: Matrix = match req_data.matrix_b {
            Some(s) => s,
            None => {
                return Ok(Response::new(MatrixOperationReply {
                    result_matrix: None,
                    status_message: format!(
                        "Matrix X wasn't provided, please provide two matrixes"
                    ),
                }));
            }
        };

        let (y, _y_rows_dim, y_cols_dim) = extract_matrix(y_matrix);
        let nodes_duties: Box<NodesOpsMsg> = match pln.plan_matrix_naive_multiply(x, y).await {
            Ok(duties) => duties,
            Err(e) => {
                let err_msg = format!("Failed to create plans due to: {}", e);
                err!(err_msg);
                return Ok(Response::new(MatrixOperationReply {
                    result_matrix: None,
                    status_message: err_msg,
                }));
            }
        };
        let mut gatherer = Gatherer::new(operation_id).await;
        let num_res = match gatherer
            .gather_matrix_multiply(nodes_duties, (x_rows_dim, y_cols_dim))
            .await
        {
            Ok(rs) => rs,
            Err(e) => {
                let err_msg = format!("Failed to gather results due to: {}", e);
                err!(err_msg);
                return Ok(Response::new(MatrixOperationReply {
                    result_matrix: None,
                    status_message: err_msg,
                }));
            }
        };
        let reply = MatrixOperationReply {
            result_matrix: Some(num_res),
            status_message: format!("Succesfully got your result."),
        };
        Ok(Response::new(reply))
    }

    async fn list_average(
        &self,
        request: Request<ListAverageOperationRequest>,
    ) -> Result<Response<ListAverageOperationReply>, Status> {
        info!(
            "gRPC: got list average  request from: {:?}",
            request.remote_addr()
        );
        let req_data: ListAverageOperationRequest = request.into_inner();
        let operation_id = req_data.operation_id;
        let pln = Planner::new(operation_id.clone());

        let nodes_duties = match pln.plan_average(req_data.x).await {
            Ok(duties) => duties,
            Err(e) => {
                let err_msg = format!("Failed to create plans due to: {}", e);
                err!(err_msg);
                return Ok(Response::new(ListAverageOperationReply {
                    result_average: None,
                    status_message: err_msg,
                }));
            }
        };
        let mut gatherer = Gatherer::new(operation_id).await;
        let num_res = match gatherer.gather_list_average(nodes_duties).await {
            Ok(rs) => rs,
            Err(e) => {
                let err_msg = format!("Failed to gather results due to: {}", e);
                err!(err_msg);
                return Ok(Response::new(ListAverageOperationReply {
                    result_average: None,
                    status_message: err_msg,
                }));
            }
        };

        let reply = ListAverageOperationReply {
            result_average: Some(num_res),
            status_message: format!("Succesfully got your result."),
        };
        Ok(Response::new(reply))
    }
}

pub async fn start_server() -> Result<(), ThothErrors> {
    info!("Start of gRPC server");
    let addr = "[::]:50051".parse().unwrap();
    let matrix_ops: MathOperations = MathOperations::default();
    let limit = 10000 * 1024 * 1024; // 10 GB
    let mathops_server: MathOpsServer<MathOperations> = MathOpsServer::new(matrix_ops)
        .max_decoding_message_size(limit)
        .max_encoding_message_size(limit);
    info!("Will start gRPC server now on address: {:?}", addr);
    Server::builder()
        .add_service(mathops_server)
        .serve(addr)
        .await?;

    info!("Established gRPC server.");
    Ok(())
}

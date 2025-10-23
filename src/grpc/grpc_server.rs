use mathop::{
    ListAverageOperationReply,
    ListAverageOperationRequest,
    ListMaxReply,
    ListMaxRequest,
    ListMinReply,
    ListMinRequest,
    MatrixOperationReply,
    // ListModeReply,ListModeRequest,
    MatrixOperationRequest,
    OrderListReply,
    OrderListRequest,
    math_ops_server::{MathOps, MathOpsServer},
};
use tonic::{Request, Response, Status, transport::Server};

use crate::{
    err,
    errors::thot_errors::ThothErrors,
    grpc::{
        grpc_server::mathop::{EmptyReply, Matrix},
        utils::extract_matrix,
    },
    info,
    operations::{
        cache::objects::{CachedObj, get_cached_object, insert_cache_object, remove_cached_object},
        gatherer::structs::Gatherer,
        planner::{charts::structs::NodesOpsMsg, organizer::Planner},
    },
    structs::numerics::structs::SharedNumeric,
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
        let shared_numeric = get_cached_object(&operation_id).await.unwrap().data;
        let nodes_duties = match pln.plan_average(shared_numeric).await {
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

    async fn order_list(
        &self,
        request: Request<OrderListRequest>,
    ) -> Result<Response<OrderListReply>, Status> {
        info!(
            "gRPC: got list median request from: {:?}",
            request.remote_addr()
        );
        let req_data = request.into_inner();
        let operation_id = req_data.operation_id;
        let pln = Planner::new(operation_id.clone());
        let shared_numeric = get_cached_object(&operation_id).await.unwrap().data;
        let nodes_duties = match pln
            .plan_order_list(shared_numeric, req_data.ascending)
            .await
        {
            Ok(duties) => duties,
            Err(e) => {
                let err_msg = format!("Failed to create plans due to: {}", e);
                err!(err_msg);
                return Ok(Response::new(OrderListReply {
                    result: vec![],
                    status_message: err_msg,
                }));
            }
        };
        let mut gatherer = Gatherer::new(operation_id).await;
        let num_res = match gatherer.gather_order_list(nodes_duties).await {
            Ok(rs) => rs,
            Err(e) => {
                let err_msg = format!("Failed to gather results due to: {}", e);
                err!(err_msg);
                return Ok(Response::new(OrderListReply {
                    result: vec![],
                    status_message: err_msg,
                }));
            }
        };

        let reply = OrderListReply {
            result: num_res,
            status_message: format!("Successfully got your result."),
        };
        Ok(Response::new(reply))
    }

    async fn list_max(
        &self,
        request: Request<ListMaxRequest>,
    ) -> Result<Response<ListMaxReply>, Status> {
        info!(
            "gRPC: got list max request from: {:?}",
            request.remote_addr()
        );
        let req_data = request.into_inner();
        let operation_id = req_data.operation_id;
        let pln = Planner::new(operation_id.clone());
        let shared_numeric = get_cached_object(&operation_id).await.unwrap().data;
        let nodes_duties = match pln.plan_max_list(shared_numeric).await {
            Ok(duties) => duties,
            Err(e) => {
                let err_msg = format!("Failed to create plans due to: {}", e);
                err!(err_msg);
                return Ok(Response::new(ListMaxReply {
                    result: None,
                    status_message: err_msg,
                }));
            }
        };
        let mut gatherer = Gatherer::new(operation_id).await;
        let num_res = match gatherer.gather_list_max(nodes_duties).await {
            Ok(rs) => rs,
            Err(e) => {
                let err_msg = format!("Failed to gather results due to: {}", e);
                err!(err_msg);
                return Ok(Response::new(ListMaxReply {
                    result: None,
                    status_message: err_msg,
                }));
            }
        };

        let reply = ListMaxReply {
            result: Some(num_res),
            status_message: format!("Successfully got your result."),
        };
        Ok(Response::new(reply))
    }

    async fn list_min(
        &self,
        request: Request<ListMinRequest>,
    ) -> Result<Response<ListMinReply>, Status> {
        info!(
            "gRPC: got list min request from: {:?}",
            request.remote_addr()
        );
        let req_data = request.into_inner();
        let operation_id = req_data.operation_id;
        let pln = Planner::new(operation_id.clone());
        let shared_numeric = get_cached_object(&operation_id).await.unwrap().data;
        let nodes_duties = match pln.plan_min_list(shared_numeric).await {
            Ok(duties) => duties,
            Err(e) => {
                let err_msg = format!("Failed to create plans due to: {}", e);
                err!(err_msg);
                return Ok(Response::new(ListMinReply {
                    result: None,
                    status_message: err_msg,
                }));
            }
        };
        let mut gatherer = Gatherer::new(operation_id).await;
        let num_res = match gatherer.gather_list_min(nodes_duties).await {
            Ok(rs) => rs,
            Err(e) => {
                let err_msg = format!("Failed to gather results due to: {}", e);
                err!(err_msg);
                return Ok(Response::new(ListMinReply {
                    result: None,
                    status_message: err_msg,
                }));
            }
        };

        let reply = ListMinReply {
            result: Some(num_res),
            status_message: format!("Successfully got your result."),
        };
        Ok(Response::new(reply))
    }

    async fn create_object(
        &self,
        request: Request<mathop::CreateObjectRequest>,
    ) -> Result<Response<EmptyReply>, Status> {
        info!(
            "gRPC: got create object request from: {:?}",
            request.remote_addr()
        );
        let req_data = request.into_inner();
        let id = req_data.operation_id;

        insert_cache_object(CachedObj {
            id: id,
            data: SharedNumeric::new(crate::structs::numerics::structs::Numeric::Scaler(0.0)),
        })
        .await;

        Ok(Response::new(EmptyReply {}))
    }

    async fn add_data_object(
        &self,
        request: Request<mathop::AddDataObjectRequest>,
    ) -> Result<Response<EmptyReply>, Status> {
        info!(
            "gRPC: got add data object request from: {:?}",
            request.remote_addr()
        );
        let req_data = request.into_inner();
        let id = req_data.operation_id;
        let value = req_data.data;

        insert_cache_object(CachedObj {
            id: id,
            data: SharedNumeric::new(crate::structs::numerics::structs::Numeric::Vector(value)),
        })
        .await;

        Ok(Response::new(EmptyReply {}))
    }
    async fn clear_object(
        &self,
        request: Request<mathop::ClearObjectRequest>,
    ) -> Result<Response<EmptyReply>, Status> {
        info!(
            "gRPC: got clear object request from: {:?}",
            request.remote_addr()
        );
        let req_data = request.into_inner();
        let id = req_data.operation_id;

        remove_cached_object(&id).await;
        Ok(Response::new(EmptyReply {}))
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

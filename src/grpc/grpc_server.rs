use tonic::{ Request, Response, Status,transport::Server};
use mathop::{MatrixOperationRequest,MatrixOperationReply,math_ops_server::{MathOps,MathOpsServer}};

use crate::{err, info};
pub mod mathop {
    tonic::include_proto!("mathop"); 
}

#[derive(Debug,Default)]
pub struct MatrixOperations {}

#[tonic::async_trait]
impl MathOps for MatrixOperations {
    async fn matrix_multiply(&self,
        request: Request<MatrixOperationRequest>,
    ) -> Result<Response<MatrixOperationReply>, Status> {
        info!("Just got a request from: {:?}",request.remote_addr().unwrap());
        let reply = MatrixOperationReply {
            result_matrix:None,
            status_message: format!("Hello {}", "Hold Temporarily"),
        };
        Ok(Response::new(reply))
    }
}

pub async fn start_server()  {
        info!("Start of gRPC server");
        let addr = "[::1]:50051".parse().unwrap();
        let matrix_ops: MatrixOperations = MatrixOperations::default();
        let mathops_server: MathOpsServer<MatrixOperations>=MathOpsServer::new(matrix_ops);
        info!("Will start gRPC server now");
        match Server::builder()
            .add_service(mathops_server)
            .serve(addr)
            .await{
                Ok(_)=>info!("Established gRPC server."),
                Err(e)=>err!("Failed to establish gRPC server due to: {}",e)
            }
}

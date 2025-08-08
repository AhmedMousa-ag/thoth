
use crate::{grpc::grpc_server::{mathop::{math_ops_server::MathOps, ListAverageOperationRequest}, MathOperations}};


#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_plan_average() {

    let request = tonic::Request::new(ListAverageOperationRequest{
        operation_id: "test_operation".to_string(),
        x: vec![1.0, 2.0, 3.0, 4.0, 5.0],
    });
    let result = MathOperations::default().list_average(request).await.unwrap();
    
    let expected_average = 3.0; // (1.0 + 2.0 + 3.0 + 4.0 + 5.0) / 5.0 = 3.0
    assert_eq!(result.into_inner().result_average.unwrap(), expected_average);

    
}

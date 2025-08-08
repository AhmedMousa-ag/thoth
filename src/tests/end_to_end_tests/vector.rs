use crate::grpc::grpc_server::{
    MathOperations,
    mathop::{ListAverageOperationRequest, math_ops_server::MathOps},
};

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_plan_average() {
    let request = tonic::Request::new(ListAverageOperationRequest {
        operation_id: format!("test_list_average_{}", uuid::Uuid::new_v4()),
        x: vec![1.0, 2.0, 3.0, 4.0, 5.0],
    });
    let result = MathOperations::default()
        .list_average(request)
        .await
        .unwrap();
    let expected_average = 3.0; // (1.0 + 2.0 + 3.0 + 4.0 + 5.0) / 5.0 = 3.0
    assert_eq!(
        result.into_inner().result_average.unwrap(),
        expected_average
    );
}

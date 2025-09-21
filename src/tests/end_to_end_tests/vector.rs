use crate::grpc::grpc_server::{
    MathOperations,
    mathop::{ListAverageOperationRequest, OrderListRequest, math_ops_server::MathOps},
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

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_order_list() {
    let list_to_order = vec![3.0, 1.0, 4.0, 5.0, 2.0, 4.8, 3.3, 2.2, 1.1, 20.0];
    let request = tonic::Request::new(OrderListRequest {
        operation_id: format!("test_order_list_acending_{}", uuid::Uuid::new_v4()),
        x: list_to_order.clone(),
        ascending: true,
    });

    let result = MathOperations::default().order_list(request).await.unwrap();
    let expected_ordered_list = vec![1.0, 1.1, 2.0, 2.2, 3.0, 3.3, 4.0, 4.8, 5.0, 20.0];
    assert_eq!(result.into_inner().result, expected_ordered_list);
    // descending testing.
    let request = tonic::Request::new(OrderListRequest {
        operation_id: format!("test_order_list_decending_{}", uuid::Uuid::new_v4()),
        x: list_to_order,
        ascending: false,
    });
    let result = MathOperations::default().order_list(request).await.unwrap();
    let expected_ordered_list_desc = vec![20.0, 5.0, 4.8, 4.0, 3.3, 3.0, 2.2, 2.0, 1.1, 1.0];
    assert_eq!(result.into_inner().result, expected_ordered_list_desc);
}
//TODO test with same UUID and check caching results.

//TODO test with several nodes in the cluster instead of just one.

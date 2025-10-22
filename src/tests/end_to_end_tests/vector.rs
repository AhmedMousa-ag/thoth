use crate::grpc::grpc_server::{
    MathOperations,
    mathop::{
        AddDataObjectRequest, ListAverageOperationRequest, ListMaxRequest, ListMinRequest,
        OrderListRequest, math_ops_server::MathOps,
    },
};

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_plan_average() {
    let operation_id = format!("test_plan_average_{}", uuid::Uuid::new_v4());
    let obj_req = tonic::Request::new(AddDataObjectRequest {
        operation_id: operation_id.clone(),
        data: vec![1.0, 2.0, 3.0, 4.0, 5.0],
    });
    let _ = MathOperations::default()
        .add_data_object(obj_req)
        .await
        .unwrap();

    let request = tonic::Request::new(ListAverageOperationRequest { operation_id });

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
    let operation_id = format!("test_order_list_acending_{}", uuid::Uuid::new_v4());
    let list_to_order = vec![3.0, 1.0, 4.0, 5.0, 2.0, 4.8, 3.3, 2.2, 1.1, 20.0];
    let obj_req = tonic::Request::new(AddDataObjectRequest {
        operation_id: operation_id.clone(),
        data: list_to_order.clone(),
    });
    let _ = MathOperations::default()
        .add_data_object(obj_req)
        .await
        .unwrap();

    let request = tonic::Request::new(OrderListRequest {
        operation_id: operation_id,
        ascending: true,
    });

    let result = MathOperations::default().order_list(request).await.unwrap();
    let expected_ordered_list = vec![1.0, 1.1, 2.0, 2.2, 3.0, 3.3, 4.0, 4.8, 5.0, 20.0];
    assert_eq!(result.into_inner().result, expected_ordered_list);

    let operation_id = format!("test_order_list_descending_{}", uuid::Uuid::new_v4());
    let obj_req = tonic::Request::new(AddDataObjectRequest {
        operation_id: operation_id.clone(),
        data: list_to_order.clone(),
    });
    let _ = MathOperations::default()
        .add_data_object(obj_req)
        .await
        .unwrap();
    // descending testing.
    let request = tonic::Request::new(OrderListRequest {
        operation_id,
        ascending: false,
    });
    let result = MathOperations::default().order_list(request).await.unwrap();
    let expected_ordered_list_desc = vec![20.0, 5.0, 4.8, 4.0, 3.3, 3.0, 2.2, 2.0, 1.1, 1.0];
    assert_eq!(result.into_inner().result, expected_ordered_list_desc);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_max_list() {
    let list_to_max = vec![3.0, 1.0, 4.0, 5.0, 2.0, 4.8, 3.3, 2.2, 1.1, 20.0];
    let operation_id = format!("test_max_list_{}", uuid::Uuid::new_v4());
    let obj_req = tonic::Request::new(AddDataObjectRequest {
        operation_id: operation_id.clone(),
        data: list_to_max,
    });
    let _ = MathOperations::default()
        .add_data_object(obj_req)
        .await
        .unwrap();

    let request = tonic::Request::new(ListMaxRequest { operation_id });
    let result = MathOperations::default().list_max(request).await.unwrap();
    let expected_max = 20.0;
    assert_eq!(result.into_inner().result.unwrap(), expected_max);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_min_list() {
    let list_to_min = vec![3.0, 1.0, 4.0, 5.0, 2.0, 4.8, 3.3, 2.2, 1.1, 20.0];
    let operation_id = format!("test_min_list_{}", uuid::Uuid::new_v4());
    let obj_req = tonic::Request::new(AddDataObjectRequest {
        operation_id: operation_id.clone(),
        data: list_to_min,
    });
    let _ = MathOperations::default()
        .add_data_object(obj_req)
        .await
        .unwrap();

    let request = tonic::Request::new(ListMinRequest { operation_id });
    let result = MathOperations::default().list_min(request).await.unwrap();
    let expected_min = 1.0;
    assert_eq!(result.into_inner().result.unwrap(), expected_min);
}
//TODO test with same UUID and check caching results.

//TODO test with several nodes in the cluster instead of just one.

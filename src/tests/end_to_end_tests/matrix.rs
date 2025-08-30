use crate::grpc::grpc_server::{
    MathOperations,
    mathop::{Matrix, MatrixOperationRequest, MatrixRow, math_ops_server::MathOps},
};

#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_matrix_multiply() {
    let matrix_a = Matrix {
        rows: vec![
            MatrixRow {
                values: vec![1.0, 2.0, 3.0],
            },
            MatrixRow {
                values: vec![4.0, 5.0, 6.0],
            },
        ],
    };
    let matrix_b = Matrix {
        rows: vec![
            MatrixRow {
                values: vec![7.0, 8.0],
            },
            MatrixRow {
                values: vec![9.0, 10.0],
            },
            MatrixRow {
                values: vec![11.0, 12.0],
            },
        ],
    };
    let request = tonic::Request::new(MatrixOperationRequest {
        operation_id: format!("test_matrix_multiply_{}", uuid::Uuid::new_v4()),
        matrix_a: Some(matrix_a),
        matrix_b: Some(matrix_b),
    });
    let result = MathOperations::default()
        .matrix_multiply(request)
        .await
        .unwrap();
    let expected_result = Matrix {
        rows: vec![
            MatrixRow {
                values: vec![58.0, 64.0],
            },
            MatrixRow {
                values: vec![139.0, 154.0],
            },
        ],
    };
    let res = result.into_inner();
    assert_eq!(res.result_matrix.unwrap(), expected_result);
}

//TODO test with same UUID and check caching results.

//TODO test with several nodes in the cluster instead of just one.

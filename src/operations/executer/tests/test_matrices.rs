use crate::operations::executer::matrices::multiply;
#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_multipy() {
    let x = vec![vec![1.0, 2.0], vec![4.0, 5.0], vec![7.0, 8.0]];
    let y = vec![vec![1.0, 2.0], vec![4.0, 5.0]];
    let res = multiply(x, y);
    assert_eq!(
        res,
        vec![vec![9.0, 12.0], vec![24.0, 33.0], vec![39.0, 54.0]]
    );
}

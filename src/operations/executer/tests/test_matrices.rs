use crate::operations::executer::matrices;
#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_multipy() {
    let x = vec![vec![1.0, 2.0], vec![4.0, 5.0], vec![7.0, 8.0]];
    let y = vec![vec![1.0, 2.0], vec![4.0, 5.0]];
    let expected_result = vec![vec![9.0, 12.0], vec![24.0, 33.0], vec![39.0, 54.0]];
    let naive_res = matrices::naive_multiply(x, y);
    // let strassen_res = matrices::strassen_multiply(x, y);
    assert_eq!(naive_res, expected_result);
    // assert_eq!(strassen_res,expected_result)
}


#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_add() {
    let x = vec![vec![1.0, 2.0], vec![4.0, 5.0]];
    let y = vec![vec![1.0, 2.0], vec![4.0, 5.0]];
    let expected_result = vec![vec![2.0, 4.0], vec![8.0, 10.0]];
    let add_res = matrices::add(x, y);
    assert_eq!(add_res, expected_result);
}




#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_sub() {
    let x = vec![vec![1.0, 2.0], vec![4.0, 5.0]];
    let y = vec![vec![7.0, 2.0], vec![6.0, 5.0]];
    let expected_result =  vec![vec![-6.0, 0.0], vec![-2.0, 0.0]];
    let sub_res = matrices::sub(x, y);
    assert_eq!(sub_res, expected_result);
}


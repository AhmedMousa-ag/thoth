use crate::grpc::grpc_server::mathop::Matrix;

pub fn extract_matrix(x: Matrix) -> (Vec<Vec<Box<f64>>>, usize, usize) {
    let (mut rows_dim, mut cols_dim): (usize, usize) = (0, 0);
    let res: Vec<Vec<Box<f64>>> = x
        .rows
        .iter()
        .map(|row| {
            rows_dim += 1;
            if rows_dim == 1 {
                cols_dim = row.values.len();
            }
            row.values.iter().map(|val| Box::new(*val)).collect()
        })
        .collect();
    (res, rows_dim, cols_dim)
}

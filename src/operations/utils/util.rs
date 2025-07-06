use crate::debug;

pub fn transpose<T>(x: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if x.is_empty() || x[0].is_empty() {
        return Vec::new();
    }

    let rows = x.len();
    let cols = x[0].len();

    let mut result: Vec<Vec<T>> = (0..cols).map(|_| Vec::with_capacity(rows)).collect();

    for row in x {
        for (col_idx, value) in row.into_iter().enumerate() {
            result[col_idx].push(value);
        }
    }

    result
}

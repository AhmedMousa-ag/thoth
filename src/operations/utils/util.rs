
pub fn transpose<T>(x: Vec<Vec<Box<T>>>) -> Vec<Vec<Box<T>>> {
    if x.is_empty() || x[0].is_empty() {
        return Vec::new();
    }

    let rows = x.len();
    let cols = x[0].len();

    let mut result: Vec<Vec<Box<T>>> = (0..cols).map(|_| Vec::with_capacity(rows)).collect();

    for row in x {
        for (col_idx, value) in row.into_iter().enumerate() {
            result[col_idx].push(value);
        }
    }

    result
}

pub fn get_node_id(idx: &mut usize, nodes_num: usize, nodes_keys: &Vec<String>) -> String {
    if idx.clone() >= nodes_num {
        *idx = 0;
    }
    let key = &nodes_keys[*idx];
    *idx += 1;
    key.to_string()
}

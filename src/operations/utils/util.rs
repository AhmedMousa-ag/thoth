// use crate::{
//     // operations::{gatherer::structs::GatheredResponse, planner::charts::structs::ExtraInfo},
//     // structs::numerics::structs::Numeric,
// };

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

pub fn get_node_id(idx: &mut usize, nodes_num: usize, nodes_keys: &Vec<String>) -> String {
    if idx.clone() >= nodes_num {
        *idx = 0;
    }
    let key = &nodes_keys[*idx];
    *idx += 1;
    key.to_string()
}

// pub fn load_sql_step_to_gatherer_res(sql_step: &steps::Model) -> GatheredResponse {
//     let extra_info = ExtraInfo {
//         res_pos: serde_json::from_str(&sql_step.res_pos.clone().unwrap_or_default()).ok(),
//         res_type: serde_json::from_str(&sql_step.res_type.clone().unwrap_or_default()).ok(),
//     };
//     GatheredResponse {
//         result: Numeric::from_string(sql_step.result.clone()).unwrap_or(Numeric::Scaler(0.0)),
//         use_prev_res: sql_step.use_prev_res,
//         extra_info: Some(extra_info),
//     }
// }

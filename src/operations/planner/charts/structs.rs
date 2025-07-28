use crate::{operations::executer::types::OperationTypes, warn};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum Numeric {
    Scaler(f64),
    Vector(Vec<f64>),
    Matrix(Vec<Vec<f64>>),
}
impl Numeric {
    pub fn to_string(&self) -> String {
        match self {
            //TODO, fix it.
            Numeric::Scaler(_) => format!("{:?}", self),
            Numeric::Vector(_) => format!("{:?}", self),
            Numeric::Matrix(_) => format!("{:?}", self),
        }
    }
    ///Don't use if your type isn't scaler.
    pub fn get_scaler_value(&self) -> f64 {
        match self {
            Numeric::Scaler(val) => *val,
            _ => {
                let msg = "Expected Scaler variant, will return a zero";
                warn!("{}", msg);
                0.0
            }
        }
    }

    ///Don't use if your type isn't vector.
    pub fn get_vector_value(&self) -> Vec<f64> {
        match self {
            Numeric::Vector(val) => val.clone(),
            _ => {
                let msg = "Expected Vector variant, will return a zero";
                warn!("{}", msg);
                vec![0.0]
            }
        }
    }

    ///Don't use if your type isn't a Matrix.
    pub fn get_matrices_value(&self) -> Vec<Vec<f64>> {
        match self {
            Numeric::Matrix(val) => val.clone(),
            _ => {
                let msg = "Expected Matrix variant, will return a zero";
                warn!("{}", msg);
                vec![vec![0.0]]
            }
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct ExtraInfo {
    pub res_pos: Option<Vec<usize>>, //If it's a matrix or a list, two points should be maximum
    pub res_type: Option<Numeric>, // To define the final output shape, shall it be matrix or vector or a scaler ?
}

//TODO probably you would like to create functions instead of all of this mess.
#[derive(Debug, Encode, Decode, Serialize, Deserialize)]
pub struct Steps {
    pub node_id: String,
    pub operation_id: String,
    pub step_id: String,
    pub x: Option<Numeric>,
    pub y: Option<Numeric>,
    pub op_type: OperationTypes,
    pub result: Option<Numeric>,
    pub next_step: Option<String>,
    pub prev_step: Option<String>,
    pub use_prev_res: bool, //If true, then this will be used instead of x.
    pub extra_info: Option<ExtraInfo>,
}
#[derive(Debug, Encode, Decode, Clone)]
pub struct OperationInfo {
    pub operation_id: String,
    pub step_id: String,
}

type NodeOpsMsgType = Vec<OperationInfo>;
//This one can be send between threads in async functions.
// #[derive(Debug, Encode, Decode)]
// pub struct RNodesOpsMsg {
//     pub nodes_duties: HashMap<String, Box<NodeOpsMsgType>>,
// }

pub type NodesDuties = HashMap<String, Arc<RwLock<NodeOpsMsgType>>>;
///This one can't be sent between threads in async function
#[derive(Debug, Encode, Decode, Clone)]
pub struct NodesOpsMsg {
    pub nodes_duties: NodesDuties,
}

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::operations::executer::base_operations::OperationTypes;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum Numeric {
    Number(f64),
    Vector(Vec<f64>),
    Matrix(Vec<Vec<f64>>),
}

//TODO probably you would like to create functions instead of all of this mess.
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Steps {
    pub x: Option<Numeric>,
    pub y: Option<Numeric>,
    pub op_type: OperationTypes,
    pub result: Option<Numeric>,
    pub next_step: Option<Box<Self>>,
    pub prev_step: Option<Box<Self>>,
    pub use_prev_res: bool, //If true, then this will be used instead of x.
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct NodesOpsMsg {
    pub nodes_duties: HashMap<String, Box<Steps>>,
}

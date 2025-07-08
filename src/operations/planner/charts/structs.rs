use crate::operations::executer::base_operations::OperationTypes;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum Numeric {
    Number(Box<f64>),
    Vector(Vec<Box<f64>>),
    Matrix(Vec<Vec<Box<f64>>>),
}
impl Numeric {
    pub fn to_string(&self) -> String {
        match self {
            //TODO, fix it.
            Numeric::Number(_) => format!("{:?}", self),
            Numeric::Vector(_) => format!("{:?}", self),
            Numeric::Matrix(_) => format!("{:?}", self),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct ExtraInfo {
    pub res_pos: Option<Vec<u64>>, //If it's a matrix or a list, two points should be maximum
}

//TODO probably you would like to create functions instead of all of this mess.
#[derive(Debug, Encode, Decode)]
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

#[derive(Debug, Encode, Decode)]
pub struct NodesOpsMsg {
    pub nodes_duties: HashMap<String, Rc<RefCell<Steps>>>,
}

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::operations::executer::base_operations::OperationTypes;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum Numeric {
    Number(Box<f64>),
    Vector(Vec<Box<f64>>),
    Matrix(Vec<Vec<Box<f64>>>),
}
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct ExtraInfo {
    pub res_pos: Option<Vec<u64>>, //If it's a matrix or a list, two points should be maximum
}
//TODO probably you would like to create functions instead of all of this mess.
#[derive(Debug, Encode, Decode)]
pub struct Steps {
    pub x: Option<Numeric>,
    pub y: Option<Numeric>,
    pub op_type: OperationTypes,
    pub result: Option<Numeric>,
    pub next_step: Option<Rc<RefCell<Self>>>,
    pub prev_step: Option<Rc<RefCell<Self>>>,
    pub use_prev_res: bool, //If true, then this will be used instead of x.
    pub extra_info: Option<ExtraInfo>,
}

#[derive(Debug, Encode, Decode)]
pub struct NodesOpsMsg {
    pub nodes_duties: HashMap<String, Rc<RefCell<Steps>>>,
}

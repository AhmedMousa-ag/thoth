use crate::{operations::executer::types::OperationTypes, warn};
use bincode::{Decode, Encode};
use sea_orm::sea_query::value;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, fmt::Debug, ops::Deref, rc::Rc};

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum Numeric {
    Scaler(Box<f64>),
    Vector(Vec<Box<f64>>),
    Matrix(Vec<Vec<Box<f64>>>),
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
    pub fn get_scaler_value(&self) -> Box<f64> {
        match self {
            Numeric::Scaler(val) => val.clone(),
            _ => {
                let msg = "Expected Scaler variant, will return a zero";
                warn!("{}", msg);
                Box::from(0.0)
            }
        }
    }

    ///Don't use if your type isn't vector.
    pub fn get_vector_value(&self) -> Vec<Box<f64>> {
        match self {
            Numeric::Vector(val) => val.clone(),
            _ => {
                let msg = "Expected Vector variant, will return a zero";
                warn!("{}", msg);
                vec![Box::new(0.0)].clone()
            }
        }
    }

    ///Don't use if your type isn't a Matrix.
    pub fn get_matrices_value(&self) -> Vec<Vec<Box<f64>>> {
        match self {
            Numeric::Matrix(val) => val.clone(),
            _ => {
                let msg = "Expected Matrix variant, will return a zero";
                warn!("{}", msg);
                vec![vec![Box::new(0.0)]].clone()
            }
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct ExtraInfo {
    pub res_pos: Option<Vec<u64>>, //If it's a matrix or a list, two points should be maximum
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
#[derive(Debug, Encode, Decode)]
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

pub type NodesDuties = HashMap<String, Rc<RefCell<NodeOpsMsgType>>>;
///This one can't be sent between threads in async function
#[derive(Debug, Encode, Decode)]
pub struct NodesOpsMsg {
    pub nodes_duties: NodesDuties,
}

use crate::{operations::executer::types::OperationTypes, structs::numerics::structs::Numeric};
use bincode::{Decode, Encode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationFile {
    pub operation_id: String,
    pub result: Option<Numeric>,
    pub execution_date: DateTime<Utc>,
}

#[derive(Debug, Encode, Decode, Clone, Serialize, Deserialize)]
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

// pub type NodesDuties = HashMap<String, Arc<RwLock<NodeOpsMsgType>>>;

/// Serializable version of NodesDuties for encoding/decoding and serialization/deserialization.
pub type NodesDuties = HashMap<String, NodeOpsMsgType>;

#[derive(Debug, Encode, Decode, Clone, Serialize, Deserialize)]
pub struct NodesOpsMsg {
    pub nodes_duties: NodesDuties,
}

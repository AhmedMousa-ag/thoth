use crate::{
    operations::executer::types::OperationTypes, structs::numerics::structs::SharedNumeric,
};
use bincode::{Decode, Encode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct ExtraInfo {
    pub res_pos: Option<Vec<usize>>, //If it's a matrix or a list, two points should be maximum
    pub res_type: Option<SharedNumeric>, // To define the final output shape, shall it be matrix or vector or a scaler ?
    pub helper_number: Option<SharedNumeric>, // To help in operations like AVG where the number of elements is needed.
}

//TODO probably you would like to create functions instead of all of this mess.
#[derive(Debug, Encode, Decode, Serialize, Deserialize)]
pub struct Steps {
    pub node_id: String,
    pub operation_id: String,
    pub step_id: String,
    pub x: Option<SharedNumeric>,
    pub y: Option<SharedNumeric>,
    pub op_type: OperationTypes,
    pub result: Option<SharedNumeric>,
    pub next_step: Option<String>,
    pub prev_step: Option<String>,
    pub use_prev_res: bool, //If true, then this will be used instead of x.
    pub extra_info: Option<ExtraInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationFile {
    pub operation_id: String,
    pub result: Option<SharedNumeric>,
    pub execution_date: DateTime<Utc>,
}

// Manual implementation of Encode/Decode for OperationFile
impl bincode::Encode for OperationFile {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        self.operation_id.encode(encoder)?;
        self.result.encode(encoder)?;
        // Encode DateTime<Utc> as i64 timestamp
        self.execution_date.timestamp().encode(encoder)?;
        Ok(())
    }
}

impl<C> bincode::Decode<C> for OperationFile {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let operation_id = String::decode(decoder)?;
        let result = Option::<SharedNumeric>::decode(decoder)?;
        let timestamp = i64::decode(decoder)?;
        let dt = DateTime::from_timestamp(timestamp, 0)
            .ok_or(bincode::error::DecodeError::Other("Invalid timestamp"))?;
        let execution_date: DateTime<Utc> =
            DateTime::from_naive_utc_and_offset(dt.naive_utc(), Utc);
        Ok(OperationFile {
            operation_id,
            result,
            execution_date,
        })
    }
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

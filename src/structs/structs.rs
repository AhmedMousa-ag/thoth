use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Encode, Decode)]
pub struct NodeInfo {
    pub id: String,
    pub av_threads: usize,
    pub av_ram: u64,
}

#[derive(Clone)]
pub struct Topics {
    pub name: String,
}
// #[derive(Debug, Clone, PartialEq)]
// pub enum MessageParties {
//     InternalComponents,
//     NodesToNodes,
// }
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum RequestsTypes {
    RequestNodeInfo,
    ReplyNodeInfoUpdate,
    PlansToExecute,
    StartExecutePlan,
    EndedExecutingPlan,
}
impl RequestsTypes {
    pub fn as_str(&self) -> &str {
        match self {
            RequestsTypes::RequestNodeInfo => "Request Node Info",
            RequestsTypes::ReplyNodeInfoUpdate => "Reply Node Info Update",
            RequestsTypes::PlansToExecute => "Plan to Execute",
            RequestsTypes::StartExecutePlan => "Start Execute Plan",
            RequestsTypes::EndedExecutingPlan => "Ended Executing Plan",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Message {
    // pub parties: MessageParties,
    pub topic_name: String,
    pub request: RequestsTypes,
    pub message: Option<Vec<u8>>,
}

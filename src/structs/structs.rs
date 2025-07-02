use std::fmt;

use bincode::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct NodeInfo {
    pub id: String,
    pub av_threads: usize,
    pub av_ram: u64,
}

impl fmt::Display for NodeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(
            f,
            "Id: {}, Available Threads: {}, Available Ram: {}",
            self.id, self.av_threads, self.av_ram
        )
    }
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
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub enum RequestsTypes {
    RequestNodeInfo,
    ReplyNodeInfoUpdate,
    PlansToExecute,
}
impl RequestsTypes {
    pub fn as_str(&self) -> &str {
        match self {
            RequestsTypes::RequestNodeInfo => "Request Node Info",
            RequestsTypes::ReplyNodeInfoUpdate => "Reply Node Info Update",
            RequestsTypes::PlansToExecute => "Plan to Execute",
        }
    }
}

impl fmt::Display for RequestsTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Request Type: {}, ", self.as_str())
    }
}
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct Message {
    // pub parties: MessageParties,
    pub topic_name: String,
    pub request: RequestsTypes,
    pub message: Option<Vec<u8>>,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Topic Name: {}, Request Type: {}",
            self.topic_name, self.request
        )
    }
}

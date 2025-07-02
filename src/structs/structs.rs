use bincode::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
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
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub enum RequestsTypes {
    RequestNodeInfo,
    ReplyNodeInfoUpdate,
    PlansToExecute,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct Message {
    // pub parties: MessageParties,
    pub topic_name: String,
    pub request: RequestsTypes,
    pub message: Option<Vec<u8>>,
}

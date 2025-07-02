// #[derive(Debug, Clone, PartialEq)]
// pub enum MessageParties {
//     InternalComponents,
//     NodesToNodes,
// }
#[derive(Debug, Clone, PartialEq)]
pub enum RequestsTypes {
    RequestNodeInfo,
    ReplyNodeInfoUpdate,
    PlansToExecute,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    // pub parties: MessageParties,
    pub request: RequestsTypes,
    pub message: Option<Vec<u8>>,
}

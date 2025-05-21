#[derive(Debug, Clone, PartialEq)]
pub enum MessageParties {
    InternalComponents,
    NodesToNodes,
}
#[derive(Debug, Clone, PartialEq)]
pub enum RequestsTypes {
    RequestNodeInfo,
    ReplyNodeInfoUpdate,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message<T> {
    pub parties: MessageParties,
    pub request: RequestsTypes,
    pub message: Option<T>,
}

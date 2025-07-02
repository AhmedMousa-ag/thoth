use crate::{
    info,
    router::{
        messages::{Message, RequestsTypes},
        post_offices::external_com_ch::ExternalComm,
        traits::PostOfficeTrait,
    },
    structs::{structs::NodeInfo, traits::EncodingDecoding},
};

pub struct NodesOffice {}

impl PostOfficeTrait<NodeInfo> for NodesOffice {
    fn send_message(message: Box<NodeInfo>) {
        let rep_message = Box::new(Message {
            request: RequestsTypes::ReplyNodeInfoUpdate,
            message: Some(message.encode_bytes()),
        });
        ExternalComm::send_message(Box::clone(&rep_message));
        info!("Sent message in Nodes Office.");
    }
}

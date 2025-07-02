use crate::{
    connections::{channels_node_info::NodeInfoTrait, configs::topics::TopicsEnums},
    info,
    router::{
        post_offices::{
            external_com_ch::ExternalComm, //nodes_info::channel::InternalCommunications,
        },
        traits::PostOfficeTrait, // SenderReciverTrait},
    },
    structs::{
        structs::{Message, NodeInfo, RequestsTypes},
        traits::EncodingDecoding,
    },
};

pub struct NodesInfoOffice {}

impl PostOfficeTrait<NodeInfo> for NodesInfoOffice {
    fn send_message(message: Box<NodeInfo>) {
        let rep_message = Box::new(Message {
            topic_name: TopicsEnums::NodesInfo.as_str().to_string(),
            request: RequestsTypes::ReplyNodeInfoUpdate,
            message: Some(message.encode_bytes()),
        });
        ExternalComm::send_message(Box::clone(&rep_message));
        info!("Sent message in Nodes Office.");
    }
    async fn handle_incom_msg(message: Box<NodeInfo>) {
        NodeInfo::add_node(&message);
        // TODO you might use this code to trigger event to start the planned operations
        // InternalCommunications::get_sender_tx()
        //     .lock()
        //     .await
        //     .send(Box::new(Message {
        //         topic_name: TopicsEnums::NodesInfo.as_str().to_string(),
        //         request: RequestsTypes::ReplyNodeInfoUpdate,
        //         message: Some(NodeInfo::encode_bytes(&message)),
        //     }))
        //     .await.unwrap();
    }
}

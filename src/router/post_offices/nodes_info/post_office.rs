//Nodes Information
use super::channel::InternalNodesInfoCh;
use crate::connections::{types::NodeInfo,channels_node_info:: get_nodes_info};
use crate::router::{
    messages::{Message, MessageParties, RequestsTypes},
    traits::{PostOfficeTrait, SenderReciverTrait},
};
use tokio::runtime::Runtime;
use tokio::spawn;

pub struct CommunicationOffic {}

impl PostOfficeTrait<Vec<NodeInfo>> for CommunicationOffic {
    fn send_message(message: Vec<NodeInfo>) {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let rep_message = Message {
                parties: MessageParties::InternalComponents,
                request: RequestsTypes::ReplyNodeInfoUpdate,
                message: Some(message),
            };
            if let Err(e) = InternalNodesInfoCh::get_sender_tx()
                .lock()
                .await
                .send(Box::new(rep_message.clone()))
                .await
            {
                println!("Error Sending Message: {:?} , Error: {}", &rep_message, e);
            }
        });
    }

    fn start_back_office() {
        // Watch for internal communication requests
        spawn(async {
            loop {
                if let Some(message) = InternalNodesInfoCh::get_reciver_rx()
                    .lock()
                    .await
                    .recv()
                    .await
                {
                    if message.request == RequestsTypes::RequestNodeInfo {
                        let nodes_info = get_nodes_info().await;
                        Self::send_message(nodes_info);
                        println!("{:?}", message)
                    }
                }
            }
        });
    }
}

//Nodes Information
use super::super::traits::PostOfficeTrait;
use crate::connections::nodes_info::{NodeInfo, get_nodes_info};
use crate::router::{
    channels::{InternalExternal, SenderReciverTrait},
    messages::{Message, MessageParties, RequestsTypes},
};
use tokio::runtime::Runtime;
use tokio::spawn;

// Send and recieve messages about nodes information
pub struct InternalExternalOffic {}

impl PostOfficeTrait<Vec<NodeInfo>> for InternalExternalOffic {
    //TODO So far we only have one channel of communication, therefore we don't need to check all types and parties...etc
    fn send_message(message: Vec<NodeInfo>) {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let rep_message = Message {
                parties: MessageParties::ExternalInternal,
                request: RequestsTypes::ReplyNodeInfoUpdate,
                message: Some(message),
            };
            if let Err(e) = InternalExternal::get_sender_tx()
                .lock()
                .await
                .send(rep_message.clone())
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
                if let Some(message) = InternalExternal::get_reciver_rx().lock().await.recv().await
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

//Nodes Information
use super::super::traits::PostOfficeTrait;
use crate::connections::nodes_info::NodeInfo;
use crate::router::{
    channels::{InternalExternal, SenderReciverTrait},
    messages::{Message, MessageParties, RequestsTypes},
};
use tokio::runtime::Runtime;
use tokio::spawn;

pub struct InternalOffic {}

impl InternalOffic {
    pub fn request_nodes_info() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let rep_message = Message {
                parties: MessageParties::ExternalInternal,
                request: RequestsTypes::RequestNodeInfo,
                message: None,
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
}

impl PostOfficeTrait<Vec<NodeInfo>> for InternalOffic {
    fn send_message(message: Vec<NodeInfo>) {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            //TODO If requesting update, figure it out later.
            Self::request_nodes_info();
            println!("Sent message: {:?}", message);
        });
    }

    fn start_back_office() {
        // Watch for internal communication requests
        spawn(async {
            loop {
                if let Some(message) = InternalExternal::get_reciver_rx().lock().await.recv().await
                {
                    if message.request == RequestsTypes::ReplyNodeInfoUpdate {
                        if let Some(nodes_info) = message.message {
                            println!("Got nodes informations: {:?}", nodes_info);
                        }
                    }
                }
            }
        });
    }
}

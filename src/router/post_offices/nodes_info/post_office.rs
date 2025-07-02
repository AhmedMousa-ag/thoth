use crate::{
    connections::channels_node_info::NodeInfoTrait,
    err, info,
    router::{
        messages::{Message, RequestsTypes},
        post_offices::nodes_info::channel::InternalCommunications,
        traits::{PostOfficeTrait, SenderReciverTrait},
    },
    structs::{structs::NodeInfo, traits::EncodingDecoding},
};
use tokio::runtime::Handle;
use tokio::spawn;
use tokio::task::block_in_place;

pub struct CommunicationOffic {}

impl PostOfficeTrait<NodeInfo> for CommunicationOffic {
    fn send_message(message: Box<NodeInfo>) {
        
        block_in_place(|| {
            Handle::current().block_on(async {
                let rep_message =Box::new( Message {
                    request: RequestsTypes::ReplyNodeInfoUpdate,
                    message: Some(message.encode_bytes()),
                });
                if let Err(e) = InternalCommunications::get_sender_tx()
                    .lock()
                    .await
                    .send(Box::clone(&rep_message))
                    .await
                {
                    err!("Error Sending Message: {:?} , Error: {}", &rep_message, e);
                }
            })
        });
    }

}

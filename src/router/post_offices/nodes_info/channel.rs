use crate::router::{configs::config::CONFIGS, messages::Message, traits::SenderReciverTrait};
use lazy_static::lazy_static;
use tokio::sync::{
    Mutex,
    mpsc::{self, Receiver, Sender},
};

type NodesMessage = Box<Message>;
lazy_static! {
    static ref INTER_REQ_NODE: (Mutex<Sender<NodesMessage>>, Mutex<Receiver<NodesMessage>>) = {
        let (tx, rx) = mpsc::channel::<NodesMessage>(CONFIGS.max_ch_buff);
        (Mutex::new(tx), Mutex::new(rx))
    };
}
pub struct InternalCommunications {}
impl
    SenderReciverTrait<&'static Mutex<Sender<NodesMessage>>, &'static Mutex<Receiver<NodesMessage>>>
    for InternalCommunications
{
    fn get_sender_tx() -> &'static Mutex<Sender<NodesMessage>> {
        &INTER_REQ_NODE.0
    }

    fn get_reciver_rx() -> &'static Mutex<Receiver<NodesMessage>> {
        &INTER_REQ_NODE.1
    }
}

use crate::{
    connections::types::NodeInfo,
    router::{configs::config::CONFIGS, messages::Message, traits::SenderReciverTrait},
};
use lazy_static::lazy_static;
use tokio::sync::{
    Mutex,
    mpsc::{self, Receiver, Sender},
};

type NodesInfoMessagesType = Box<Message<Vec<NodeInfo>>>;
lazy_static! {
    static ref INTER_REQ_NODE: (
        Mutex<Sender<NodesInfoMessagesType>>,
        Mutex<Receiver<NodesInfoMessagesType>>
    ) = {
        let (tx, rx) = mpsc::channel::<NodesInfoMessagesType>(CONFIGS.max_ch_buff);
        (Mutex::new(tx), Mutex::new(rx))
    };
}
pub struct InternalNodesInfoCh {}
impl
    SenderReciverTrait<
        &'static Mutex<Sender<NodesInfoMessagesType>>,
        &'static Mutex<Receiver<NodesInfoMessagesType>>,
    > for InternalNodesInfoCh
{
    fn get_sender_tx() -> &'static Mutex<Sender<NodesInfoMessagesType>> {
        &INTER_REQ_NODE.0
    }

    fn get_reciver_rx() -> &'static Mutex<Receiver<NodesInfoMessagesType>> {
        &INTER_REQ_NODE.1
    }
}

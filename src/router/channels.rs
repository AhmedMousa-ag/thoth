use super::messages::Message;
use crate::connections::nodes_info::NodeInfo;
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
        let (tx, rx) = mpsc::channel::<NodesInfoMessagesType>(500);
        (Mutex::new(tx), Mutex::new(rx))
    };
}
pub trait SenderReciverTrait<S, R> {
    fn get_sender_tx() -> S;
    fn get_reciver_rx() -> R;
}

pub struct InternalExternal {}

impl
    SenderReciverTrait<
        &'static Mutex<Sender<NodesInfoMessagesType>>,
        &'static Mutex<Receiver<NodesInfoMessagesType>>,
    > for InternalExternal
{
    fn get_sender_tx() -> &'static Mutex<Sender<NodesInfoMessagesType>> {
        &INTER_REQ_NODE.0
    }

    fn get_reciver_rx() -> &'static Mutex<Receiver<NodesInfoMessagesType>> {
        &INTER_REQ_NODE.1
    }
}

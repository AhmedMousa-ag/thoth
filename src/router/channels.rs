use super::messages::Message;
use crate::connections::nodes_info::NodeInfo;
use lazy_static::lazy_static;
use tokio::sync::{
    Mutex,
    mpsc::{self, Receiver, Sender},
};
lazy_static! {
    static ref INTER_REQ_NODE: (
        Mutex<Sender<Message<Vec<NodeInfo>>>>,
        Mutex<Receiver<Message<Vec<NodeInfo>>>>
    ) = {
        let (tx, rx) = mpsc::channel::<Message<Vec<NodeInfo>>>(500);
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
        &'static Mutex<Sender<Message<Vec<NodeInfo>>>>,
        &'static Mutex<Receiver<Message<Vec<NodeInfo>>>>,
    > for InternalExternal
{
    fn get_sender_tx() -> &'static Mutex<Sender<Message<Vec<NodeInfo>>>> {
        &INTER_REQ_NODE.0
    }

    fn get_reciver_rx() -> &'static Mutex<Receiver<Message<Vec<NodeInfo>>>> {
        &INTER_REQ_NODE.1
    }
}

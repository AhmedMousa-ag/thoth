use crate::{
    err,
    router::{configs::config::CONFIGS, traits::SenderReciverTrait},
    structs::structs::Message,
};
use lazy_static::lazy_static;
use tokio::{
    runtime::Handle,
    sync::{
        Mutex,
        mpsc::{self, Receiver, Sender},
    },
    task::block_in_place,
};

pub type NodesMessage = Box<Message>;

lazy_static! {
    static ref INTER_REQ_NODE: (Mutex<Sender<NodesMessage>>, Mutex<Receiver<NodesMessage>>) = {
        let (tx, rx) = mpsc::channel::<NodesMessage>(CONFIGS.max_ch_buff);
        (Mutex::new(tx), Mutex::new(rx))
    };
}
pub struct ExternalComm {}
impl
    SenderReciverTrait<&'static Mutex<Sender<NodesMessage>>, &'static Mutex<Receiver<NodesMessage>>>
    for ExternalComm
{
    fn get_sender_tx() -> &'static Mutex<Sender<NodesMessage>> {
        &INTER_REQ_NODE.0
    }

    fn get_reciver_rx() -> &'static Mutex<Receiver<NodesMessage>> {
        &INTER_REQ_NODE.1
    }
}

impl ExternalComm {
    pub fn send_message(message: NodesMessage) {
        block_in_place(|| {
            Handle::current().block_on(async {
                if let Err(e) = ExternalComm::get_sender_tx()
                    .lock()
                    .await
                    .send(Box::clone(&message))
                    .await
                {
                    err!("Error Sending Message: {:?} , Error: {}", &message, e);
                }
            })
        })
    }
    pub async fn recieve_messages() -> Option<NodesMessage> {
        ExternalComm::get_reciver_rx().lock().await.recv().await
    }
}

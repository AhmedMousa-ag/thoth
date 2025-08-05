use lazy_static::lazy_static;
use tokio::sync::{
    Mutex,
    mpsc::{self, UnboundedReceiver, UnboundedSender},
};

use crate::syncer::structs::SyncMessage;

lazy_static! {
    static ref CHANNEL: (
        UnboundedSender<SyncMessage>,
        Mutex<UnboundedReceiver<SyncMessage>>
    ) = {
        let (tx, rx) = mpsc::unbounded_channel();
        (tx, Mutex::new(rx))
    };
}

pub fn get_sender() -> UnboundedSender<SyncMessage> {
    CHANNEL.0.clone()
}

pub fn get_reciever() -> &'static Mutex<UnboundedReceiver<SyncMessage>> {
    &CHANNEL.1
}

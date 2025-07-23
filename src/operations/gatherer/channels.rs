use crate::operations::gatherer::structs::GatheredMessage;
use lazy_static::lazy_static;
use tokio::sync::{
    Mutex,
    mpsc::{self, UnboundedReceiver, UnboundedSender},
};

lazy_static! {
    static ref GATHERD_CH: (
        UnboundedSender<GatheredMessage>,
        Mutex<UnboundedReceiver<GatheredMessage>>
    ) = {
        let (tx, rx) = mpsc::unbounded_channel();
        (tx, Mutex::new(rx))
    };
}

pub fn get_sender_tx() -> &'static UnboundedSender<GatheredMessage> {
    &GATHERD_CH.0
}

pub fn get_reciver_rx() -> &'static Mutex<UnboundedReceiver<GatheredMessage>> {
    &GATHERD_CH.1
}

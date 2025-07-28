use std::collections::HashMap;

use crate::operations::gatherer::structs::GatheredMessage;
use lazy_static::lazy_static;
use tokio::sync::{Mutex, mpsc::UnboundedSender};

lazy_static! {
    // static ref GATHERD_CH: (
    //     UnboundedSender<GatheredMessage>,
    //     Mutex<UnboundedReceiver<GatheredMessage>>
    // ) = {
    //     let (tx, rx) = mpsc::unbounded_channel();
    //     (tx, Mutex::new(rx))
    // };
    static ref OPENED_CH:Mutex<HashMap<String,UnboundedSender<GatheredMessage>>>=Mutex::new(HashMap::new());
}

// pub fn get_sender_tx() -> &'static UnboundedSender<GatheredMessage> {
//     &GATHERD_CH.0
// }

// pub fn get_reciver_rx() -> &'static Mutex<UnboundedReceiver<GatheredMessage>> {
//     &GATHERD_CH.1
// }

///Will be added when the gatherer is constructed.
pub fn add_ch_sender(operation_id: String, ch: UnboundedSender<GatheredMessage>) {
    OPENED_CH.try_lock().unwrap().insert(operation_id, ch);
}

///It will be used in the connection crate with gossib, check if the channel is opened, then this is the controller that gathers all messages.
pub fn get_opened_ch_sender(operation_id: &str) -> Option<UnboundedSender<GatheredMessage>> {
    OPENED_CH.try_lock().unwrap().get(operation_id).cloned()
}

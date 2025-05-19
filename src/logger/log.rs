use tokio::sync::mpsc;
use tokio::sync::{mpsc::{ Receiver, Sender},RwLock};

use lazy_static::lazy_static;

pub struct LoggingMessage{
    log:String,
}
lazy_static! {
    static ref CHANNEL: (RwLock<Sender<LoggingMessage>>, RwLock<Receiver<LoggingMessage>>) = {
        let (tx, rx) = mpsc::channel(10000);
        (RwLock::new(tx), RwLock::new(rx))
    };
}

pub trait SendMessages{
    // Accessor functions to get the TX and RX parts
    fn get_sender_tx() -> &'static RwLock<Sender<LoggingMessage>> {
        &CHANNEL.0
    }

    fn get_reciver_rx() -> &'static RwLock<Receiver<LoggingMessage>> {
        &CHANNEL.1
    }
}


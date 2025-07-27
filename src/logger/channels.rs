use lazy_static::lazy_static;
use tokio::sync::{
    Mutex,
    mpsc::{self, UnboundedReceiver, UnboundedSender},
};

lazy_static! {
    static ref INFO_CH: (UnboundedSender<String>, Mutex<UnboundedReceiver<String>>) = {
        let (tx, rx) = mpsc::unbounded_channel();
        (tx, Mutex::new(rx))
    };
    static ref DEBUG_CH: (UnboundedSender<String>, Mutex<UnboundedReceiver<String>>) = {
        let (tx, rx) = mpsc::unbounded_channel();
        (tx, Mutex::new(rx))
    };
    static ref WARN_CH: (UnboundedSender<String>, Mutex<UnboundedReceiver<String>>) = {
        let (tx, rx) = mpsc::unbounded_channel();
        (tx, Mutex::new(rx))
    };
    static ref ERR_CH: (UnboundedSender<String>, Mutex<UnboundedReceiver<String>>) = {
        let (tx, rx) = mpsc::unbounded_channel();
        (tx, Mutex::new(rx))
    };
    static ref OPS_CH: (UnboundedSender<String>, Mutex<UnboundedReceiver<String>>) = {
        let (tx, rx) = mpsc::unbounded_channel();
        (tx, Mutex::new(rx))
    };
}

pub fn get_info_sender() -> &'static UnboundedSender<String> {
    &INFO_CH.0
}

pub fn get_info_reciever() -> &'static Mutex<UnboundedReceiver<String>> {
    &INFO_CH.1
}

pub fn get_debug_sender() -> &'static UnboundedSender<String> {
    &DEBUG_CH.0
}

pub fn get_debug_reciever() -> &'static Mutex<UnboundedReceiver<String>> {
    &DEBUG_CH.1
}

pub fn get_warn_sender() -> &'static UnboundedSender<String> {
    &WARN_CH.0
}

pub fn get_warn_reciever() -> &'static Mutex<UnboundedReceiver<String>> {
    &WARN_CH.1
}

pub fn get_err_sender() -> &'static UnboundedSender<String> {
    &ERR_CH.0
}

pub fn get_err_reciever() -> &'static Mutex<UnboundedReceiver<String>> {
    &ERR_CH.1
}

pub fn get_ops_sender() -> &'static UnboundedSender<String> {
    &OPS_CH.0
}

pub fn get_ops_reciever() -> &'static Mutex<UnboundedReceiver<String>> {
    &OPS_CH.1
}

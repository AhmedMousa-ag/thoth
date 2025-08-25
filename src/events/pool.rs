use crate::events::events::Event;
use lazy_static::lazy_static;
use std::collections::HashMap;
use tokio::sync::{
    Mutex,
    MutexGuard,
    // mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
};

type PoolItems = HashMap<String, Event>;
lazy_static! {
    static ref EVENTS_POOL: Mutex<PoolItems> = Mutex::new(HashMap::new());
    // static ref EVENTS_CHANNEL: (UnboundedSender<Event>, Mutex<UnboundedReceiver<Event>>) = {
    //     let (tx, rx) = unbounded_channel();
    //     (tx, Mutex::new(rx))
    // };
}

pub async fn add_event(event_id: &str, event: Event) {
    let mut pool = EVENTS_POOL.lock().await;
    pool.insert(event_id.to_string(), event);
}
pub async fn get_pool() -> MutexGuard<'static, PoolItems> {
    EVENTS_POOL.lock().await
}

pub async fn remove_event(event_id: &str) -> Option<Event> {
    let mut pool = EVENTS_POOL.lock().await;
    if let Some(mut event) = pool.get(event_id).cloned() {
        event.num_triggers -= 1;
        if event.num_triggers < 1 {
            return pool.remove(event_id);
        } else {
            pool.insert(event_id.to_string(), event.clone());
            return Some(event);
        }
    }
    None
}

// pub fn get_events_sender_channel() -> UnboundedSender<Event> {
//     EVENTS_CHANNEL.0.clone()
// }

// pub fn get_events_receiver_channel() -> &'static Mutex<UnboundedReceiver<Event>> {
//     &EVENTS_CHANNEL.1
// }

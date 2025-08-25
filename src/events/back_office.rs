use std::collections::HashMap;

use futures::lock::Mutex;
use lazy_static::lazy_static;
use tokio::{
    spawn,
    sync::{
        RwLock,
        mpsc::{UnboundedReceiver, UnboundedSender},
    },
};

use crate::{
    err,
    events::{
        events::{Event, EventType},
        pool::{add_event, get_pool, remove_event},
    },
    info, warn,
};
lazy_static! {
    static ref LISTENER_CHANNELS: RwLock<HashMap<String, Vec<UnboundedSender<Event>>>> =
        RwLock::new(HashMap::new());
    static ref BACK_OFFICE_CHANNELS: (UnboundedSender<Event>, Mutex<UnboundedReceiver<Event>>) = {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        (sender, Mutex::new(receiver))
    };
}

/// Will be used on listeners constructor to tell the communication office it can communicate with it on this channel.
pub async fn insert_listener_sender_channel(id: String, sender: UnboundedSender<Event>) {
    let mut listeners = LISTENER_CHANNELS.write().await;
    let existing_senders = listeners.entry(id.clone()).or_insert_with(Vec::new);
    for existing_sender in existing_senders.iter() {
        if existing_sender.same_channel(&sender) {
            return;
        }
    }
    existing_senders.push(sender);
}

pub async fn remove_listener_sender_channel(id: &str, sender: &UnboundedSender<Event>) {
    let mut listeners = LISTENER_CHANNELS.write().await;
    if let Some(existing_senders) = listeners.get_mut(id) {
        existing_senders.retain(|existing_sender| !existing_sender.same_channel(sender));
        if existing_senders.is_empty() {
            listeners.remove(id);
        }
    }
}

pub fn get_back_office_sender_channel() -> UnboundedSender<Event> {
    BACK_OFFICE_CHANNELS.0.clone()
}
pub struct EventsCommunicationOffice {}

impl EventsCommunicationOffice {
    pub fn add_event_to_pool(id: &str, thread: bool) {
        let id = id.to_string();

        let fnc = async move {
            if let Some(event) = get_pool().await.get_mut(&id) {
                event.num_triggers += 1;
            } else {
                let message = Event {
                    id: id.to_string(),
                    event_type: EventType::RestingState,
                    num_triggers: 0,
                };

                add_event(&id, message).await;
            }
        };
        if thread {
            spawn(fnc);
        } else {
            tokio::task::block_in_place(|| {
                let rt = tokio::runtime::Handle::current();
                rt.block_on(fnc);
            });
        }
    }
    pub fn remove_event_from_pool(id: &str, thread: bool) {
        let id = id.to_string();
        let fnc = async move {
            if let Some(event) = remove_event(&id).await {
                if event.num_triggers > 0 {
                    return;
                }
            }
            Self::notify_removal(&id).await;
        };
        if thread {
            spawn(fnc);
        } else {
            tokio::task::block_in_place(|| {
                let rt = tokio::runtime::Handle::current();
                rt.block_on(fnc);
            });
        }
    }

    pub fn start_back_office() {
        // Watch for internal communication requests
        spawn(async move {
            info!("Started Pool back office");
            let mut receiver = BACK_OFFICE_CHANNELS.1.lock().await;
            loop {
                if let Some(message) = receiver.recv().await {
                    spawn(Self::handle_event(message));
                }
            }
        });
    }

    async fn handle_event(message: Event) {
        info!("Handling event: {}", message.id);
        match message.event_type {
            EventType::AddEvent => {
                info!("Adding event to pool: {}", message.id);
                // Handle adding message to the pool.
                Self::add_event_to_pool(&message.id, true);
            }
            EventType::RemovedEvent => {
                info!("Retrieving event from pool: {}", message.id);
                // Handle removal of pool messages.
                let retrieved_event = remove_event(&message.id).await;
                if let Some(event) = retrieved_event {
                    info!("Removed event: {}", event);
                }
            }
            _ => {
                warn!("Unknown event type: {:?}", message.event_type);
            }
        }
    }

    async fn notify_removal(id: &str) {
        if let Some(listeners) = LISTENER_CHANNELS.read().await.get(id).cloned() {
            for sender in listeners {
                if let Err(e) = sender.send(Event {
                    id: id.to_string(),
                    event_type: EventType::RemovedEvent,
                    num_triggers: 0,
                }) {
                    err!("Failed to send event to listener: {}", e);
                }
            }
        }
    }
}

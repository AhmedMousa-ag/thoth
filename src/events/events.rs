use std::thread::spawn;

use tokio::sync::{
    Mutex,
    mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
};

use crate::{
    err,
    events::{
        back_office::{
            get_back_office_sender_channel, insert_listener_sender_channel,
            remove_listener_sender_channel,
        },
        pool::remove_event,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    RestingState,
    AddEvent,
    RemovedEvent,
}
impl EventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::RestingState => "RestingState",
            EventType::AddEvent => "AddEvent",
            EventType::RemovedEvent => "RemovedEvent",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub id: String,
    pub event_type: EventType,
    pub num_triggers: usize,
}

#[derive(Debug)]
pub struct EventListener {
    pub id: String,
    pub reciever_ch: Mutex<UnboundedReceiver<Event>>,
    pub sender_ch: UnboundedSender<Event>,
}

impl EventListener {
    pub fn new(id: String) -> Self {
        let (sender, channel) = unbounded_channel();
        tokio::task::block_in_place(|| {
            // Use a runtime to block on the async function
            let rt = tokio::runtime::Handle::current();
            rt.block_on(insert_listener_sender_channel(id.clone(), sender.clone()));
        });
        EventListener {
            id,
            reciever_ch: Mutex::new(channel),
            sender_ch: sender,
        }
    }
    pub fn wait_for_event(&self) -> Event {
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Handle::current();
            rt.block_on(async {
                if let Some(removed_event) = remove_event(self.id.as_str()).await {
                    if removed_event.num_triggers < 1 {
                        return removed_event;
                    }
                }

                let mut channel = self.reciever_ch.lock().await;
                loop {
                    if let Some(event) = channel.recv().await {
                        if event.event_type == EventType::RemovedEvent
                            && event.id == self.id
                            && event.num_triggers < 1
                        {
                            return event;
                        }
                    };
                }
            })
        })
    }
}

impl Drop for EventListener {
    fn drop(&mut self) {
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Handle::current();
            rt.block_on(async {
                remove_listener_sender_channel(&self.id, &self.sender_ch).await;
            });
        });
    }
}

pub struct EventsHandler {
    pub id: String,
    pub listener: EventListener,
}
impl EventsHandler {
    pub fn new(id: &str) -> Self {
        let listener = EventListener::new(id.to_string());
        EventsHandler { id: id.to_string(), listener }
    }
    pub fn new_read_step(id:&str) -> Self {
        let id = format!("read_{}", id);
        Self::new_step(&id)
    }
    pub fn new_write_step(id:&str) -> Self {
        let id = format!("write_{}", id);
        Self::new_step(&id)
    }
    pub fn new_step(id: &str) -> Self {
        let id = format!("step_{}", id);
        Self::new(&id)
    }
    pub fn new_operation(id: &str) -> Self {
        let id = format!("operation_{}", id);
        Self::new(&id)
    }
    pub fn exists(&self) -> bool {
        let event = self.listener.wait_for_event();
        event.num_triggers < 1
    }
    pub fn add_event(self, thread: bool) -> Self {
        let id = self.listener.id.clone();
        let fnc = move || {
            let event = Event {
                id,
                event_type: EventType::AddEvent,
                num_triggers: 0,
            };
            get_back_office_sender_channel()
                .send(event.clone())
                .unwrap_or_else(|e| {
                    err!(
                        "Failed to send Adding event to pool: {}, Event: {:?}",
                        e,
                        event
                    )
                });
        };
        if thread {
            spawn(fnc);
        } else {
            fnc();
        }
        return self;
    }
    
    pub fn remove_event(self, thread: bool)->Self {
        let id = self.listener.id.clone();
        let fnc = move || {
            let event = Event {
                id,
                event_type: EventType::RemovedEvent,
                num_triggers: 0,
            };
            get_back_office_sender_channel()
                .send(event.clone())
                .unwrap_or_else(|e| {
                    err!(
                        "Failed to send Removing event to pool: {}, Event: {:?}",
                        e,
                        event
                    )
                });
        };
        if thread {
            spawn(fnc);
        } else {
            fnc();
        }
        return self;
    }
}

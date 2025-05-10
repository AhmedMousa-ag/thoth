use lazy_static::lazy_static;
use libp2p::{gossipsub::IdentTopic, swarm};
use std::collections::HashMap;

use crate::connections::types::GossipBehaviour;

lazy_static! {
    static ref TOPICS: HashMap<&'static str, IdentTopic> = {
        let all_topics = vec!["operations"];

        let mut m = HashMap::new();
        for topic_name in all_topics.iter() {
            m.insert(*topic_name, IdentTopic::new(topic_name.to_string()));
        }
        m
    };
}

pub fn get_topics<'a>() -> Vec<&'a IdentTopic> {
    let mut topics = Vec::new();
    for (_, topic) in TOPICS.iter() {
        topics.push(topic);
    }
    topics
}

pub fn send_message(
    swarm: &mut swarm::Swarm<GossipBehaviour>,
    message: String,
    topic_name: &str,
) -> bool {
    // let topic = get_topics().read().unwrap().clone().into_iter().nth(0);
    match TOPICS.get(topic_name) {
        Some(topic) => {
            if let Err(e) = swarm
                .behaviour_mut()
                .gossipsub
                .publish(topic.clone(), message.as_bytes())
            {
                println!("Publish error: {e:?}");
            };
            true
        }
        _ => false,
    }
}

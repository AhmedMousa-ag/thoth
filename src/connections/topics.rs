use lazy_static::lazy_static;
use libp2p::gossipsub::IdentTopic;
use std::collections::HashMap;

lazy_static! {
    static ref TOPICS: HashMap<&'static str, IdentTopic> = {
        let all_topics = ["operations"];

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

pub fn get_topic(topic_name: &str) -> Option<&IdentTopic> {
    TOPICS.get(topic_name)
}

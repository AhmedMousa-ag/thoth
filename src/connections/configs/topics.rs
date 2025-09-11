use lazy_static::lazy_static;
use libp2p::gossipsub::IdentTopic;
use std::collections::HashMap;

pub enum TopicsEnums {
    Operations,
    NodesInfo,
    Sync,
}

impl TopicsEnums {
    pub fn as_str(&self) -> &str {
        match self {
            TopicsEnums::Operations => "OPERATIONS",
            TopicsEnums::NodesInfo => "NODES_INFO",
            TopicsEnums::Sync => "SYNC",
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            TopicsEnums::Operations => String::from("OPERATIONS"),
            TopicsEnums::NodesInfo => String::from("NODES_INFO"),
            TopicsEnums::Sync => String::from("SYNC"),
        }
    }
}

lazy_static! {
    static ref TOPICS: HashMap<&'static str, IdentTopic> = {
        let all_topics = [
            TopicsEnums::Operations.as_str(),
            TopicsEnums::NodesInfo.as_str(),
            TopicsEnums::Sync.as_str(),
        ];

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

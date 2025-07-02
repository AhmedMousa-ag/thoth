use bincode::{Decode, Encode};

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct NodeInfo {
    pub id: String,
    pub av_threads: usize,
    pub av_ram: u64,
}
#[derive(Clone)]
pub struct Topics {
    pub name: String,
}
#[derive(Clone, Encode, Decode)]
pub struct Messages {
    pub topic_name: String,
    pub message: Vec<u8>,
}

use super::structs::{Message, NodeInfo};
use crate::{
    operations::{
        gatherer::structs::GatheredMessage,
        planner::charts::structs::{NodesOpsMsg, Steps},
    },
    syncer::structs::SyncMessage,
};
use bincode::config;

pub trait EncodingDecoding {
    fn encode_bytes(&self) -> Vec<u8>;
    fn decode_bytes(bytes: &[u8]) -> Self;
}

impl EncodingDecoding for Message {
    fn encode_bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, config::standard()).unwrap()
    }

    fn decode_bytes(bytes: &[u8]) -> Self {
        let (messages, _): (Self, usize) =
            bincode::decode_from_slice(bytes, config::standard()).unwrap();
        messages
    }
}

impl EncodingDecoding for NodeInfo {
    fn encode_bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, config::standard()).unwrap()
    }

    fn decode_bytes(bytes: &[u8]) -> Self {
        let (messages, _): (Self, usize) =
            bincode::decode_from_slice(bytes, config::standard()).unwrap();
        messages
    }
}

impl EncodingDecoding for Steps {
    fn encode_bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, config::standard()).unwrap()
    }

    fn decode_bytes(bytes: &[u8]) -> Self {
        let (messages, _): (Self, usize) =
            bincode::decode_from_slice(bytes, config::standard()).unwrap();
        messages
    }
}

impl EncodingDecoding for NodesOpsMsg {
    fn encode_bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, config::standard()).unwrap()
    }

    fn decode_bytes(bytes: &[u8]) -> Self {
        let (messages, _): (Self, usize) =
            bincode::decode_from_slice(bytes, config::standard()).unwrap();
        messages
    }
}

impl EncodingDecoding for GatheredMessage {
    fn encode_bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, config::standard()).unwrap()
    }

    fn decode_bytes(bytes: &[u8]) -> Self {
        let (messages, _): (Self, usize) =
            bincode::decode_from_slice(bytes, config::standard()).unwrap();
        messages
    }
}

impl EncodingDecoding for SyncMessage {
    fn encode_bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, config::standard()).unwrap()
    }

    fn decode_bytes(bytes: &[u8]) -> Self {
        let (messages, _): (Self, usize) =
            bincode::decode_from_slice(bytes, config::standard()).unwrap();
        messages
    }
}

use bincode::{Decode, Encode, config};
use libp2p::{gossipsub, mdns, ping, swarm::NetworkBehaviour};
// We create a custom network behaviour that combines Gossipsub and Mdns.
#[derive(NetworkBehaviour)]
pub struct GossipBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
    pub ping: ping::Behaviour,
}
#[derive(Clone, PartialEq, Debug,Encode,Decode)]
pub struct NodeInfo {
    pub id: String,
    pub ip: String,
    pub av_threads: i32,
    pub av_ram: i64, //MB
}

#[derive(Clone)]
pub struct Topics {
    pub name: String,
}

#[derive(Clone, Encode, Decode)]
pub struct Messages<T> {
    pub topic_name: String,
    pub message: T,
}

pub trait EncodingDecoding {
    fn encode_bytes(&self) -> Vec<u8>;
    fn decode_bytes(&self, bytes: &[u8]) -> Self;
}

impl EncodingDecoding for Messages<String> {
    fn encode_bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, config::standard()).unwrap()
    }

    fn decode_bytes(&self, bytes: &[u8]) -> Self {
        let (messages, _): (Self, usize) =
            bincode::decode_from_slice(bytes, config::standard()).unwrap();
        messages
    }
}



impl EncodingDecoding for Messages<NodeInfo> {
    fn encode_bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, config::standard()).unwrap()
    }

    fn decode_bytes(&self, bytes: &[u8]) -> Self {
        let (messages, _): (Self, usize) =
            bincode::decode_from_slice(bytes, config::standard()).unwrap();
        messages
    }
}

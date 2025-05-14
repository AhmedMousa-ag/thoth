use bincode::{Decode, Encode, config};
use libp2p::{gossipsub, mdns, swarm::NetworkBehaviour,ping};

// We create a custom network behaviour that combines Gossipsub and Mdns.
#[derive(NetworkBehaviour)]
pub struct GossipBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
    pub ping: ping::Behaviour,
}

#[derive(Clone)]
pub struct Topics {
    pub name: String,
}

#[derive(Clone, Encode, Decode)]
pub struct Messages {
    pub topic_name: String,
    pub message: String,
}

pub trait EncodingDecoding {
    fn encode_bytes(&self) -> Vec<u8>;
    fn decode_bytes(&self, bytes: &[u8]) -> Self;
}

impl EncodingDecoding for Messages {
    fn encode_bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, config::standard()).unwrap()
    }

    fn decode_bytes(&self, bytes: &[u8]) -> Self {
        let (messages, _): (Self, usize) =
            bincode::decode_from_slice(bytes, config::standard()).unwrap();
        messages
    }
}

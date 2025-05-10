use libp2p::{gossipsub, mdns, swarm::NetworkBehaviour};

// We create a custom network behaviour that combines Gossipsub and Mdns.
#[derive(NetworkBehaviour)]
pub struct GossipBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}

#[derive(Clone)]
pub struct Topics {
    pub name: String,
}

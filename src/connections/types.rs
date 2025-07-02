use libp2p::{gossipsub, mdns, ping, swarm::NetworkBehaviour};
#[derive(NetworkBehaviour)]
pub struct GossipBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
    pub ping: ping::Behaviour,
}

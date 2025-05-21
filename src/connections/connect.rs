use super::types::EncodingDecoding;
use crate::connections::{
    configs::config::CONFIGS,
    configs::topics::{get_topic, get_topics},
    types::{GossipBehaviour, GossipBehaviourEvent, Messages},
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};

use libp2p::{
    Swarm, gossipsub, mdns, noise, ping,
    swarm::{self, SwarmEvent},
    tcp, yamux,
};
use tokio::io;
use tracing_subscriber::EnvFilter;

use futures::stream::StreamExt;
use lazy_static::lazy_static;
use std::error::Error;
use tokio::select;
use tokio::sync::RwLock;
use tokio::sync::mpsc::{self, Receiver, Sender};

lazy_static! {
    static ref CHANNEL: (RwLock<Sender<Messages>>, RwLock<Receiver<Messages>>) = {
        let (tx, rx) = mpsc::channel(100);
        (RwLock::new(tx), RwLock::new(rx))
    };
}

pub struct GossibConnection {}
impl GossibConnection {
    pub async fn p2pconnect() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut swarm: Swarm<GossipBehaviour> = Self::create_gossip_swarm();
        // Create a Gossipsub topics
        for topic in get_topics().iter() {
            // subscribes to our topic
            swarm.behaviour_mut().gossipsub.subscribe(topic)?;
        }

        // Listen on all interfaces and whatever port the OS assigns
        // swarm.listen_on(format!("/ip4/0.0.0.0/udp/{}/quic-v1", port).parse()?)?;
        swarm.listen_on(format!("/ip4/0.0.0.0/tcp/{}", CONFIGS.port).parse()?)?;

        Self::listen_messages(&mut swarm).await
    }

    pub fn create_gossip_swarm() -> Swarm<GossipBehaviour> {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .try_init();
        let swarm: Swarm<GossipBehaviour> = libp2p::SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )
            .expect("Error building swarm tcp.")
            .with_quic()
            .with_behaviour(|key| {
                // To content-address message, we can take the hash of message and use it as an ID.
                let message_id_fn = |message: &gossipsub::Message| {
                    let mut s = DefaultHasher::new();
                    message.data.hash(&mut s);
                    gossipsub::MessageId::from(s.finish().to_string())
                };
                // Set a custom gossipsub configuration
                let gossipsub_config = gossipsub::ConfigBuilder::default()
                    // .heartbeat_initial_delay(Duration::from_secs(5))
                    .heartbeat_interval(Duration::from_secs(1))
                    .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message
                    // signing)
                    .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
                    .build()
                    .map_err(io::Error::other)
                    .expect("Error building gossib configuration."); // Temporary hack because `build` does not return a proper `std::error::Error`.

                // build a gossipsub network behaviour
                let gossipsub = gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossipsub_config,
                )
                .expect("Error building gossib builder.");

                let mdns =
                    mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())
                        .expect("Error building tokio behavior.");
                Ok(GossipBehaviour {
                    gossipsub,
                    mdns,
                    ping: ping::Behaviour::default(),
                })
            })
            .expect("Error building swarms.")
            .build();
        swarm
    }

    async fn listen_messages(
        swarm: &mut swarm::Swarm<GossipBehaviour>,
        // rx: &mut Receiver<Messages>,
    ) -> ! {
        loop {
            // SwarmEvent::IncomingConnection { connection_id: (), local_addr: (), send_back_addr: () }
            select! {
                rec_message=recieve_messages()=>{
                    if rec_message.is_some() {
                        let message = rec_message.unwrap();
                        let rec_topic = get_topic(message.topic_name.as_str());
                        if rec_topic.is_some(){
                            let topic = rec_topic.unwrap().clone();
                            if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic, message.encode_bytes()){
                                println!("Error publishing message: {:?}",e);
                            };
                        }
                    }
                },

                event = swarm.select_next_some() => match event {
                    SwarmEvent::Behaviour(GossipBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                        for (peer_id, _multiaddr) in list {
                            println!("mDNS discovered a new peer: {peer_id}");
                            swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                        }
                    },
                    SwarmEvent::Behaviour(GossipBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                        for (peer_id, _multiaddr) in list {
                            println!("mDNS discover peer has expired: {peer_id}");
                            swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                        }
                    },
                    SwarmEvent::Behaviour(GossipBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                        propagation_source: peer_id,
                        message_id: id,
                        message,
                    })) => println!(
                            "Got message: '{}' with id: {id} from peer: {peer_id}",
                            String::from_utf8_lossy(&message.data),
                        ),
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("Local node is listening on {address}");
                    },
                    SwarmEvent::ConnectionEstablished{peer_id, connection_id,num_established,..}=>{
                        println!("Established Connection id: {}, peer id: {}, number of established: {}",connection_id,peer_id ,num_established)
                    },
                    SwarmEvent::ConnectionClosed{peer_id, connection_id,num_established,cause,..}=>{
                        println!("Connection Closed id: {}, peer id: {}, number of established: {},  due to: {:?}",connection_id,peer_id ,num_established,cause)
                    },
                    SwarmEvent::IncomingConnection{connection_id,local_addr,send_back_addr}=>{
                        println!("Incomming connection id: {}, local address: {} send back address: {}",connection_id,local_addr,send_back_addr)
                    },
                    SwarmEvent::IncomingConnectionError{connection_id,error,..}=>{
                        println!("Incoming Connection Error on id: {} and the error: {}",connection_id,error)
                },
                    _ => {}//{println!("None of these options: {:?}",event)}
                }
            }
        }
    }
}

// Accessor functions to get the TX and RX parts
fn get_sender_tx() -> &'static RwLock<Sender<Messages>> {
    &CHANNEL.0
}

fn get_reciver_rx() -> &'static RwLock<Receiver<Messages>> {
    &CHANNEL.1
}

pub async fn send_messages(message: Messages) {
    if let Err(e) = get_sender_tx().write().await.send(message).await {
        println!("Error sending message: {:?}", e);
    };
}
async fn recieve_messages() -> Option<Messages> {
    get_reciver_rx().write().await.recv().await
}

use super::types::EncodingDecoding;
use crate::connections::{
    config::create_gossip_swarm,
    topics::{get_topic, get_topics},
    types::{GossipBehaviour, GossipBehaviourEvent, Messages},
};
use futures::stream::StreamExt;
use lazy_static::lazy_static;
use libp2p::{
    gossipsub::{self},
    mdns,
    swarm::{self, Swarm, SwarmEvent},
};
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

// Accessor functions to get the TX and RX parts
fn get_sender_tx() -> &'static RwLock<Sender<Messages>> {
    &CHANNEL.0
}

fn get_reciver_rx() -> &'static RwLock<Receiver<Messages>> {
    &CHANNEL.1
}
pub async fn p2pconnect() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut swarm: Swarm<GossipBehaviour> = create_gossip_swarm();
    // Create a Gossipsub topics
    for topic in get_topics().iter() {
        // subscribes to our topic
        swarm.behaviour_mut().gossipsub.subscribe(topic)?;
    }

    // Listen on all interfaces and whatever port the OS assigns
    let port = 49221;
    // let port=0;

    swarm.listen_on(format!("/ip4/0.0.0.0/udp/{}/quic-v1", port).parse()?)?;
    swarm.listen_on(format!("/ip4/0.0.0.0/tcp/{:}", port).parse()?)?;

    listen_messages(&mut swarm).await
}

pub async fn send_messages(message: Messages) {
    if let Err(e) = get_sender_tx().write().await.send(message).await {
        println!("Error sending message: {:?}", e);
    };
}
async fn recieve_messages() -> Option<Messages> {
    get_reciver_rx().write().await.recv().await
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
                _ => {println!("None of these options: {:?}",event)}
            }
        }
    }
}

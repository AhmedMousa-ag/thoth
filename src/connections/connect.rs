use std::error::Error;

use crate::connections::{
    config::create_fucking_swarm,
    topics::{get_topic, get_topics},
    types::{GossipBehaviour, GossipBehaviourEvent, Messages},
};
use futures::stream::StreamExt;
use libp2p::{
    gossipsub::{self},
    mdns,
    swarm::{self, Swarm, SwarmEvent},
};

use super::types::EncodingDecoding;
use tokio::select;
use tokio::sync::mpsc::{self, Sender, Receiver};
// use std::sync::mpsc::{ Sender, Receiver};
use lazy_static::lazy_static;
use std::sync::RwLock;


lazy_static! {
    static ref CHANNEL: ( RwLock<Sender<Messages>>,  RwLock<Receiver<Messages>>) = {
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
pub async fn p2pconnect<'a>() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut swarm: Swarm<GossipBehaviour> = create_fucking_swarm();
    // Create a Gossipsub topics
    for topic in get_topics().iter() {
        // subscribes to our topic
        swarm.behaviour_mut().gossipsub.subscribe(topic)?;
    }

    // Listen on all interfaces and whatever port the OS assigns
    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;


    listen_messages(&mut swarm).await
}

pub async fn send_messages(message: Messages) {
    let tx = get_sender_tx().write().unwrap();
    if let Err(e)=tx.send(message).await{println!("Error sending message: {:?}",e);};
}
async fn recieve_messages() -> Option<Messages> {
     let mut rx = get_reciver_rx().write().unwrap(); 
    rx.recv().await
}

async fn listen_messages<'a>(
    swarm: &'a mut swarm::Swarm<GossipBehaviour>,
    // rx: &mut Receiver<Messages>,
) -> ! {
    loop {
        select! {
            (rec_message)=recieve_messages()=>{
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
                }
                _ => {println!("None of these options: {:?}",event)}
            }
        }
    }
}

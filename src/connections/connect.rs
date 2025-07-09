use crate::{
    connections::{
        channels_node_info::NodeInfoTrait,
        configs::{
            config::get_config,
            topics::{TopicsEnums, get_topic, get_topics},
        },
        types::{GossipBehaviour, GossipBehaviourEvent},
    },
    err, info,
    router::{
        post_offices::{
            external_com_ch::ExternalComm,
            nodes_info::post_office::{
                NodesInfoOffice, OperationStepExecuter, OperationsExecuterOffice,
            },
        },
        traits::PostOfficeTrait,
    },
    structs::{
        structs::{Message, NodeInfo, RequestsTypes},
        traits::EncodingDecoding,
    },
    warn,
};
use futures::{FutureExt, stream::StreamExt};
use libp2p::{
    Swarm, gossipsub, mdns, noise, ping,
    swarm::{self, SwarmEvent},
    tcp, yamux,
};
use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    hash::{Hash, Hasher},
    time::Duration,
};
use tokio::{io, select};
use tracing_subscriber::EnvFilter;

pub struct GossibConnection {}
impl GossibConnection {
    pub async fn p2pconnect() -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("Will start p2p connection now");
        let mut swarm: Swarm<GossipBehaviour> = Self::create_gossip_swarm();
        // Create a Gossipsub topics
        for topic in get_topics().iter() {
            // subscribes to our topic
            swarm.behaviour_mut().gossipsub.subscribe(topic)?;
        }

        // Listen on all interfaces and whatever port the OS assigns
        // swarm.listen_on(format!("/ip4/0.0.0.0/udp/{}/quic-v1", port).parse()?)?;
        swarm.listen_on(format!("/ip4/0.0.0.0/tcp/{}", get_config().port).parse()?)?;
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

    async fn listen_messages(swarm: &mut swarm::Swarm<GossipBehaviour>) -> ! {
        let ops_topic = TopicsEnums::OPERATIONS.as_str();
        let node_topic = TopicsEnums::NodesInfo.as_str();
        loop {
            select! {
                            rec_message=ExternalComm::recieve_messages()=>{
                                if rec_message.is_some() {
                                    let message = rec_message.unwrap();
                                    match get_topic(message.topic_name.as_str()){
                                        Some(topic) =>{
                                        if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic.clone(), message.encode_bytes()){
                                            err!("Error publishing message: {:?}",e);
                                        };}
                                        None=>{warn!("Didn't find topic: {}",message.topic_name.as_str())}
                                    }
                                }
                            },

                            event = swarm.select_next_some() => match event {
                                SwarmEvent::Behaviour(GossipBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                                    for (peer_id, _multiaddr) in list {
                                        info!("mDNS discovered a new peer: {peer_id}");
                                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                                    }
                                },
                                SwarmEvent::Behaviour(GossipBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                                    for (peer_id, _multiaddr) in list {
                                        info!("mDNS discover peer has expired: {}",peer_id);
                                        swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                                    }
                                },
                                SwarmEvent::Behaviour(GossipBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                                    propagation_source: peer_id,
                                    message_id: id,
                                    message,
                                })) => {info!(
                                        "Got message:  with id: {} from peer: {}",
                                        id,peer_id
                                    );
                                    let topic_name=message.topic.as_str();
                                    match topic_name{
                                        ops_topic if ops_topic==topic_name=>{
                                            info!("Got Operation Topic: {}",ops_topic); //message.data
                                            let ops_msg:Message=Message::decode_bytes(&message.data);
                                            match ops_msg.request {
                                                RequestsTypes::PlansToExecute=>{OperationStepExecuter::handle_incom_msg(ops_msg.message);},
                                                RequestsTypes::StartExecutePlan | RequestsTypes::EndedExecutingPlan=>{OperationsExecuterOffice::handle_incom_msg(ops_msg.message);},

                                                _=>warn!("Got operation topic message with no Request Type")

                                            };

                                    },
                                        node_topic if node_topic==topic_name=>{
                                            info!("Got node info exchange Topic: {}",node_topic);//message.data
                                            let ops_msg:Message=Message::decode_bytes(&message.data);
                                            NodesInfoOffice::handle_incom_msg(ops_msg.message);
                                        },
                                        _=>{
                                            warn!("Couldn't find the topic type");
                                        }
                                    };
                                    //TODO When a node keeps pinging their resources or something.
            },
                                SwarmEvent::NewListenAddr { address, .. } => {
                                    info!("Local node is listening on {}",address);
                                },
                                SwarmEvent::ConnectionEstablished{peer_id, connection_id,num_established,..}=>{
                                    info!("Established Connection id: {}, peer id: {}, number of established: {}",connection_id,peer_id ,num_established);
                                    NodeInfo::request_other_nodes_info();
                                },
                                SwarmEvent::ConnectionClosed{peer_id, connection_id,num_established,cause,..}=>{
                                    //When a node is not connected, remove it.
                                    NodeInfo::remove_node(peer_id.to_string());
                                    warn!("Connection Closed id: {}, peer id: {}, number of established: {},  due to: {:?}",connection_id,peer_id ,num_established,cause)
                                },
                                SwarmEvent::IncomingConnection{connection_id,local_addr,send_back_addr}=>{
                                    info!("Incomming connection id: {}, local address: {} send back address: {}",connection_id,local_addr,send_back_addr)
                                },
                                SwarmEvent::IncomingConnectionError{connection_id,error,..}=>{
                                    warn!("Incoming Connection Error on id: {} and the error: {}",connection_id,error)
                            },
                                _ => {warn!("None of these options: {:?}",event)}
                            }
                        }
        }
    }
}

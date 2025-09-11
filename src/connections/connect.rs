use crate::{
    connections::{
        channels_node_info::{NodeInfoTrait, get_current_node_cloned},
        configs::{
            config::get_config,
            topics::{TopicsEnums, get_topic, get_topics},
        },
        types::{GossipBehaviour, GossipBehaviourEvent},
    },
    err,
    errors::thot_errors::ThothErrors,
    info,
    router::{
        post_offices::{
            external_com_ch::ExternalComm,
            nodes_info::post_office::{
                GathererOffice, NodesInfoOffice, OperationStepExecuter, OperationsExecuterOffice,
                SyncerOffice,
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
use futures::stream::StreamExt;
use libp2p::{
    Swarm, gossipsub, mdns, noise, ping,
    swarm::{self, SwarmEvent},
    tcp, yamux,
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};
use tokio::{io, select};
use tracing_subscriber::EnvFilter;

pub struct GossibConnection {}
impl GossibConnection {
    pub async fn subscribe_topics(
        swarm: &mut swarm::Swarm<GossipBehaviour>,
    ) -> &mut swarm::Swarm<GossipBehaviour> {
        for topic in get_topics() {
            // subscribes to our topic
            if let Err(e) = swarm.behaviour_mut().gossipsub.subscribe(topic) {
                err!(
                    "Couldn't subscribe to topic {} due to {}",
                    topic,
                    ThothErrors::from(e)
                );
            }
            info!("Subscribed to topic: {}", topic);
        }
        swarm
    }
    pub async fn p2pconnect() -> Result<(), ThothErrors> {
        info!("Will start p2p connection now");
        let mut swarm: Swarm<GossipBehaviour> = Self::create_gossip_swarm();
        // Create a Gossipsub topics
        GossibConnection::subscribe_topics(&mut swarm).await;

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
                    .message_id_fn(message_id_fn)
                    .max_transmit_size(get_config().max_msg_size) // content-address messages. No two messages of the same content will be propagated.
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
        let ops_topic = TopicsEnums::Operations.as_str();
        let node_topic = TopicsEnums::NodesInfo.as_str();
        let sync_topic = TopicsEnums::Sync.as_str();
        loop {
            select! {
                            rec_message=ExternalComm::recieve_messages()=>{
                                if rec_message.is_some() {
                                    let message = rec_message.unwrap();
                                    match get_topic(message.topic_name.as_str()){
                                        Some(topic) =>{
                                            info!("Will send a message to topic: {}",topic);
                                        if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic.clone(), message.encode_bytes()){
                                            err!("Error publishing message: {:?} to topic: {}",e,topic);
                                        };}
                                        None=>{warn!("Didn't find topic: {}",message.topic_name.as_str())}
                                    }
                                }
                            },

                            event = swarm.select_next_some() => match event {
                                SwarmEvent::Behaviour(GossipBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                                    for (peer_id, _multiaddr) in list {
                                        info!("mDNS discovered a new peer: {}", peer_id);
                                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                                    }
                                },
                                SwarmEvent::Behaviour(GossipBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                                    for (peer_id, _multiaddr) in list {
                                        info!("mDNS discover peer has expired: {}",peer_id);
                                        swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                                    }
                                },
                                SwarmEvent::Behaviour(GossipBehaviourEvent::Gossipsub(gossipsub::Event::Subscribed { peer_id, topic })) => {
                                    info!("Peer {} subscribed to topic {}", peer_id, topic);
                                    // After subscribing, send our node info to other nodes. NOTE: putting in established connection event is useless because the subscription must be done first which doesn't happen yet there.
                                    NodesInfoOffice::send_message(Box::new(get_current_node_cloned()));
                                    NodeInfo::request_other_nodes_info();

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
                                    let decoded_msg:Message=Message::decode_bytes(&message.data);
                                    if topic_name==ops_topic{
                                        info!("Got Operation Topic: {}",ops_topic);
                                            match decoded_msg.request { //TODO refactor.
                                                RequestsTypes::PlansToExecute=>{OperationStepExecuter::handle_incom_msg(decoded_msg.message).await;},
                                                RequestsTypes::StartExecutePlan | RequestsTypes::EndedExecutingPlan=>{OperationsExecuterOffice::handle_incom_msg(decoded_msg.message).await;},
                                                RequestsTypes::RequestGatherPlans=>{GathererOffice::handle_reply_gather_res(decoded_msg.message);},
                                                RequestsTypes::ReplyGatherPlansRes=>{GathererOffice::handle_incom_msg(decoded_msg.message).await;},
                                                _=>warn!("Got operation topic message with no Request Type")

                                            };
                                    }else if topic_name==node_topic{
                                        info!("Got node info exchange Topic: {}",node_topic);//message.data
                                            if decoded_msg.request==RequestsTypes::RequestNodeInfo{
                                                NodesInfoOffice::send_message(Box::new(get_current_node_cloned()));
                                            }else if decoded_msg.request==RequestsTypes::ReplyNodeInfoUpdate{
                                                NodesInfoOffice::handle_incom_msg(decoded_msg.message).await;
                                            }else{
                                                warn!("Node Info request type couldn't be identified.")
                                            }
                                    }else if topic_name==sync_topic{
                                        info!("Got a sync message {}",topic_name);
                                        SyncerOffice::handle_incom_msg(decoded_msg.message).await;
                                    }else{
                                        warn!("Couldn't find the topic type");
                                    }

                                    //TODO When a node keeps pinging their resources or something.
            },
                                SwarmEvent::NewListenAddr { address, .. } => {
                                    info!("Local node is listening on {}",address);
                                },
                                SwarmEvent::ConnectionEstablished{peer_id, connection_id,num_established,..}=>{
                                    info!("Established Connection id: {}, peer id: {}, number of established: {}",connection_id,peer_id ,num_established);
                                    // Wait one second to allow the connection to be fully established before sending the message.
                                    // tokio::time::sleep(Duration::from_secs(1)).await;

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
                            // SwarmEvent::Behaviour(GossipBehaviourEvent {peer_id, topic}){},
                            // SwarmEvent::ExternalAddrConfirmed{address}=>{info!("External Address Confirmed: {}",address);NodesInfoOffice::send_message(Box::new(get_current_node_cloned()));
                            //         NodeInfo::request_other_nodes_info();},
                                _ => {}//warn!("None of these options: {:?}",event)}
                            }
                        }
        }
    }
}

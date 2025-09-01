use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    connections::{
        channels_node_info::{NodeInfoTrait, get_current_node_cloned},
        configs::topics::TopicsEnums,
    },
    err, info,
    operations::{
        executer::types::Executer,
        gatherer::{
            channels::get_opened_ch_sender as get_gatherer_ch_sender,
            structs::{GatheredMessage, Gatherer},
        },
        planner::charts::structs::{NodesOpsMsg, Steps},
    },
    router::{post_offices::external_com_ch::ExternalComm, traits::PostOfficeTrait},
    structs::{
        structs::{Message, NodeInfo, RequestsTypes},
        traits::EncodingDecoding,
    },
    warn,
};
use tokio::spawn;
pub struct NodesInfoOffice {}
pub struct OperationsExecuterOffice {}
pub struct OperationStepExecuter {}
pub struct GathererOffice {}

pub struct SyncerOffice {}

impl PostOfficeTrait<Box<NodeInfo>> for NodesInfoOffice {
    fn send_message(message: Box<NodeInfo>) {
        let rep_message = Box::new(Message {
            topic_name: TopicsEnums::NodesInfo.to_string(),
            request: RequestsTypes::ReplyNodeInfoUpdate,
            message: Some(message.encode_bytes()),
        });

        ExternalComm::send_message(Box::clone(&rep_message));
        info!("Sent message in Nodes Office.");
    }
    async fn handle_incom_msg(message: Option<Vec<u8>>) {
        spawn(async {
            let msg = NodeInfo::decode_bytes(&message.unwrap());
            NodeInfo::add_node(&msg);
        });
    }
}

impl PostOfficeTrait<Arc<RwLock<Steps>>> for OperationStepExecuter {
    fn send_message(msg: Arc<RwLock<Steps>>) {
        spawn(async move {
            let nodes_msg = Box::new(Message {
                topic_name: TopicsEnums::Operations.to_string(),
                request: RequestsTypes::PlansToExecute,
                message: Some(msg.read().await.encode_bytes()),
            });
            ExternalComm::send_message(nodes_msg);
            info!("Sent step to be executed.");
        });
    }

    async fn handle_incom_msg(message: Option<Vec<u8>>) {
        spawn(async {
            info!("Received step to be executed.");
            let step = Arc::new(RwLock::new(Steps::decode_bytes(&message.unwrap())));
            // DbOpsRegisterer::new_step(step.clone(), false).await;
            let mut executer = Executer {
                // op_file_manager: OperationsFileManager::new(&step.read().await.operation_id),
            };
            executer.execute_step(step).await;
        });
    }
}

impl PostOfficeTrait<Box<NodesOpsMsg>> for OperationsExecuterOffice {
    fn send_message(msg: Box<NodesOpsMsg>) {
        let nodes_msg = Box::new(Message {
            topic_name: TopicsEnums::Operations.to_string(),
            request: RequestsTypes::StartExecutePlan,
            message: Some(msg.encode_bytes()),
        });
        ExternalComm::send_message(nodes_msg);
        info!("Sent plans to be executed.")
    }
    async fn handle_incom_msg(message: Option<Vec<u8>>) {
        spawn(async {
            let duties = Box::new(NodesOpsMsg::decode_bytes(&message.unwrap()));
            let node_key = get_current_node_cloned().id;
            let operation_info = duties.nodes_duties.get(&node_key);
            if let Some(_) = operation_info {
                // let op_id = op_info[0].operation_id.clone();
                Executer {
                    // op_file_manager: OperationsFileManager::new(&op_id),
                }
                .execute_duties(duties)
                .await;
            }
        });
    }
}

impl PostOfficeTrait<GatheredMessage> for GathererOffice {
    fn send_message(msg: GatheredMessage) {
        spawn(async move {
            let nodes_msg = Box::new(Message {
                topic_name: TopicsEnums::Operations.to_string(),
                request: RequestsTypes::RequestGatherPlans,
                message: Some(msg.encode_bytes()),
            });
            ExternalComm::send_message(nodes_msg);
            info!("Sent Gathered Requests to be executed.");
        });
    }
    async fn handle_incom_msg(message: Option<Vec<u8>>) {
        spawn(async {
            let gathered_reply: GatheredMessage = GatheredMessage::decode_bytes(&message.unwrap());
            let msg_sender = match get_gatherer_ch_sender(&gathered_reply.operation_id).await {
                Some(sender) => sender,
                None => {
                    warn!("Tried to get gatherer sende channel but not found.");
                    return;
                }
            };
            match msg_sender.send(gathered_reply) {
                Ok(_) => {}
                Err(e) => err!("Error sending message of Gatherer Office: {}", e),
            };
        });
    }
}

impl GathererOffice {
    pub fn handle_reply_gather_res(message: Option<Vec<u8>>) {
        spawn(async {
            let gathered_msg: GatheredMessage = GatheredMessage::decode_bytes(&message.unwrap());
            reply_gather_res(gathered_msg);
        });
    }
}

pub fn reply_gather_res(gathered_msg: GatheredMessage) {
    let res = match Gatherer::reply_gathered_msg(gathered_msg) {
        Some(res) => {
            match res.respond {
                Some(ref r) => {
                    if r.result.is_none() {
                        warn!("No result in the gathered response, will not reply.");
                        return;
                    }
                }
                None => {
                    return;
                }
            };
            res
        }
        None => return,
    };
    let nodes_msg = Box::new(Message {
        topic_name: TopicsEnums::Operations.to_string(),
        request: RequestsTypes::ReplyGatherPlansRes,
        message: Some(res.encode_bytes()),
    });
    ExternalComm::send_message(nodes_msg);
    info!("Sent Gathered replies to other nodes.");
}

// impl PostOfficeTrait<SyncMessage> for SyncerOffice {
//     fn send_message(message: SyncMessage) {
//         let rep_message = Box::new(Message {
//             topic_name: TopicsEnums::Sync.to_string(),
//             request: message.message_type.clone(),
//             message: Some(message.encode_bytes()),
//         });

//         ExternalComm::send_message(Box::clone(&rep_message));
//         info!("Sent message in Nodes Office.");
//     }
//     async fn handle_incom_msg(message: Option<Vec<u8>>) {
//         spawn(async move {
//             let message = SyncMessage::decode_bytes(&message.unwrap());
//             if let Err(e) = get_sender().send(message) {
//                 err!("Sending SyncMessage Channel: {}", ThothErrors::from(e));
//             };
//         });
//     }
// }

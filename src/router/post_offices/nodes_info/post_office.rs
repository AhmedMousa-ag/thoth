use std::sync::{Arc, RwLock};

use crate::{
    connections::{
        channels_node_info::{NodeInfoTrait, get_current_node_cloned},
        configs::topics::TopicsEnums,
    },
    debug, info,
    logger::writters::writter::OperationsFileManager,
    operations::{
        executer::types::Executer,
        gatherer::{
            channels::get_opened_ch_sender,
            structs::{GatheredMessage, Gatherer},
        },
        planner::charts::structs::{NodesOpsMsg, Steps},
    },
    router::{post_offices::external_com_ch::ExternalComm, traits::PostOfficeTrait},
    structs::{
        structs::{Message, NodeInfo, RequestsTypes},
        traits::EncodingDecoding,
    },
};
use tokio::spawn;
pub struct NodesInfoOffice {}
pub struct OperationsExecuterOffice {}
pub struct OperationStepExecuter {}

pub struct GathererOffice {}

impl PostOfficeTrait<Box<NodeInfo>> for NodesInfoOffice {
    fn send_message(message: Box<NodeInfo>) {
        let rep_message = Box::new(Message {
            topic_name: TopicsEnums::NodesInfo.to_string(),
            request: RequestsTypes::ReplyNodeInfoUpdate,
            message: Some(message.encode_bytes()),
        });

        debug!("Will send nodes info to other nodes: {:?}", rep_message);
        ExternalComm::send_message(Box::clone(&rep_message));
        info!("Sent message in Nodes Office.");
    }
    fn handle_incom_msg(message: Option<Vec<u8>>) {
        spawn(async {
            let msg = NodeInfo::decode_bytes(&message.unwrap());
            NodeInfo::add_node(&msg);
            // TODO you might use this code to trigger event to start the planned operations
            // InternalCommunications::get_sender_tx()
            //     .lock()
            //     .await
            //     .send(Box::new(Message {
            //         topic_name: TopicsEnums::NodesInfo.to_string(),
            //         request: RequestsTypes::ReplyNodeInfoUpdate,
            //         message: Some(NodeInfo::encode_bytes(&message)),
            //     }))
            //     .await.unwrap();
        });
    }
}

impl PostOfficeTrait<Arc<RwLock<Steps>>> for OperationStepExecuter {
    fn send_message(msg: Arc<RwLock<Steps>>) {
        let nodes_msg = Box::new(Message {
            topic_name: TopicsEnums::OPERATIONS.to_string(),
            request: RequestsTypes::PlansToExecute,
            message: Some(msg.try_read().unwrap().encode_bytes()),
        });
        ExternalComm::send_message(nodes_msg);
        info!("Sent step to be executed.")
    }
    fn handle_incom_msg(message: Option<Vec<u8>>) {
        spawn(async {
            let step = Arc::new(RwLock::new(Steps::decode_bytes(&message.unwrap())));
            let mut executer = Executer {
                op_file_manager: OperationsFileManager::new(
                    step.try_read().unwrap().operation_id.clone(),
                )
                .unwrap(),
            };
            executer.execute_step(step);
        });
    }
}

impl PostOfficeTrait<Box<NodesOpsMsg>> for OperationsExecuterOffice {
    fn send_message(msg: Box<NodesOpsMsg>) {
        let nodes_msg = Box::new(Message {
            topic_name: TopicsEnums::OPERATIONS.to_string(),
            request: RequestsTypes::StartExecutePlan,
            message: Some(msg.encode_bytes()),
        });
        ExternalComm::send_message(nodes_msg);
        info!("Sent plans to be executed.")
    }
    fn handle_incom_msg(message: Option<Vec<u8>>) {
        spawn(async {
            let duties = Box::new(NodesOpsMsg::decode_bytes(&message.unwrap()));
            let node_key = get_current_node_cloned().id;
            let operation_info = duties.nodes_duties.get(&node_key);
            if let Some(op_info) = operation_info {
                let op_id = op_info.try_read().unwrap()[0].operation_id.clone();
                Executer {
                    op_file_manager: OperationsFileManager::new(op_id).unwrap(),
                }
                .execute_duties(duties);
            }
        });
    }
}

impl PostOfficeTrait<GatheredMessage> for GathererOffice {
    fn send_message(msg: GatheredMessage) {
        spawn(async move {
            let nodes_msg = Box::new(Message {
                topic_name: TopicsEnums::OPERATIONS.to_string(),
                request: RequestsTypes::RequestGatherPlans,
                message: Some(msg.encode_bytes()),
            });
            ExternalComm::send_message(nodes_msg);
            info!("Sent Gathered Requests to be executed.");
        });
    }
    fn handle_incom_msg(message: Option<Vec<u8>>) {
        spawn(async {
            let gathered_reply: GatheredMessage = GatheredMessage::decode_bytes(&message.unwrap());
            let msg_sender = match get_opened_ch_sender(&gathered_reply.operation_id) {
                Some(sender) => sender,
                None => return,
            };
            msg_sender.send(gathered_reply);
        });
    }
}

impl GathererOffice {
    pub fn handle_reply_gather_res(message: Option<Vec<u8>>) {
        spawn(async {
            let gathered_msg: GatheredMessage = GatheredMessage::decode_bytes(&message.unwrap());
            let res = match Gatherer::reply_gathered_msg(gathered_msg) {
                Some(res) => res,
                None => return,
            };
            let nodes_msg = Box::new(Message {
                topic_name: TopicsEnums::OPERATIONS.to_string(),
                request: RequestsTypes::ReplyGatherPlansRes,
                message: Some(res.encode_bytes()),
            });
            ExternalComm::send_message(nodes_msg);
            info!("Sent Gathered replies to other nodes.");
        });
    }
}

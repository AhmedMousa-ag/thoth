use std::{cell::RefCell, rc::Rc};

use crate::{
    connections::{channels_node_info::NodeInfoTrait, configs::topics::TopicsEnums},
    info,
    operations::planner::charts::structs::{SNodesOpsMsg, Steps},
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

impl PostOfficeTrait<Rc<RefCell<Steps>>> for OperationStepExecuter {
    fn send_message(msg: Rc<RefCell<Steps>>) {
        let nodes_msg = Box::new(Message {
            topic_name: TopicsEnums::OPERATIONS.to_string(),
            request: RequestsTypes::PlansToExecute,
            message: Some(msg.borrow().encode_bytes()),
        });
        ExternalComm::send_message(nodes_msg);
    }
    fn handle_incom_msg(message: Option<Vec<u8>>) {
        spawn(async {});
    }
}

impl PostOfficeTrait<Box<SNodesOpsMsg>> for OperationsExecuterOffice {
    fn send_message(msg: Box<SNodesOpsMsg>) {
        let nodes_msg = Box::new(Message {
            topic_name: TopicsEnums::OPERATIONS.to_string(),
            request: RequestsTypes::StartExecutePlan,
            message: Some(msg.encode_bytes()),
        });
        ExternalComm::send_message(nodes_msg);
        info!("Sent plans to be executed.")
    }
    fn handle_incom_msg(message: Option<Vec<u8>>) {
        spawn(async {});
    }
}

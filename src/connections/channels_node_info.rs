use std::collections::HashMap;

use crate::connections::configs::topics::TopicsEnums;
use crate::router::post_offices::external_com_ch::{ExternalComm, NodesMessage};
use crate::structs::structs::{Message, NodeInfo, RequestsTypes};
use lazy_static::lazy_static;
use sysinfo::System;
use tokio::runtime::Handle;
use tokio::sync::RwLock;
use tokio::task::block_in_place;
use uuid::Uuid;

lazy_static! {
    static ref NODES_INFO: RwLock<HashMap<String, NodeInfo>> = HashMap::new().into();
    static ref CURR_NODE_INFO: RwLock<NodeInfo> = RwLock::new(NodeInfo::new());
}

pub trait NodeInfoTrait {
    fn add_node(node: &NodeInfo) {
        block_in_place(|| {
            Handle::current().block_on(async {
                NODES_INFO
                    .write()
                    .await
                    .insert(node.id.clone(), node.clone());
            })
        });
    }
    fn new() -> NodeInfo {
        let id = Uuid::new_v4().to_string();
        let (av_threads, av_ram) = Self::calc_node_info();
        let node = NodeInfo {
            id,
            av_threads,
            av_ram,
        };
        Self::add_node(&node);
        node
    }

    fn remove_node(node_id: String) {
        block_in_place(|| {
            Handle::current().block_on(async { NODES_INFO.write().await.remove(&node_id) })
        });
    }
    fn calc_node_info() -> (usize, u64) {
        let mut sys = System::new_all();
        sys.refresh_all();
        let available_threads = sys.cpus().len();
        let available_memory_kb = sys.available_memory();
        (available_threads, available_memory_kb)
    }
    fn update_current_node_info() -> NodeInfo;
    fn request_other_nodes_info() {
        ExternalComm::send_message(Box::new(Message {
            topic_name: TopicsEnums::NodesInfo.to_string(),
            request: RequestsTypes::RequestNodeInfo,
            message: None,
        }));
    }
}

impl NodeInfoTrait for NodeInfo {
    fn update_current_node_info() -> NodeInfo {
        let (av_threads, av_ram) = Self::calc_node_info();
        let id = get_current_node().id;
        let updated_node = NodeInfo {
            id,
            av_threads,
            av_ram,
        };
        Self::add_node(&updated_node);
        updated_node
    }
}

pub fn get_current_node() -> NodeInfo {
    block_in_place(|| Handle::current().block_on(async { CURR_NODE_INFO.read().await.clone() }))
}
pub async fn get_nodes_info() -> HashMap<std::string::String, NodeInfo> {
    NODES_INFO.read().await.clone()
}

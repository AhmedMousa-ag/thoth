use crate::connections::configs::topics::TopicsEnums;
use crate::info;
use crate::router::post_offices::external_com_ch::ExternalComm;
use crate::structs::structs::{Message, NodeInfo, RequestsTypes};
use std::collections::HashMap;
use std::sync::OnceLock;
use sysinfo::System;
use tokio::runtime::Handle;
use tokio::sync::RwLock;
use tokio::task::block_in_place;
use uuid::Uuid;

static CURRENT_NODE: OnceLock<RwLock<NodeInfo>> = OnceLock::new();
static NODES_INFO: OnceLock<RwLock<HashMap<String, NodeInfo>>> = OnceLock::new();

pub trait NodeInfoTrait {
    fn add_node(node: &NodeInfo) {
        block_in_place(|| {
            Handle::current().block_on(async {
                get_nodes_info()
                    .write()
                    .await
                    .insert(node.id.clone(), node.clone());
            })
        });
        info!("Added new node: {}", node);
    }
    fn new() -> NodeInfo {
        let id = Uuid::new_v4().to_string();
        let (av_threads, av_ram) = Self::calc_node_info();
        let node = NodeInfo {
            id,
            av_threads,
            av_ram,
        };
        // Self::add_node(&node);
        node
    }

    fn remove_node(node_id: String) {
        block_in_place(|| {
            Handle::current().block_on(async { get_nodes_info().write().await.remove(&node_id) })
        });
        info!("Removed Node: {}", node_id);
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
        info!("Requsted Other Nodes Info");
        ExternalComm::send_message(Box::new(Message {
            topic_name: TopicsEnums::NodesInfo.to_string(),
            request: RequestsTypes::RequestNodeInfo,
            message: None,
        }));
    }
}

impl NodeInfoTrait for NodeInfo {
    fn update_current_node_info() -> NodeInfo {
        info!("Will update current node info");
        let (av_threads, av_ram) = Self::calc_node_info();
        let id = get_current_node_cloned().id;
        let updated_node = NodeInfo {
            id,
            av_threads,
            av_ram,
        };
        Self::add_node(&updated_node);
        updated_node
    }
}

pub fn get_current_node() -> &'static RwLock<NodeInfo> {
    CURRENT_NODE.get_or_init(|| RwLock::new(NodeInfo::new()))
}
pub fn get_nodes_info_cloned() -> HashMap<String, NodeInfo> {
    block_in_place(|| Handle::current().block_on(async { get_nodes_info().read().await.clone() }))
}
pub fn get_current_node_cloned() -> NodeInfo {
    block_in_place(|| Handle::current().block_on(async { get_current_node().read().await.clone() }))
}
pub fn get_nodes_info() -> &'static RwLock<HashMap<String, NodeInfo>> {
    NODES_INFO.get_or_init(|| {
        let curr_node = block_in_place(|| {
            Handle::current().block_on(async { get_current_node().read().await.clone() })
        });

        let mut all_nodes: HashMap<String, NodeInfo> = HashMap::new();
        all_nodes.insert(curr_node.id.clone(), curr_node.clone());
        all_nodes.into()
    })
}

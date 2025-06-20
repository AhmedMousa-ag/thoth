use std::collections::HashMap;

use super::types::NodeInfo;
use lazy_static::lazy_static;
use sysinfo::System;
use tokio::runtime::Handle;
use tokio::sync::RwLock;
use tokio::task::block_in_place;
lazy_static! {
    static ref NODES_INFO: RwLock<HashMap<String, NodeInfo>> = HashMap::new().into();
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
    fn new(id: String, av_threads: usize, av_ram: u64) -> NodeInfo {
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
}
impl NodeInfoTrait for NodeInfo {}

pub async fn get_nodes_info() -> HashMap<std::string::String, NodeInfo> {
    NODES_INFO.read().await.clone()
}

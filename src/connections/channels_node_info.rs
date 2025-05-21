use lazy_static::lazy_static;
use tokio::sync::RwLock;
// use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::runtime::Runtime;
#[derive(Clone, PartialEq, Debug)]
pub struct NodeInfo {
    pub id: String,
    pub ip: String,
    pub av_threads: i32,
    pub av_ram: i64, //MB
}

lazy_static! {
    static ref NODES_INFO: RwLock<Vec<NodeInfo>> = Vec::new().into();
}

pub trait NodeInfoTrait {
    fn add_node(node: &NodeInfo) {
        let rt = Runtime::new().unwrap();
        rt.block_on(async { NODES_INFO.write().await.push(node.clone()) })
    }
    fn new(id: String, ip: String, av_threads: i32, av_ram: i64) -> NodeInfo {
        let node = NodeInfo {
            id,
            ip,
            av_threads,
            av_ram,
        };
        Self::add_node(&node);
        node
    }
    fn destruct_me(&self);

    fn remove_node(node_to_remove: &NodeInfo) {
        let rt = Runtime::new().unwrap();
        let node_pos = rt.block_on(async {
            NODES_INFO
                .read()
                .await
                .iter()
                .position(|n| n == node_to_remove)
        });
        if let Some(pos) = node_pos {
            rt.block_on(async { NODES_INFO.write().await.remove(pos) });
        }
    }
}

impl NodeInfoTrait for NodeInfo {
    fn destruct_me(&self) {
        let rt = Runtime::new().unwrap();
        let node_pos =
            rt.block_on(async { NODES_INFO.read().await.iter().position(|n| n == self) });

        if let Some(pos) = node_pos {
            rt.block_on(async { NODES_INFO.write().await.remove(pos) });
        }
    }
}

pub async fn get_nodes_info() -> Vec<NodeInfo> {
    NODES_INFO.read().await.clone()
}

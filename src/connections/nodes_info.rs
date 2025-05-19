use lazy_static::lazy_static;
use tokio::sync::RwLock;
// use tokio::sync::mpsc::{self, Receiver, Sender};

#[derive(Clone)]
pub struct NodeInfo{
    pub id:String,
    pub ip:String,
    pub av_threads: i32,
    pub av_ram:i64, //MB
}
lazy_static! {
    static ref NODES_INFO: RwLock<Vec<NodeInfo>> = {
        Vec::new().into()
    };
}


pub async fn add_node(node:NodeInfo){
    NODES_INFO.write().await.push(node);
}


pub async fn get_nodes_info()->Vec<NodeInfo>{
    NODES_INFO.read().await.clone()
}
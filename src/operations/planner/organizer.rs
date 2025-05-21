use super::RequestOp;

// Send the plan to all nodes with unique id for each node role, each node must reply to the same message 


struct NodesInfo{
    id: String,
    av_threads:i32,
    av_ram_mb:i32,
}
struct Organizer{
    connected_nodes:Vec<>,
    requested_op:RequestOp
}

trait DisributeOperations{
    fn get_available_nodes(&self)->Vec<NodesInfo>;
    fn calc_distributed_operations(&self);
    fn send_distributed_messages(&self);//It should return results.
}
use crate::{
    connections::channels_node_info::NodeInfoTrait,
    info,
    router::{
        post_offices::nodes_info::{channel::InternalCommunications, post_office::NodesInfoOffice},
        traits::{PostOfficeTrait, SenderReciverTrait},
    },
    structs::structs::{NodeInfo, RequestsTypes},
    warn,
};
use tokio::spawn;
pub fn start_back_office() {
    // Watch for internal communication requests
    spawn(async {
        info!("Started Communications back office");
        loop {
            if let Some(message) = InternalCommunications::get_reciver_rx()
                .lock()
                .await
                .recv()
                .await
            {
                match message.request {
                    RequestsTypes::RequestNodeInfo => {
                        info!("Got request type: {:?}", RequestsTypes::RequestNodeInfo);
                        let nodes_info = NodeInfo::update_current_node_info();
                        NodesInfoOffice::send_message(Box::new(nodes_info));
                        info!("{:?}", message);
                    }
                    _ => warn!(
                        "Backoffice request type none of the above, it's: {:?}",
                        message.request
                    ),
                }
            }
        }
    });
}

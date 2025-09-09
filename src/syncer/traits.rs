use chrono::Utc;
use tokio::{spawn, sync::RwLock};

use crate::{
    connections::{
        channels_node_info::{get_current_node_cloned, get_nodes_info_cloned},
        configs::topics::TopicsEnums,
    },
    db::controller::{registerer::DbOpsRegisterer, traits::SqlSyncedOps},
    info,
    router::{
        post_offices::{external_com_ch::ExternalComm, nodes_info::post_office::SyncerOffice},
        traits::PostOfficeTrait,
    },
    structs::{
        structs::{Message, RequestsTypes},
        traits::EncodingDecoding,
    },
    syncer::{
        channels::get_reciever,
        configs::get_config,
        structs::{OperationType, SyncMessage, SyncOperations, Syncer},
    },
    utils::util::convert_string_datetime,
    warn,
};
use std::time::Duration;
use std::{sync::Arc, thread};
impl Syncer {
    pub fn new() -> &'static Self {
        &Self { is_syncing: false }
    }
    pub fn run(&'static self) {
        self.perform_sync_ops();
        self.trigger_sync();
    }
    fn trigger_sync(&'static self) {
        spawn(async {
            let config = get_config();
            loop {
                thread::sleep(Duration::from_secs(config.sleep_time_min * 60)); //Convert to seconds.
                info!("Triggering Syncer Process");
                let num_nodes = get_nodes_info_cloned().len();
                let quorum = config.quorum;
                if self.is_syncing || num_nodes < quorum {
                    warn!(
                        "Skipping Syncer Process due to: is_syncing: {} or nodes count: {} < quorum: {}",
                        self.is_syncing, num_nodes, quorum
                    );
                    continue;
                };
                let target_nodes: Vec<String> = get_nodes_info_cloned()
                    .iter()
                    .map(|node| node.1.id.clone())
                    .collect::<Vec<String>>()[0..quorum]
                    .to_vec();
                // TODO define nodes to sync with based on the quorum.
                let start_date: Option<String>;
                let end_date: String = Utc::now().to_string();
                match SqlSyncedOps::get_latest_finished() {
                    Some(last) => start_date = Some(last.to_date.to_string()),
                    None => start_date = None,
                }
                let sync_ops = SyncOperations {
                    start_date: start_date.clone(),
                    end_date: end_date.clone(),
                    operation: None,
                };
                let sync_msg = SyncMessage {
                    message_type: RequestsTypes::RequestSyncing,
                    message: sync_ops,
                    target_nodes: Some(target_nodes),
                };
                DbOpsRegisterer::new_syncer(
                    convert_string_datetime(start_date),
                    convert_string_datetime(Some(end_date)),
                    true,
                );
                SyncerOffice::send_message(sync_msg);
            }
        });
    }

    fn reply_request(&'static self, message: SyncOperations, target_nodes: Option<Vec<String>>) {
        spawn(async move {
            let (start_date, end_date) = (
                convert_string_datetime(message.start_date),
                convert_string_datetime(Some(message.end_date)),
            );

            DbOpsRegisterer::new_syncer(start_date, end_date, true);
            let mut reply_operations: Vec<OperationType> = Vec::new();

            let db_ops = DbOpsRegisterer::get_operation_by_date(Some(start_date), Some(end_date));
            //TODO make it lazy instead of sending everything at once.
            for op in db_ops {
                for stp in DbOpsRegisterer::get_steps_by_op_id(&op.operation_id) {
                    let step = DbOpsRegisterer::get_step_file(&op.operation_id, &stp.step_id);
                    if step.is_none() {
                        warn!(
                            "Step file not found for operation_id: {} step_id: {} syncer reply",
                            &op.operation_id, &stp.step_id
                        );
                        continue;
                    }
                    let step = step.unwrap();
                    let op_type = OperationType::Step(step);
                    reply_operations.push(op_type);
                }
            }
            let sync_message = SyncMessage {
                message_type: RequestsTypes::ReplySyncing,
                message: SyncOperations {
                    start_date: Some(start_date.to_string()),
                    end_date: end_date.to_string(),
                    operation: Some(reply_operations),
                },
                target_nodes: target_nodes.clone(),
            };
            let sync_message = sync_message.encode_bytes();
            let sync_message = Message {
                topic_name: TopicsEnums::Sync.to_string(),
                message: Some(sync_message),
                request: RequestsTypes::ReplySyncing,
            };
            ExternalComm::send_message(Box::new(sync_message));
        });
    }
    fn perform_register(&'static self, message: SyncOperations) {
        spawn(async {
            let operations = message.operation;
            if operations.is_none() {
                warn!("No operations found in the sync message to register.");
                return;
            }
            let operations = operations.unwrap();
            for op in operations {
                match op {
                    OperationType::Step(step) => {
                        if DbOpsRegisterer::get_step_file(&step.operation_id, &step.step_id)
                            .is_some()
                        {
                            //Already have it.
                            continue;
                        }
                        DbOpsRegisterer::new_step(Arc::new(RwLock::new(step)), true).await;
                    }
                    OperationType::NodesDuties(nodes_duties) => {
                        for (node_id, duties) in nodes_duties {
                            for duty in duties {
                                if DbOpsRegisterer::get_duty_by_step_id(&duty.operation_id)
                                    .is_some()
                                {
                                    //Already have it.
                                    continue;
                                }
                                DbOpsRegisterer::new_duty(
                                    node_id.clone(),
                                    duty.operation_id,
                                    duty.step_id,
                                    true,
                                );
                            }
                        }
                    }
                }
            }
        });
    }

    fn perform_sync_ops(&'static self) {
        let mut reciever = get_reciever().try_lock().unwrap();
        spawn(async move {
            loop {
                match reciever.recv().await {
                    Some(sync_op) => {
                        let curr_node_id = get_current_node_cloned().id;
                        if sync_op.target_nodes.is_some()
                            && !sync_op
                                .target_nodes
                                .as_ref()
                                .unwrap()
                                .contains(&curr_node_id)
                        {
                            //Not for this node.
                            continue;
                        }

                        match sync_op.message_type {
                            RequestsTypes::RequestSyncing => {
                                self.reply_request(sync_op.message, sync_op.target_nodes);
                            }
                            RequestsTypes::ReplySyncing => self.perform_register(sync_op.message),
                            RequestsTypes::RegisterSyncing => {} //TODO consider removing it.
                            _ => {
                                warn!(
                                    "Got invalid Syncing Type Message of: {}",
                                    sync_op.message_type
                                )
                            }
                        }
                    }
                    None => {
                        // Channel closed
                        break;
                    } // Trigger message.
                      // DbOpsRegisterer::new_syncer(date_from, date_to);
                      // Every node sends what they have.
                      // Every node recieves each others operations counts.
                      // Compare with the QUROM configs.
                      // Decide to eaither register it or not.
                      // Mark synced as completed.
                }
            }
        });
    }
}

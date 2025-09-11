use chrono::Utc;
use tokio::{spawn, sync::RwLock, task::block_in_place};

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
        channels::{get_reciever, get_sender},
        configs::get_config,
        structs::{OperationType, SyncMessage, SyncOperations, Syncer},
    },
    utils::util::convert_string_datetime,
    warn,
};
use std::time::Duration;
use std::{sync::Arc, thread};

lazy_static::lazy_static!(
    static ref IS_SYNCING: RwLock<bool> = RwLock::new(false);
);

async fn flip_syncing_state() {
    let is_syncing = IS_SYNCING.read().await.clone();
    *IS_SYNCING.write().await = !is_syncing;
}

fn _get_sync_state() -> bool {
    block_in_place(|| futures::executor::block_on(async { IS_SYNCING.read().await.clone() }))
}

impl Syncer {
    pub fn new() -> &'static Self {
        &Self {}
    }
    pub fn run(&'static self) {
        info!("Starting Syncer Process in the Background....");
        self.perform_sync_ops();
        self.trigger_sync();
    }
    fn trigger_sync(&'static self) {
        spawn(async {
            let config = get_config();
            loop {
                //TODO consider syncing at certain times of the day only which should be low traffic times such as middle of the night.
                thread::sleep(Duration::from_secs(config.sleep_time_min * 60)); //Convert to seconds.
                info!("Triggering Syncer Process");
                let num_nodes = get_nodes_info_cloned().len();
                let quorum = config.quorum;
                // let is_syncing = get_sync_state();
                // TODO if is_syncing continue;
                if num_nodes < quorum {
                    warn!(
                        "Skipping Syncer Process due to nodes count: {} < quorum: {}",
                        num_nodes, quorum
                    );
                    continue;
                };
                flip_syncing_state().await;
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
                    target_nodes: Some(target_nodes.clone()),
                };
                let (date_from, date_to) = (
                    convert_string_datetime(start_date.clone()),
                    convert_string_datetime(Some(end_date.clone())),
                );
                if DbOpsRegisterer::get_syncer_ops(date_from, date_to).is_none() {
                    DbOpsRegisterer::new_syncer(date_from, date_to, false);
                };
                SyncerOffice::send_message(sync_msg);

                //Sync this node operations to all other nodes as well. Triggering this sender, reciever channel internally.
                request_sync_internally(target_nodes, start_date, end_date);
            }
        });
    }

    fn reply_request(&'static self, message: SyncOperations, target_nodes: Option<Vec<String>>) {
        spawn(async move {
            info!("Replying to Sync Request");
            flip_syncing_state().await;
            let (start_date, end_date) = (
                convert_string_datetime(message.start_date),
                convert_string_datetime(Some(message.end_date)),
            );

            if DbOpsRegisterer::get_syncer_ops(start_date, end_date).is_none() {
                DbOpsRegisterer::new_syncer(start_date, end_date, false);
            }
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
        info!("Performing Registering Sync Operations");
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
                            info!(
                                "Already have step file for operation_id: {} step_id: {}",
                                &step.operation_id, &step.step_id
                            );
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
                                    info!(
                                        "Already have duty for operation_id: {} step_id: {}",
                                        &duty.operation_id, &duty.step_id
                                    );
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
            flip_syncing_state().await;
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

fn request_sync_internally(
    target_nodes: Vec<String>,
    start_date: Option<String>,
    end_date: String,
) {
    info!("Requesting Sync Internally");
    let sync_ops = SyncOperations {
        start_date: start_date,
        end_date: end_date,
        operation: None,
    };
    let sync_msg = SyncMessage {
        message_type: RequestsTypes::RequestSyncing,
        message: sync_ops,
        target_nodes: Some(target_nodes),
    };
    if let Err(e) = get_sender().send(sync_msg) {
        warn!("Error sending sync message to channel: {:?}", e);
    };
}

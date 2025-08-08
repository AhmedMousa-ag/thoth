use chrono::Utc;
use tokio::spawn;

use crate::{
    db::controller::{
        registerer::DbOpsRegisterer,
        traits::{SqlOperations, SqlSteps, SqlSyncedOps},
    },
    router::{post_offices::nodes_info::post_office::SyncerOffice, traits::PostOfficeTrait},
    structs::structs::RequestsTypes,
    syncer::{
        channels::get_reciever,
        configs::get_config,
        structs::{OperationType, SyncMessage, SyncOperations, Syncer},
    },
    utils::util::convert_string_datetime,
    warn,
};
use std::thread;
use std::time::Duration;
impl Syncer {
    pub fn new() -> &'static Self {
        &Self { is_syncing: false }
    }
    fn trigger_sync(&'static self) {
        spawn(async {
            loop {
                thread::sleep(Duration::from_secs(get_config().sleep_time_min * 60)); //Convert to seconds.
                if self.is_syncing {
                    continue;
                };

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
                };
                DbOpsRegisterer::new_syncer(
                    convert_string_datetime(start_date),
                    convert_string_datetime(Some(end_date)),
                );
                SyncerOffice::send_message(sync_msg);
            }
        });
    }

    fn reply_request(&'static self, message: SyncOperations) {
        spawn(async {
            let (start_date, end_date) = (
                convert_string_datetime(message.start_date),
                convert_string_datetime(Some(message.end_date)),
            );
            DbOpsRegisterer::new_syncer(start_date, end_date);
            let _reply_operations: Vec<OperationType> = Vec::new();
            let db_ops = SqlOperations::get_by_date(start_date, end_date);
            //TODO make it lazy instead of sending everything at once.
            for op in db_ops {
                let steps = SqlSteps::get_by_op_id(op.op_id);
                for _step in steps {
                    // let step_msg = OperationType::Step();
                    // reply_operations.push(step_msg);
                }
            }
        });
    }
    fn perform_register(&'static self, _message: SyncOperations) {
        spawn(async {});
    }
    fn perform_sync_ops(&'static self) {
        let mut reciever = get_reciever().try_lock().unwrap();
        spawn(async move {
            loop {
                match reciever.recv().await {
                    Some(sync_op) => {
                        match sync_op.message_type {
                            RequestsTypes::RequestSyncing => {
                                self.reply_request(sync_op.message);
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
    pub fn back_office(&'static self) {
        //trigger in a thread every couple of hours.
        self.trigger_sync();
        self.perform_sync_ops();
    }
}

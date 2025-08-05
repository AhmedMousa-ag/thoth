use chrono::Utc;
use tokio::spawn;

use crate::{
    db::controller::traits::SqlSyncedOps,
    router::{post_offices::nodes_info::post_office::SyncerOffice, traits::PostOfficeTrait},
    structs::structs::RequestsTypes,
    syncer::{
        configs::get_config,
        structs::{SyncMessage, SyncOperations, Syncer},
    },
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
                    start_date,
                    end_date,
                    operation: None,
                };
                let sync_msg = SyncMessage {
                    message_type: RequestsTypes::RequestSyncing,
                    message: sync_ops,
                };
                SyncerOffice::send_message(sync_msg);
            }
        });
    }
    pub fn back_office(&'static self) {
        //trigger in a thread every couple of hours.
        self.trigger_sync();
        // SyncerOffice::send_message(message);
        // DbOpsRegisterer::new_syncer(date_from, date_to);
    }
}

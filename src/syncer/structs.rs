/*

Run a back office to recieve any requests to sync.

Create Configurations of the QUORUM values.

Every 12 hours run sync.

Create a command line to trigger it and change the configurations if possible.
Register last time a sync did happen.
Get all database records since that register happened.
The triggerer node keeps track of how many replication compared to the quorum.
Compare and send message to the nodes that are missing the data.

*/

use std::str::FromStr;

use crate::{
    err,
    operations::planner::charts::structs::{NodesDuties, Steps},
    structs::structs::RequestsTypes,
};
use bincode::{Decode, Encode};
use chrono::{DateTime, Utc};
#[derive(Debug)]
pub struct Syncer {
    pub is_syncing: bool,
}

#[derive(Encode, Decode)]
pub struct SyncMessage {
    pub message_type: RequestsTypes,
    pub message: SyncOperations,
    pub target_nodes: Option<Vec<String>>, //If None, broadcast to all nodes.
}

#[derive(Encode, Debug, Decode)] //TODO impl display
pub enum OperationType {
    Step(Steps),
    NodesDuties(NodesDuties),
}

impl OperationType {
    pub fn to_string(&self) -> String {
        match self {
            OperationType::Step(_) => format!("{:?}", self),
            OperationType::NodesDuties(_) => format!("{:?}", self),
        }
    }
    ///Don't use if your type isn't steps.
    pub fn get_step_value(&self) -> &Steps {
        match self {
            OperationType::Step(val) => val,
            _ => {
                let msg = "Expected Step value.";
                err!("{}", msg);
                unreachable!();
            }
        }
    }

    ///Don't use if your type isn't steps.
    pub fn get_nodes_duties_value(&self) -> &NodesDuties {
        match self {
            OperationType::NodesDuties(val) => val,
            _ => {
                let msg = "Expected Nodes Duties value.";
                err!("{}", msg);
                unreachable!();
            }
        }
    }
}

#[derive(Encode, Decode)]
pub struct SyncOperations {
    pub operation: Option<Vec<OperationType>>,
    pub start_date: Option<String>,
    pub end_date: String,
}

impl SyncOperations {
    pub fn date_to_string(date: DateTime<Utc>) -> String {
        date.to_string()
    }
    pub fn string_to_date(date: &str) -> DateTime<Utc> {
        let result: DateTime<Utc> = DateTime::from_str(date).unwrap();
        result
    }
}

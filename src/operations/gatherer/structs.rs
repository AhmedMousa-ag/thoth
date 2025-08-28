use bincode::{Decode, Encode};
// use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedReceiver;

use crate::{operations::planner::charts::structs::ExtraInfo, structs::numerics::structs::Numeric};
#[derive(Debug)]
pub struct Gatherer {
    pub reciever_ch: UnboundedReceiver<GatheredMessage>,
}
#[derive(Debug, Encode, Decode, Clone)]
pub struct GatheredResponse {
    pub result: Option<Numeric>,
    pub use_prev_res: bool,
    pub extra_info: Option<ExtraInfo>,
}
#[derive(Debug, Encode, Decode, Clone)]
pub struct GatheredMessage {
    pub operation_id: String,
    pub step_id: String,
    pub respond: Option<GatheredResponse>,
}

use crate::operations::planner::charts::structs::Numeric;
#[derive(Debug)]
pub struct Gatherer {}
#[derive(Debug)]
pub struct GatheredMessage {
    pub operation_id: String,
    pub step_id: String,
    pub respond: Option<Numeric>,
}

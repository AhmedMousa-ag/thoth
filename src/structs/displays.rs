use super::structs::{Message, NodeInfo, RequestsTypes};
use std::fmt;
impl fmt::Display for NodeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(
            f,
            "Id: {}, Available Threads: {}, Available Ram: {}",
            self.id, self.av_threads, self.av_ram
        )
    }
}

impl fmt::Display for RequestsTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Request Type: {}, ", self.as_str())
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Topic Name: {}, Request Type: {}",
            self.topic_name, self.request
        )
    }
}

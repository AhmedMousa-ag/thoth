use chrono::{DateTime, Utc};
use std::fs::File;

#[derive(Clone)]
pub enum FileTypes {
    INFO,
    DEBUG,
    WARN,
    ERROR,
    OPERATIONS,
}

impl FileTypes {
    pub fn as_str(&self) -> &str {
        match self {
            FileTypes::INFO => "INFO",
            FileTypes::DEBUG => "DEBUG",
            FileTypes::WARN => "WARN",
            FileTypes::ERROR => "ERROR",
            FileTypes::OPERATIONS => "OPERATIONS",
        }
    }
}
pub struct LogFileManager {
    pub start_time: DateTime<Utc>,
    pub max_lines: i64,
    pub current_lint: i64,
    pub file_type: FileTypes,
    pub file: File,
}

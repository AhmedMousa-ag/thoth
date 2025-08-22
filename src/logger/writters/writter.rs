use chrono::{DateTime, Utc};
use std::{collections::HashMap, fs::File, sync::Arc};
use tokio::sync::Mutex;

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
            FileTypes::OPERATIONS => "OPERATIONS", //TODO maybe you should re-think it
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
pub struct OperationsFileManager {
    pub op_id: String,
    pub files: HashMap<String, Arc<Mutex<File>>>,
}

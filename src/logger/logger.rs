use crate::info;

use super::writters::writter::{FileTypes, LogFileManager};
use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::Mutex;

lazy_static! {
    static ref INFO_WRITER: Arc<Mutex<LogFileManager>> =
        Arc::new(Mutex::new(LogFileManager::new(FileTypes::INFO).unwrap()));
    static ref DEBUG_WRITER: Arc<Mutex<LogFileManager>> =
        Arc::new(Mutex::new(LogFileManager::new(FileTypes::DEBUG).unwrap()));
    static ref WARNING_WRITER: Arc<Mutex<LogFileManager>> =
        Arc::new(Mutex::new(LogFileManager::new(FileTypes::WARN).unwrap()));
    static ref ERROR_WRITER: Arc<Mutex<LogFileManager>> =
        Arc::new(Mutex::new(LogFileManager::new(FileTypes::ERROR).unwrap()));
}

pub struct LoggerWritter;

impl LoggerWritter {
    pub async fn start() {
        // Start all log file managers
        LogFileManager::start(INFO_WRITER.clone()).await;
        LogFileManager::start(DEBUG_WRITER.clone()).await;
        LogFileManager::start(WARNING_WRITER.clone()).await;
        LogFileManager::start(ERROR_WRITER.clone()).await;
        info!("All log writers started successfully!");
    }
}

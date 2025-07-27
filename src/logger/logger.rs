use super::writters::writter::{FileTypes, LogFileManager};
use crate::logger::writters::traits::FileManagerTrait;
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
        LogFileManager::start_reciever(INFO_WRITER.clone()).await;
        LogFileManager::start_reciever(DEBUG_WRITER.clone()).await;
        LogFileManager::start_reciever(WARNING_WRITER.clone()).await;
        LogFileManager::start_reciever(ERROR_WRITER.clone()).await;
    }
}

use crate::logger::channels::{
    get_debug_reciever, get_err_reciever, get_info_reciever, get_warn_reciever,
};
use chrono::{DateTime, Utc};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, prelude::*},
    os::unix::fs::FileExt,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{spawn, sync::Mutex};

#[derive(Clone)]
pub enum FileTypes {
    INFO,
    DEBUG,
    WARN,
    ERROR,
}

impl FileTypes {
    pub fn as_str(&self) -> &str {
        match self {
            FileTypes::INFO => "INFO",
            FileTypes::DEBUG => "DEBUG",
            FileTypes::WARN => "WARN",
            FileTypes::ERROR => "ERROR",
        }
    }
}
pub struct LogFileManager {
    start_time: DateTime<Utc>,
    max_lines: i64,
    current_lint: i64,
    file_type: FileTypes,
    file: File,
}

impl LogFileManager {
    pub fn new(file_type: FileTypes) -> Result<Self, io::Error> {
        let start_time = Utc::now();
        let file_path = Self::generate_file_name(start_time, &file_type);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path)?;
        Ok(Self {
            start_time: start_time,
            max_lines: 1_000_000, //TODO you can create configurations for it.
            current_lint: 0,
            file_type: file_type,
            file: file,
        })
    }
    fn generate_file_name(start_time: DateTime<Utc>, file_type: &FileTypes) -> PathBuf {
        Path::new("logs")
            .join(file_type.as_str())
            .join(start_time.to_string())
    }
    pub fn read(&mut self, max_lines: u64) -> Result<String, io::Error> {
        let mut contents = vec![];
        self.file.read_at(&mut contents, max_lines)?;
        let file_content = String::from_utf8(contents).unwrap_or_default();
        Ok(file_content)
    }
    pub fn write(&mut self, line: String) -> Result<(), io::Error> {
        self.current_lint += 1;
        self.file.write_all(line.as_bytes())?;
        if self.is_file_limit() {
            self.new_file()?;
        };
        Ok(())
    }
    fn new_file(&mut self) -> Result<(), io::Error> {
        self.start_time = Utc::now();
        self.current_lint = 0;
        let file_path = Self::generate_file_name(self.start_time, &self.file_type);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        self.file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path)?;
        Ok(())
    }
    fn is_file_limit(&self) -> bool {
        self.current_lint >= self.max_lines
    }

    pub async fn start(manager: Arc<Mutex<Self>>) {
        let file_type = {
            let guard = manager.lock().await;
            guard.file_type.clone()
        };

        spawn(async move {
            let receiver = match file_type {
                FileTypes::INFO => get_info_reciever(),
                FileTypes::DEBUG => get_debug_reciever(),
                FileTypes::WARN => get_warn_reciever(),
                FileTypes::ERROR => get_err_reciever(),
            };

            loop {
                let msg = receiver.lock().await.recv().await; // Use async recv instead of blocking_recv
                match msg {
                    Some(line) => {
                        let mut guard = manager.lock().await;
                        if let Err(e) = guard.write(format!("{}\n", line)) {
                            eprintln!("\x1b[0m:\x1b[31m[ERROR IN LOGS WRITER]\x1b[0m: {}", e);
                        }
                    }
                    None => {
                        eprintln!(
                            "\x1b[0m:\x1b[31m[ERROR IN LOGS WRITER]\x1b[0m: Log receiver for {} disconnected. Stopping task.",
                            file_type.as_str()
                        );
                        break;
                    }
                }
            }
        });
    }
}

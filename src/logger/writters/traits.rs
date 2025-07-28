use crate::{
    debug,
    logger::{
        channels::{
            get_debug_reciever, get_err_reciever, get_info_reciever, get_ops_reciever,
            get_warn_reciever,
        },
        writters::writter::{FileTypes, LogFileManager, OperationsFileManager},
    },
    operations::planner::charts::structs::Steps,
    utils::util::create_directories,
};
use chrono::{DateTime, Utc};
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{self, prelude::*},
    os::unix::fs::FileExt,
    path::{Path, PathBuf},
    sync::{Arc, RwLock as StandardRwLock},
};
use tokio::runtime::Handle;
use tokio::task::block_in_place;
use tokio::{spawn, sync::Mutex};
pub trait FileManagerTrait {
    fn new(file_type: FileTypes) -> Result<Self, io::Error>
    where
        Self: Sized;
    fn generate_file_name(start_time: DateTime<Utc>, file_type: &FileTypes) -> PathBuf;
    fn read(&mut self, max_lines: u64) -> Result<String, io::Error>;
    fn write(&mut self, line: String) -> Result<(), io::Error>;
    fn new_file(&mut self) -> Result<(), io::Error>;
    fn start_reciever(manager: Arc<Mutex<Self>>) -> impl std::future::Future<Output = ()> + Send;
    fn is_file_limit(&self) -> bool;
}

impl FileManagerTrait for LogFileManager {
    fn new(file_type: FileTypes) -> Result<Self, io::Error> {
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
    fn read(&mut self, max_lines: u64) -> Result<String, io::Error> {
        let mut contents = vec![];
        self.file.read_at(&mut contents, max_lines)?;
        let file_content = String::from_utf8(contents).unwrap_or_default();
        Ok(file_content)
    }
    fn write(&mut self, line: String) -> Result<(), io::Error> {
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
    fn generate_file_name(start_time: DateTime<Utc>, file_type: &FileTypes) -> PathBuf {
        Path::new("logs")
            .join(file_type.as_str())
            .join(start_time.to_string())
    }
    fn is_file_limit(&self) -> bool {
        self.current_lint >= self.max_lines
    }
    async fn start_reciever(manager: Arc<Mutex<Self>>) {
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
                FileTypes::OPERATIONS => get_ops_reciever(),
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

impl OperationsFileManager {
    // It should handle operations folder, including each step file.
    pub fn new(op_id: String) -> Result<Self, io::Error> {
        let file_path = Self::generate_file_name(&op_id, "");
        create_directories(file_path.as_os_str().to_str().unwrap());
        Ok(Self {
            op_id,
            file_type: FileTypes::OPERATIONS,
            files: HashMap::new(),
        })
    }

    fn generate_file_name(op_id: &str, step_id: &str) -> PathBuf {
        Path::new("logs")
            .join(FileTypes::OPERATIONS.as_str())
            .join(op_id)
            .join(step_id)
    }

    fn open_file(&self, file_path: PathBuf) -> Result<File, io::Error> {
        debug!("Will open a file path: {:?}", file_path);

        Ok(OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path)?)
    }
    pub fn get_file(&mut self, step_id: &str, keep_file_open: bool) -> Arc<Mutex<File>> {
        if !self.files.contains_key(step_id) {
            return self.create_step_file(step_id, keep_file_open).unwrap();
        }
        self.files.get(step_id).unwrap().clone()
    }

    pub fn read(&mut self, step_id: &str, keep_file_open: bool) -> Result<String, io::Error> {
        let mut contents = vec![];
        block_in_place(|| {
            Handle::current().block_on(async {
                self.get_file(step_id, keep_file_open)
                    .lock()
                    .await
                    .read_to_end(&mut contents)?;
                let file_content = String::from_utf8(contents).unwrap_or_default();
                Ok(file_content)
            })
        })
    }
    pub fn write(
        &mut self,
        step: Arc<StandardRwLock<Steps>>,
        keep_file_open: bool,
    ) -> Result<(), io::Error> {
        block_in_place(|| {
            Handle::current().block_on(async {
                let lines = serde_json::to_string(&step).unwrap();
                self.get_file(&step.try_read().unwrap().step_id, keep_file_open)
                    .lock()
                    .await
                    .write_all(lines.as_bytes())?;
                Ok(())
            })
        })
    }

    pub fn create_step_file(
        &mut self,
        step_id: &str,
        keep_file_open: bool,
    ) -> Result<Arc<Mutex<File>>, io::Error> {
        let file = Arc::new(Mutex::new(
            self.open_file(Self::generate_file_name(&self.op_id, step_id))?,
        ));
        if keep_file_open {
            self.files.insert(step_id.to_string(), file);
            return Ok(self.files.get(step_id).unwrap().clone());
        }
        Ok(file.clone())
    }
    pub fn load_step_file(op_id: &str, step_id: &str) -> Result<Steps, io::Error> {
        let file_path = Self::generate_file_name(op_id, step_id);
        let mut file = OpenOptions::new().read(true).open(file_path)?;
        let mut contents = vec![];
        file.read_to_end(&mut contents)?;
        let file_content = String::from_utf8(contents).unwrap_or_default();

        Ok(serde_json::from_str(&file_content)?)
    }
}

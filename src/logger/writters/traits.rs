use crate::{
    debug, err,
    errors::thot_errors::ThothErrors,
    logger::{
        channels::{
            get_debug_reciever, get_err_reciever, get_info_reciever, get_ops_reciever,
            get_warn_reciever,
        },
        writters::{
            configs::OPERATIONS_LOCATIONS,
            utils::{get_files_by_date, pathbuf_str, sort_files_and_persist},
            writter::{FileTypes, LogFileManager, OperationsFileManager},
        },
    },
    operations::planner::charts::structs::{OperationFile, Steps},
    warn,
};
use chrono::{DateTime, Utc};
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions, create_dir_all},
    io::{self, prelude::*},
    os::unix::fs::FileExt,
    path::{Path, PathBuf},
    sync::{Arc, RwLock as StandardRwLock},
};
use tokio::runtime::Handle;
use tokio::task::block_in_place;
use tokio::{spawn, sync::Mutex};

fn generate_file_path(paths: Vec<String>) -> Result<PathBuf, ThothErrors> {
    let mut file_path = PathBuf::new();

    for path in paths {
        file_path.push(path);
        if file_path.extension().is_none() {
            if let Err(e) = create_dir_all(&file_path) {
                warn!("Failed to create directory: {}", e);
            }
        }
    }

    Ok(file_path)
}

fn create_symbolic_link(target: &Path, link: &Path) -> Result<(), ThothErrors> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(target, link).map_err(|e| ThothErrors::from(e))
    }
    #[cfg(windows)]
    {
        std::os::windows::fs::symlink_file(target, link).map_err(|e| ThothErrors::from(e))
    }
    #[cfg(not(any(unix, windows)))]
    {
        Err(ThothErrors::from(io::Error::new(
            io::ErrorKind::Other,
            "Symbolic links are not supported on this platform",
        )))
    }
}

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
    pub fn new(op_id: &str) -> Self {
        generate_file_path(vec![OPERATIONS_LOCATIONS.to_string(), op_id.to_string()]).unwrap();
        Self {
            op_id: op_id.to_string(),
            files: HashMap::new(),
        }
    }
    pub fn get_operations_path(op_id: &str) -> Vec<String> {
        vec![OPERATIONS_LOCATIONS.to_string(), op_id.to_string()]
    }
    pub fn get_operations_main_path(op_id: &str) -> Vec<String> {
        let mut path = Self::get_operations_path(op_id);
        path.push("main_operation.th".to_string());
        path
    }
    pub fn get_step_path(op_id: &str, step_id: &str) -> Vec<String> {
        vec![
            OPERATIONS_LOCATIONS.to_string(),
            op_id.to_string(),
            format!("{}.th", step_id),
        ]
    }

    pub fn get_date_path() -> Vec<String> {
        vec![OPERATIONS_LOCATIONS.to_string(), "dates".to_string()]
    }

    pub fn get_operation_date_path(op_id: &str) -> Vec<String> {
        vec![
            OPERATIONS_LOCATIONS.to_string(),
            "dates".to_string(),
            format!("{}.th", op_id.to_string()),
        ]
    }
    pub fn get_step_date_path(op_id: &str, step_id: &str) -> Vec<String> {
        vec![
            OPERATIONS_LOCATIONS.to_string(),
            "dates".to_string(),
            op_id.to_string(),
            format!("{}.th", step_id),
        ]
    }

    fn open_file(&self, file_path: &PathBuf) -> Option<File> {
        match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path)
        {
            Ok(file) => Some(file),
            Err(e) => {
                err!("Failed to open file: {}, {}", file_path.display(), e);
                None
            }
        }
    }
    pub fn get_open_file(
        &mut self,
        file_path: &PathBuf,
        keep_file_open: bool,
    ) -> Option<Arc<Mutex<File>>> {
        let id = &pathbuf_str(file_path);
        if self.files.contains_key(id) {
            return Some(self.files.get(id).unwrap().clone());
        }
        debug!(
            "Key file doesn't exists will create one for path: {}",
            file_path.display()
        );
        let open_file = match self.open_file(file_path) {
            Some(file) => file,
            None => return None,
        };

        let file = Arc::new(Mutex::new(open_file));
        if keep_file_open {
            self.files.insert(id.to_string(), file.clone());
            return Some(self.files.get(id).unwrap().clone());
        } else {
            return Some(file);
        }
    }

    pub fn read(&mut self, id: &PathBuf, keep_file_open: bool) -> Result<String, io::Error> {
        let mut contents = vec![];
        block_in_place(|| {
            Handle::current().block_on(async {
                self.get_open_file(id, keep_file_open)
                    .unwrap()
                    .try_lock()
                    .unwrap()
                    .read_to_end(&mut contents)?;
                let file_content = String::from_utf8(contents).unwrap_or_default();
                Ok(file_content)
            })
        })
    }
    pub fn write_step(
        &mut self,
        step: Arc<StandardRwLock<Steps>>,
        keep_file_open: bool,
    ) -> Result<(), ThothErrors> {
        block_in_place(|| {
            Handle::current().block_on(async {
                let lines = serde_json::to_string(&step).unwrap();
                let op_id = step.try_read().unwrap().operation_id.clone();
                let step_id = step.try_read().unwrap().step_id.clone();
                let step_path = generate_file_path(Self::get_step_path(&op_id, &step_id)).unwrap();
                let step_date_path =
                    generate_file_path(Self::get_step_date_path(&op_id, &step_id)).unwrap();
                self.get_open_file(&step_path, keep_file_open)
                    .unwrap()
                    .lock()
                    .await
                    .write_all(lines.as_bytes())?;
                sort_files_and_persist(&pathbuf_str(&step_path), true);
                create_symbolic_link(&step_path, &step_date_path)?;
                sort_files_and_persist(&pathbuf_str(&step_date_path), true);
                Ok(())
            })
        })
    }

    pub fn write_operation_file(
        &mut self,
        operation_file: &OperationFile,
        keep_file_open: bool,
    ) -> Result<(), ThothErrors> {
        block_in_place(|| {
            Handle::current().block_on(async {
                let lines = serde_json::to_string(&operation_file).unwrap();
                let op_path =
                    generate_file_path(Self::get_operations_main_path(&self.op_id)).unwrap();
                let op_date_path =
                    generate_file_path(Self::get_operations_main_path(&self.op_id)).unwrap();
                debug!("Writing operation file at: {}", op_path.display());
                self.get_open_file(&op_path, keep_file_open)
                    .unwrap()
                    .lock()
                    .await
                    .write_all(lines.as_bytes())?;
                sort_files_and_persist(&pathbuf_str(&op_path), true);
                create_symbolic_link(&op_path, &op_date_path)?;
                sort_files_and_persist(&pathbuf_str(&op_date_path), true);
                Ok(())
            })
        })
    }

    pub fn create_step_file(
        &mut self,
        step_id: Arc<StandardRwLock<Steps>>,
        keep_file_open: bool,
    ) -> Result<Arc<Mutex<File>>, ThothErrors> {
        let step_id = step_id.read().unwrap().step_id.clone();
        let file_path = generate_file_path(Self::get_step_path(&self.op_id, &step_id))?;
        let file = Arc::new(Mutex::new(self.open_file(&file_path.clone()).unwrap()));
        sort_files_and_persist(&pathbuf_str(&file_path), true);
        if keep_file_open {
            self.files.insert(step_id.to_string(), file);
            return Ok(self.files.get(&step_id).unwrap().clone());
        }
        Ok(file.clone())
    }
    pub fn load_step_file(op_id: &str, step_id: &str) -> Result<Steps, ThothErrors> {
        let file_path = generate_file_path(Self::get_step_path(op_id, step_id))?;
        let mut file = OpenOptions::new().read(true).open(file_path)?;
        let mut contents = vec![];
        file.read_to_end(&mut contents)?;
        let file_content = String::from_utf8(contents).unwrap_or_default();

        Ok(serde_json::from_str(&file_content)?)
    }
    pub fn create_operation_file(
        &mut self,
        operation: OperationFile,
        keep_file_open: bool,
    ) -> Result<(), ThothErrors> {
        self.write_operation_file(&operation, keep_file_open)?;
        Ok(())
    }
    pub fn load_operation_file(operation_id: &str) -> Option<OperationFile> {
        let file_path = generate_file_path(Self::get_operations_main_path(operation_id)).ok()?;
        debug!(
            "Will load operation id: {}, at path: {}",
            operation_id,
            file_path.display()
        );
        let mut file = match OpenOptions::new().read(true).open(file_path) {
            Ok(file) => file,
            Err(_) => return None,
        };
        let mut contents = vec![];
        match file.read_to_end(&mut contents) {
            Ok(_) => (),
            Err(_) => return None,
        };
        let file_content = String::from_utf8(contents).unwrap_or_default();
        Some(serde_json::from_str(&file_content).ok()?)
    }
    fn list_directory_files(dir: &str) -> Vec<String> {
        let mut files = vec![];
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.filter_map(Result::ok) {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        files.push(entry.path().to_string_lossy().into_owned());
                    }
                }
            }
        }
        files
    }
    pub fn load_operations_by_date(
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
    ) -> Vec<OperationFile> {
        let mut operations = vec![];
        let mut files_locations_vec = Self::get_operation_date_path("");
        files_locations_vec.pop(); // Remove the last element
        let files_locations = pathbuf_str(&generate_file_path(files_locations_vec).unwrap());
        let files = Self::list_directory_files(&files_locations);
        get_files_by_date(&mut operations, &files, start_date, end_date);
        operations
    }
    pub fn load_steps_by_op_id(operation_id: &str) -> Vec<Steps> {
        let mut steps = vec![];
        let mut files_locations_vec = Self::get_step_date_path(operation_id, "");
        files_locations_vec.pop(); // Remove the last element
        let files_locations = pathbuf_str(&generate_file_path(files_locations_vec).unwrap());
        let files = Self::list_directory_files(&files_locations);
        for file in files {
            if let Ok(step) = Self::load_step_file(operation_id, &file) {
                steps.push(step);
            }
        }
        steps
    }
}

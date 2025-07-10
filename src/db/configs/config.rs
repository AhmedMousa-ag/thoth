use crate::utils::util::create_directories;
use std::{path::Path, sync::OnceLock};

pub struct Config {
    pub db_name: String,
    pub path: String,
    pub db_mod: String,
}
static CONFIGS: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    CONFIGS.get_or_init(|| {
        let os_path = String::from(Path::new("db").canonicalize().unwrap().to_str().unwrap()); //Absolute path

        create_directories(&os_path);

        Config {
            db_name: "thoth".to_string(),
            path: String::from(os_path),
            db_mod: "rwc".to_string(), // read-write-create mode
        }
    })
}

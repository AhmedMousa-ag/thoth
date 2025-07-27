use std::{fs, path::Path};

use crate::{err, info, warn};

pub fn create_directories(path: &str) {
    let path = Path::new(path);

    // Check if the directory already exists
    match path.is_dir() {
        true => {
            info!("Directory '{}' already exists.", path.display());
        }
        false => {
            // If it doesn't exist, try to create it and its parent directories
            warn!(
                "Directory '{}' does not exist. Attempting to create...",
                path.display()
            );
            match fs::create_dir_all(path) {
                Ok(_) => {
                    info!("Directory '{}' created successfully.", path.display());
                }
                Err(e) => {
                    err!("Failed to create directory '{}': {}", path.display(), e);
                }
            }
        }
    }
}

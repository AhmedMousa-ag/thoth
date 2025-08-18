use std::{fs, path::Path};

use chrono::{DateTime, TimeZone, Utc};

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

pub fn convert_string_datetime(date: Option<String>) -> DateTime<Utc> {
    let date: DateTime<Utc> = match date {
        Some(date) => date.parse().unwrap(),
        //Default to year 2025, you can actually make starting from today coding time sense this is the first time to use :P .
        None => Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
    };
    date
}


pub fn find_binary_search<T: Ord>(list: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = list.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if &list[mid] == target {
            return Some(mid);
        } else if &list[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}

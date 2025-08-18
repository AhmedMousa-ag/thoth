use crate::{operations::planner::charts::structs::OperationFile, utils::util::find_binary_search};
use chrono::{DateTime, Utc};
use std::{fs, io, path::PathBuf};
use tokio::spawn;

pub fn pathbuf_str(path: &PathBuf) -> String {
    path.to_string_lossy().into_owned()
}

pub fn sort_files_and_persist(dir: &str, thread: bool) {
    let dir = dir.to_string();
    let result = move || -> io::Result<()> {
        let files: Vec<PathBuf> = fs::read_dir(dir)?
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .collect();
        let mut sorted_files = files.clone();
        sorted_files.sort();
        // Divide and conquer: recursively sort and rename files in halves
        fn sort_and_rename(files: &[PathBuf], sorted_files: &[PathBuf]) -> io::Result<()> {
            if files.len() <= 1 {
                return Ok(());
            }
            let mid = files.len() / 2;
            sort_and_rename(&files[..mid], &sorted_files[..mid])?;
            sort_and_rename(&files[mid..], &sorted_files[mid..])?;

            for (i, file) in sorted_files.iter().enumerate() {
                if *file == files[i] {
                    continue;
                }
                fs::rename(&file, &file)?;
            }
            Ok(())
        }

        sort_and_rename(&files, &sorted_files)?;
        Ok(())
    };
    if thread {
        spawn(async move {
            if let Err(e) = result() {
                eprintln!("Error sorting files: {}", e);
            }
        });
    } else {
        if let Err(e) = result() {
            eprintln!("Error sorting files: {}", e);
        }
    }
}

pub fn get_files_by_date(
    operations: &mut Vec<OperationFile>,
    files: &Vec<String>,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
) {
    let end_idx = match end_date {
        Some(date) => find_binary_search(&files, &date.to_string()),
        None => None,
    };
    let start_idx = match start_date {
        Some(date) => find_binary_search(&files, &date.to_string()),
        None => None,
    };
    if let Some(end_idx) = end_idx {
        if let Some(start_idx) = start_idx {
            let new_files: Vec<String> = files[start_idx..end_idx].to_vec();
            let new_files: Vec<OperationFile> = new_files
                .into_iter()
                .map(|f| serde_json::from_str::<OperationFile>(&f).unwrap())
                .collect();
            operations.extend(new_files);
        } else {
            let new_files: Vec<String> = files[..end_idx].to_vec();
            let new_files: Vec<OperationFile> = new_files
                .into_iter()
                .map(|f| serde_json::from_str::<OperationFile>(&f).unwrap())
                .collect();
            operations.extend(new_files);
        }
    } else if let Some(start_idx) = start_idx {
        let new_files: Vec<String> = files[start_idx..].to_vec();
        let new_files: Vec<OperationFile> = new_files
            .into_iter()
            .map(|f| serde_json::from_str::<OperationFile>(&f).unwrap())
            .collect();
        operations.extend(new_files);
    }
}

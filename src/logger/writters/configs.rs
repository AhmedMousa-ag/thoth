use lazy_static::lazy_static;
use std::path::PathBuf;

use crate::logger::writters::utils::pathbuf_str;

lazy_static! {
    pub static ref OPERATIONS_LOCATIONS: String = {
        let mut path = PathBuf::new();
        path.push("logs");
        path.push("OPERATIONS");
        pathbuf_str(&path)
    };
}

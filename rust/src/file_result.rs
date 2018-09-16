use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::prelude::*;

#[derive(Debug, Ord, PartialOrd, Clone)]
pub struct FileResult{
    pub filepath:       String,
    pub hash:           String,
    pub size:           u64,
    pub time_modified:  SystemTime,
}

impl Eq for FileResult {}

impl PartialEq for FileResult {
    fn eq(&self, other: &FileResult) -> bool {
        self.hash == other.hash
        && self.size == other.size
        && self.time_modified.duration_since(other.time_modified).unwrap().as_secs() < 1
        && self.filepath == other.filepath
    }
}

impl fmt::Display for FileResult {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let epochtime = self.time_modified.duration_since(UNIX_EPOCH).unwrap().as_secs();

        return fmt.write_str(&format!("{} ({} | {} bytes)", 
            self.filepath, 
            Local.timestamp(epochtime as i64, 0).format("%Y-%m-%d %H:%M:%S").to_string(), 
            self.size));
    }
}
use std::collections::HashMap;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::prelude::*;

#[derive(Debug)]
pub struct FileResult{
    pub filepath:       String,
    pub hash:           String,
    pub size:           u64,
    pub time_modified:  SystemTime,
}

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

pub type ScanDirectoryResult = HashMap<String, FileResult>;
pub type PatchResult = HashMap<String, Vec<FileResult>>;
pub type ReconcileResult = (PatchResult, PatchResult);
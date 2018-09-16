use std::fmt;
use std::fmt::Write;
use std::collections::HashMap;

use file_result::FileResult;

#[derive(PartialEq, Eq, Hash,Clone, Copy)]
pub enum ReconcileOperation{
    Add,
    Unchanged,
    Conflict,
}

impl fmt::Display for ReconcileOperation {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        return match *self{
            ReconcileOperation::Add => fmt.write_char('+'),
            ReconcileOperation::Unchanged => fmt.write_char('='),
            ReconcileOperation::Conflict => fmt.write_char('!'),
        }
    }
}

pub type ScanDirectoryResult = HashMap<String, FileResult>;
pub type PatchResult = HashMap<ReconcileOperation, Vec<FileResult>>;
pub type ReconcileResult = (PatchResult, PatchResult);
pub type Line = (ReconcileOperation, FileResult);
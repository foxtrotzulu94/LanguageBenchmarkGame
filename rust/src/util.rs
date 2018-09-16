use std::fs;
use std::io;
use std::path::Path;

// From https://doc.rust-lang.org/std/fs/fn.read_dir.html
// one possible implementation of walking a directory only visiting files
pub fn visit_dirs(dir: &Path, cb: &mut FnMut(&fs::DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
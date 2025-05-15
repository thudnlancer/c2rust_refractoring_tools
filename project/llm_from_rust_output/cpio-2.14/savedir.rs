use std::cmp::Ordering;
use std::ffi::{CStr, CString, OsStr};
use std::fs::{read_dir, DirEntry, ReadDir};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub enum SavedirOption {
    None,
    SortName,
    SortInode,
}

impl SavedirOption {
    fn comparison_fn(self) -> Option<fn(&DirEntry, &DirEntry) -> Ordering> {
        match self {
            SavedirOption::None => None,
            SavedirOption::SortName => Some(Self::compare_name),
            SavedirOption::SortInode => Some(Self::compare_inode),
        }
    }

    fn compare_name(a: &DirEntry, b: &DirEntry) -> Ordering {
        a.file_name().cmp(&b.file_name())
    }

    fn compare_inode(a: &DirEntry, b: &DirEntry) -> Ordering {
        a.ino().cmp(&b.ino())
    }
}

trait DirEntryExt {
    fn ino(&self) -> u64;
}

impl DirEntryExt for DirEntry {
    fn ino(&self) -> u64 {
        self.metadata().unwrap().ino()
    }
}

pub fn savedir(dir: &Path, option: SavedirOption) -> io::Result<Vec<CString>> {
    let mut entries: Vec<DirEntry> = read_dir(dir)?.filter_map(|e| e.ok()).collect();

    if let Some(compare) = option.comparison_fn() {
        entries.sort_by(compare);
    }

    entries
        .into_iter()
        .map(|e| {
            CString::new(e.file_name().as_bytes()).map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, "Invalid filename encoding")
            })
        })
        .collect()
}

pub fn streamsavedir(dir: ReadDir, option: SavedirOption) -> io::Result<Vec<CString>> {
    let mut entries: Vec<DirEntry> = dir.filter_map(|e| e.ok()).collect();

    if let Some(compare) = option.comparison_fn() {
        entries.sort_by(compare);
    }

    entries
        .into_iter()
        .map(|e| {
            CString::new(e.file_name().as_bytes()).map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, "Invalid filename encoding")
            })
        })
        .collect()
}
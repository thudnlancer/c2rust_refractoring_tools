use std::cmp::Ordering;
use std::ffi::{CStr, CString, OsStr};
use std::fs::{read_dir, DirEntry, ReadDir};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

#[derive(Debug, Clone)]
struct DirEntryInfo {
    name: CString,
    ino: u64,
}

#[derive(Copy, Clone, PartialEq)]
pub enum SavedirOption {
    None,
    SortName,
    SortInode,
}

impl SavedirOption {
    fn comparison_fn(self) -> Option<fn(&DirEntryInfo, &DirEntryInfo) -> Ordering> {
        match self {
            SavedirOption::None => None,
            SavedirOption::SortName => Some(Self::compare_name),
            SavedirOption::SortInode => Some(Self::compare_inode),
        }
    }

    fn compare_name(a: &DirEntryInfo, b: &DirEntryInfo) -> Ordering {
        a.name.as_bytes().cmp(b.name.as_bytes())
    }

    fn compare_inode(a: &DirEntryInfo, b: &DirEntryInfo) -> Ordering {
        a.ino.cmp(&b.ino)
    }
}

fn is_dot_or_dotdot(entry: &OsStr) -> bool {
    let bytes = entry.as_bytes();
    bytes == b"." || bytes == b".."
}

pub fn savedir(dir: &Path, option: SavedirOption) -> io::Result<Vec<CString>> {
    let mut entries = Vec::new();
    let mut total_size = 0;

    for entry in read_dir(dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        
        if is_dot_or_dotdot(&file_name) {
            continue;
        }

        let name = CString::new(file_name.as_bytes())?;
        total_size += name.as_bytes().len() + 1; // +1 for null terminator

        if let Some(compare_fn) = option.comparison_fn() {
            let metadata = entry.metadata()?;
            entries.push(DirEntryInfo {
                name,
                ino: metadata.ino(),
            });
        } else {
            entries.push(DirEntryInfo {
                name,
                ino: 0, // Not used when not sorting by inode
            });
        }
    }

    if let Some(compare_fn) = option.comparison_fn() {
        entries.sort_by(compare_fn);
    }

    let mut result = Vec::with_capacity(total_size + 1);
    for entry in entries {
        result.push(entry.name);
    }

    Ok(result)
}

pub fn streamsavedir(dir: ReadDir, option: SavedirOption) -> io::Result<Vec<CString>> {
    let mut entries = Vec::new();
    let mut total_size = 0;

    for entry in dir {
        let entry = entry?;
        let file_name = entry.file_name();
        
        if is_dot_or_dotdot(&file_name) {
            continue;
        }

        let name = CString::new(file_name.as_bytes())?;
        total_size += name.as_bytes().len() + 1; // +1 for null terminator

        if let Some(compare_fn) = option.comparison_fn() {
            let metadata = entry.metadata()?;
            entries.push(DirEntryInfo {
                name,
                ino: metadata.ino(),
            });
        } else {
            entries.push(DirEntryInfo {
                name,
                ino: 0, // Not used when not sorting by inode
            });
        }
    }

    if let Some(compare_fn) = option.comparison_fn() {
        entries.sort_by(compare_fn);
    }

    let mut result = Vec::with_capacity(total_size + 1);
    for entry in entries {
        result.push(entry.name);
    }

    Ok(result)
}
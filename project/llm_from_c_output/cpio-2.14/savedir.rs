use std::cmp::Ordering;
use std::ffi::{CStr, CString, OsStr, OsString};
use std::fs::{read_dir, DirEntry, ReadDir};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SavedirOption {
    SortNone,
    SortName,
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    SortInode,
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    SortFastRead,
}

impl Default for SavedirOption {
    fn default() -> Self {
        SavedirOption::SortNone
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
impl SavedirOption {
    fn comparison_fn(&self) -> Option<fn(&DirEntry, &DirEntry) -> Ordering> {
        match self {
            SavedirOption::SortNone => None,
            SavedirOption::SortName => Some(Self::compare_name),
            SavedirOption::SortInode | SavedirOption::SortFastRead => Some(Self::compare_inode),
        }
    }

    fn compare_name(a: &DirEntry, b: &DirEntry) -> Ordering {
        a.file_name().cmp(&b.file_name())
    }

    fn compare_inode(a: &DirEntry, b: &DirEntry) -> Ordering {
        a.ino().cmp(&b.ino())
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
impl SavedirOption {
    fn comparison_fn(&self) -> Option<fn(&DirEntry, &DirEntry) -> Ordering> {
        match self {
            SavedirOption::SortNone => None,
            SavedirOption::SortName => Some(Self::compare_name),
            SavedirOption::SortFastRead => None,
        }
    }

    fn compare_name(a: &DirEntry, b: &DirEntry) -> Ordering {
        a.file_name().cmp(&b.file_name())
    }
}

pub fn savedir<P: AsRef<Path>>(dir: P, option: SavedirOption) -> io::Result<Vec<u8>> {
    let dir = dir.as_ref();
    let dir_entries = read_dir(dir)?;
    streamsavedir(dir_entries, option)
}

pub fn streamsavedir(dir_entries: ReadDir, option: SavedirOption) -> io::Result<Vec<u8>> {
    let mut entries = Vec::new();
    let mut result = Vec::new();

    for entry in dir_entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let name_bytes = file_name.as_bytes();

        // Skip ".", ".." and empty names
        if name_bytes.is_empty()
            || (name_bytes.len() == 1 && name_bytes[0] == b'.')
            || (name_bytes.len() == 2 && name_bytes[0] == b'.' && name_bytes[1] == b'.')
        {
            continue;
        }

        if let Some(cmp_fn) = option.comparison_fn() {
            entries.push(entry);
        } else {
            result.extend_from_slice(name_bytes);
            result.push(0);
        }
    }

    if let Some(cmp_fn) = option.comparison_fn() {
        entries.sort_by(cmp_fn);
        for entry in entries {
            let name_bytes = entry.file_name().as_bytes();
            result.extend_from_slice(name_bytes);
            result.push(0);
        }
    }

    result.push(0);
    Ok(result)
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
trait DirEntryExt {
    fn ino(&self) -> u64;
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
impl DirEntryExt for DirEntry {
    fn ino(&self) -> u64 {
        self.metadata().unwrap().ino()
    }
}
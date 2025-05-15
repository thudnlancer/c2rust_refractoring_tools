use std::ffi::{CStr, CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::cmp::Ordering;
use std::fs::{read_dir, DirEntry};
use std::io;

#[derive(Debug, Clone)]
pub enum SavedirOption {
    None,
    SortName,
}

impl SavedirOption {
    fn comparison_fn(&self) -> Option<fn(&DirEntry, &DirEntry) -> Ordering> {
        match self {
            SavedirOption::None => None,
            SavedirOption::SortName => Some(|a, b| {
                a.file_name().cmp(&b.file_name())
            }),
        }
    }
}

pub fn savedir(dir: &Path, option: SavedirOption) -> io::Result<Vec<u8>> {
    let mut entries: Vec<DirEntry> = read_dir(dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let name = entry.file_name();
            let name = name.as_bytes();
            
            match name {
                b"." | b".." => None,
                _ => Some(entry),
            }
        })
        .collect();

    if let Some(cmp) = option.comparison_fn() {
        entries.sort_by(cmp);
    }

    let mut result = Vec::new();
    for entry in entries {
        let name = entry.file_name();
        result.extend_from_slice(name.as_bytes());
        result.push(0);
    }

    Ok(result)
}

pub fn streamsavedir(dir: &Path, option: SavedirOption) -> io::Result<Vec<u8>> {
    savedir(dir, option)
}
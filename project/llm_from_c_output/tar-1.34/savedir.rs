use std::ffi::{CStr, CString};
use std::fs::{read_dir, DirEntry, ReadDir};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SavedirOption {
    SortNone,
    SortName,
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    SortInode,
    SortFastRead,
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
impl Default for SavedirOption {
    fn default() -> Self {
        SavedirOption::SortInode
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
impl Default for SavedirOption {
    fn default() -> Self {
        SavedirOption::SortNone
    }
}

#[derive(Debug)]
struct Direntry {
    name: Vec<u8>,
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    ino: u64,
}

impl Direntry {
    fn new(entry: &DirEntry) -> io::Result<Self> {
        let name = entry.file_name().as_bytes().to_vec();
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let ino = entry.ino()?;

        Ok(Direntry {
            name,
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            ino,
        })
    }
}

fn direntry_cmp_name(a: &Direntry, b: &Direntry) -> std::cmp::Ordering {
    a.name.cmp(&b.name)
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn direntry_cmp_inode(a: &Direntry, b: &Direntry) -> std::cmp::Ordering {
    a.ino.cmp(&b.ino)
}

pub fn streamsavedir(dirp: ReadDir, option: SavedirOption) -> io::Result<Vec<u8>> {
    let mut entries = Vec::new();
    let mut name_space = Vec::new();
    let mut used = 0;

    let cmp = match option {
        SavedirOption::SortNone => None,
        SavedirOption::SortName => Some(direntry_cmp_name as fn(&Direntry, &Direntry) -> _),
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        SavedirOption::SortInode => Some(direntry_cmp_inode as fn(&Direntry, &Direntry) -> _),
        SavedirOption::SortFastRead => {
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            {
                Some(direntry_cmp_inode as fn(&Direntry, &Direntry) -> _)
            }
            #[cfg(not(any(target_os = "linux", target_os = "macos")))]
            {
                None
            }
        }
    };

    for entry_result in dirp {
        let entry = entry_result?;
        let file_name = entry.file_name();
        let name_bytes = file_name.as_bytes();

        // Skip "", ".", and ".."
        if name_bytes.is_empty()
            || (name_bytes[0] == b'.' && 
                (name_bytes.len() == 1 || 
                 (name_bytes.len() == 2 && name_bytes[1] == b'.')))
        {
            continue;
        }

        let entry_size = name_bytes.len() + 1;
        
        if let Some(cmp_fn) = cmp {
            entries.push(Direntry::new(&entry)?);
        } else {
            if name_space.len() - used < entry_size {
                name_space.resize(used + entry_size, 0);
            }
            name_space[used..used + name_bytes.len()].copy_from_slice(name_bytes);
            name_space[used + name_bytes.len()] = 0;
        }
        used += entry_size;
    }

    if let Some(cmp_fn) = cmp {
        entries.sort_by(cmp_fn);
        name_space.clear();
        for entry in entries {
            name_space.extend_from_slice(&entry.name);
            name_space.push(0);
        }
    } else {
        name_space.truncate(used);
    }

    name_space.push(0);
    Ok(name_space)
}

pub fn savedir(dir: &Path, option: SavedirOption) -> io::Result<Vec<u8>> {
    let dirp = read_dir(dir)?;
    streamsavedir(dirp, option)
}
use std::cmp::Ordering;
use std::ffi::{CStr, CString, OsStr, OsString};
use std::fs::{read_dir, DirEntry, ReadDir};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SavedirOption {
    SortNone,
    SortName,
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    SortInode,
    SortFastRead,
}

impl Default for SavedirOption {
    fn default() -> Self {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            SavedirOption::SortInode
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            SavedirOption::SortNone
        }
    }
}

#[derive(Debug)]
struct Direntry {
    name: OsString,
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    ino: u64,
}

impl Direntry {
    fn new(entry: &DirEntry) -> io::Result<Self> {
        let name = entry.file_name();
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            let metadata = entry.metadata()?;
            let ino = metadata.ino();
            Ok(Direntry { name, ino })
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            Ok(Direntry { name })
        }
    }
}

fn direntry_cmp_name(a: &Direntry, b: &Direntry) -> Ordering {
    a.name.cmp(&b.name)
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn direntry_cmp_inode(a: &Direntry, b: &Direntry) -> Ordering {
    a.ino.cmp(&b.ino)
}

fn streamsavedir(dir: ReadDir, option: SavedirOption) -> io::Result<Vec<u8>> {
    let mut entries = Vec::new();
    let mut name_space = Vec::new();

    let cmp = match option {
        SavedirOption::SortNone => None,
        SavedirOption::SortName => Some(direntry_cmp_name as fn(&Direntry, &Direntry) -> Ordering),
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        SavedirOption::SortInode => Some(direntry_cmp_inode as fn(&Direntry, &Direntry) -> Ordering),
        SavedirOption::SortFastRead => {
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            {
                Some(direntry_cmp_inode as fn(&Direntry, &Direntry) -> Ordering)
            }
            #[cfg(not(any(target_os = "linux", target_os = "macos")))]
            {
                None
            }
        }
    };

    for entry_result in dir {
        let entry = entry_result?;
        let name = entry.file_name();

        // Skip "", ".", and ".."
        if name.is_empty()
            || name == "."
            || name == ".."
        {
            continue;
        }

        let name_bytes = name.as_bytes();
        if let Some(cmp_fn) = cmp {
            let direntry = Direntry::new(&entry)?;
            entries.push(direntry);
        } else {
            name_space.extend_from_slice(name_bytes);
            name_space.push(0);
        }
    }

    if let Some(cmp_fn) = cmp {
        entries.sort_by(cmp_fn);
        for entry in entries {
            name_space.extend_from_slice(entry.name.as_bytes());
            name_space.push(0);
        }
    }

    name_space.push(0);
    Ok(name_space)
}

pub fn savedir<P: AsRef<Path>>(dir: P, option: SavedirOption) -> io::Result<Vec<u8>> {
    let dir = read_dir(dir)?;
    streamsavedir(dir, option)
}
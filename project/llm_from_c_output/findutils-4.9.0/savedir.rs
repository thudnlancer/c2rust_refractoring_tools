use std::cmp::Ordering;
use std::ffi::{CStr, CString, OsStr};
use std::fs::{read_dir, DirEntry, ReadDir};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SavedirOption {
    SortNone,
    SortName,
    #[cfg(target_os = "linux")]
    SortInode,
    SortFastRead,
}

#[cfg(target_os = "linux")]
impl Default for SavedirOption {
    fn default() -> Self {
        SavedirOption::SortInode
    }
}

#[cfg(not(target_os = "linux"))]
impl Default for SavedirOption {
    fn default() -> Self {
        SavedirOption::SortNone
    }
}

struct Direntry {
    name: String,
    #[cfg(target_os = "linux")]
    ino: u64,
}

impl Direntry {
    fn new(entry: &DirEntry) -> io::Result<Self> {
        let name = entry.file_name().into_string().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid UTF-8 in filename",
            )
        })?;

        #[cfg(target_os = "linux")]
        let ino = entry.ino();

        Ok(Direntry {
            name,
            #[cfg(target_os = "linux")]
            ino,
        })
    }
}

fn direntry_cmp_name(a: &Direntry, b: &Direntry) -> Ordering {
    a.name.cmp(&b.name)
}

#[cfg(target_os = "linux")]
fn direntry_cmp_inode(a: &Direntry, b: &Direntry) -> Ordering {
    a.ino.cmp(&b.ino)
}

fn stream_savedir(dir: ReadDir, option: SavedirOption) -> io::Result<Vec<u8>> {
    let mut entries = Vec::new();
    let mut name_space = Vec::new();

    let cmp = match option {
        SavedirOption::SortNone => None,
        SavedirOption::SortName => Some(direntry_cmp_name as fn(&Direntry, &Direntry) -> Ordering),
        #[cfg(target_os = "linux")]
        SavedirOption::SortInode | SavedirOption::SortFastRead => {
            Some(direntry_cmp_inode as fn(&Direntry, &Direntry) -> Ordering)
        }
        #[cfg(not(target_os = "linux"))]
        SavedirOption::SortFastRead => None,
    };

    for entry_result in dir {
        let entry = entry_result?;
        let direntry = Direntry::new(&entry)?;
        let name = direntry.name.as_bytes();

        // Skip ".", "..", and empty names
        if name.is_empty()
            || (name == b".")
            || (name == b"..")
        {
            continue;
        }

        if let Some(cmp_fn) = cmp {
            entries.push(direntry);
        } else {
            name_space.extend_from_slice(name);
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

pub fn savedir(dir: &Path, option: SavedirOption) -> io::Result<Vec<u8>> {
    let dir_reader = read_dir(dir)?;
    stream_savedir(dir_reader, option)
}
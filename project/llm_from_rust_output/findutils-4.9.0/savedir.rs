use std::ffi::{CStr, CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr;
use std::slice;
use std::cmp::Ordering;
use std::fs::{read_dir, DirEntry};
use libc::{c_char, c_int, c_void, size_t, ptrdiff_t};

#[derive(Debug, Clone, Copy)]
pub enum SavedirOption {
    None,
    SortName,
    SortInode,
}

impl SavedirOption {
    fn comparison_fn(self) -> Option<fn(&Direntry, &Direntry) -> Ordering> {
        match self {
            SavedirOption::None => None,
            SavedirOption::SortName => Some(Direntry::cmp_name),
            SavedirOption::SortInode => Some(Direntry::cmp_inode),
        }
    }
}

#[derive(Debug)]
struct Direntry {
    name: CString,
    ino: u64,
}

impl Direntry {
    fn cmp_name(&self, other: &Self) -> Ordering {
        self.name.as_bytes().cmp(other.name.as_bytes())
    }

    fn cmp_inode(&self, other: &Self) -> Ordering {
        self.ino.cmp(&other.ino)
    }
}

pub fn savedir(dir: &Path, option: SavedirOption) -> Option<Vec<CString>> {
    let dir_entries = match read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return None,
    };

    let mut entries = Vec::new();
    let cmp = option.comparison_fn();

    for entry in dir_entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let file_name = entry.file_name();
        let name = match CString::new(file_name.as_bytes()) {
            Ok(n) => n,
            Err(_) => continue,
        };

        // Skip . and .. entries
        if name.as_bytes() == b"." || name.as_bytes() == b".." {
            continue;
        }

        let ino = match entry.metadata() {
            Ok(meta) => meta.ino(),
            Err(_) => continue,
        };

        entries.push(Direntry { name, ino });
    }

    if let Some(cmp_fn) = cmp {
        entries.sort_by(cmp_fn);
    }

    Some(entries.into_iter().map(|e| e.name).collect())
}

pub fn streamsavedir(dir: &Path, option: SavedirOption) -> Option<Vec<CString>> {
    savedir(dir, option)
}

// FFI-compatible wrapper functions
#[no_mangle]
pub extern "C" fn savedir_c(
    dir: *const c_char,
    option: c_int,
) -> *mut *mut c_char {
    let option = match option {
        0 => SavedirOption::None,
        1 => SavedirOption::SortName,
        2 => SavedirOption::SortInode,
        _ => return ptr::null_mut(),
    };

    let dir_path = unsafe { CStr::from_ptr(dir) };
    let dir_path = Path::new(OsStr::from_bytes(dir_path.to_bytes()));

    let entries = match savedir(dir_path, option) {
        Some(e) => e,
        None => return ptr::null_mut(),
    };

    let mut result = Vec::with_capacity(entries.len() + 1);
    for entry in entries {
        result.push(entry.into_raw());
    }
    result.push(ptr::null_mut());

    let boxed = result.into_boxed_slice();
    Box::into_raw(boxed) as *mut *mut c_char
}

#[no_mangle]
pub extern "C" fn streamsavedir_c(
    dir: *const c_char,
    option: c_int,
) -> *mut *mut c_char {
    savedir_c(dir, option)
}

#[no_mangle]
pub extern "C" fn free_savedir_result(result: *mut *mut c_char) {
    if result.is_null() {
        return;
    }

    unsafe {
        let slice = slice::from_raw_parts_mut(result, {
            let mut len = 0;
            while !(*result.add(len)).is_null() {
                len += 1;
            }
            len + 1
        });

        for &mut ptr in &mut slice[..slice.len() - 1] {
            if !ptr.is_null() {
                drop(CString::from_raw(ptr));
            }
        }

        drop(Box::from_raw(slice));
    }
}
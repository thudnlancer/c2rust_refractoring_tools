use std::ffi::{CString, CStr};
use std::fs::{File, OpenOptions};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::ptr;
use std::collections::HashMap;
use libc::{self, c_int, c_void, c_char, dev_t, ino_t};

#[derive(Debug, Clone)]
struct SharefileEntry {
    device: dev_t,
    inode: ino_t,
    name: CString,
    file: File,
}

impl SharefileEntry {
    fn new(filename: &CStr, mode: &CStr) -> Option<Self> {
        let file = match OpenOptions::new()
            .read(true)
            .write(mode.to_bytes().contains(&b'w'[0]))
            .open(Path::new(filename.to_str().ok()?))
        {
            Ok(f) => f,
            Err(_) => return None,
        };

        let metadata = match file.metadata() {
            Ok(m) => m,
            Err(_) => return None,
        };

        Some(Self {
            device: metadata.dev(),
            inode: metadata.ino(),
            name: filename.to_owned(),
            file,
        })
    }
}

#[derive(Debug)]
pub struct Sharefile {
    mode: CString,
    table: HashMap<(dev_t, ino_t), SharefileEntry>,
}

impl Sharefile {
    pub fn new(mode: &CStr) -> Option<Box<Self>> {
        Some(Box::new(Self {
            mode: mode.to_owned(),
            table: HashMap::new(),
        }))
    }

    pub fn open(&mut self, filename: &CStr) -> Option<&File> {
        let entry = SharefileEntry::new(filename, &self.mode)?;

        let key = (entry.device, entry.inode);
        if let Some(existing) = self.table.get(&key) {
            return Some(&existing.file);
        }

        self.table.insert(key, entry);
        Some(&self.table.get(&key)?.file)
    }
}

pub type SharefileHandle = Box<Sharefile>;

#[no_mangle]
pub extern "C" fn sharefile_init(mode: *const c_char) -> *mut c_void {
    let mode_cstr = unsafe { CStr::from_ptr(mode) };
    match Sharefile::new(mode_cstr) {
        Some(handle) => Box::into_raw(handle) as *mut c_void,
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn sharefile_destroy(handle: *mut c_void) {
    if !handle.is_null() {
        unsafe { Box::from_raw(handle as *mut Sharefile) };
    }
}

#[no_mangle]
pub extern "C" fn sharefile_fopen(
    handle: *mut c_void,
    filename: *const c_char,
) -> *mut libc::FILE {
    if handle.is_null() || filename.is_null() {
        return ptr::null_mut();
    }

    let sharefile = unsafe { &mut *(handle as *mut Sharefile) };
    let filename_cstr = unsafe { CStr::from_ptr(filename) };

    match sharefile.open(filename_cstr) {
        Some(file) => unsafe { libc::fdopen(file.as_raw_fd(), sharefile.mode.as_ptr()) },
        None => ptr::null_mut(),
    }
}
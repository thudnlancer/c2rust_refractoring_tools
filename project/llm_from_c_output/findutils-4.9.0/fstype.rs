/*!
 * Determine type of file systems that files are on.
 * Converted from C to Rust with safety and idiomatic practices.
 */

use std::ffi::{CStr, CString};
use std::fs;
use std::io;
use std::mem;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::ptr;
use std::str;

use libc::{dev_t, stat};
use nix::mount::{getmntent, MntIter};

#[derive(Debug)]
struct MountEntry {
    me_dev: dev_t,
    me_mountdir: PathBuf,
    me_type: String,
    me_next: Option<Box<MountEntry>>,
}

impl MountEntry {
    fn new(dev: dev_t, mountdir: PathBuf, fs_type: String) -> Self {
        MountEntry {
            me_dev: dev,
            me_mountdir,
            me_type: fs_type,
            me_next: None,
        }
    }
}

fn free_file_system_list(mut p: Option<Box<MountEntry>>) {
    while let Some(entry) = p {
        p = entry.me_next;
    }
}

fn get_file_system_list(need_fs_type: bool) -> Option<Box<MountEntry>> {
    static mut MOUNT_LIST: Option<Box<MountEntry>> = None;
    static mut HAS_FSTYPE: bool = false;

    unsafe {
        if MOUNT_LIST.is_some() && !HAS_FSTYPE && need_fs_type {
            free_file_system_list(MOUNT_LIST.take());
        }
        if MOUNT_LIST.is_none() {
            MOUNT_LIST = read_file_system_list(need_fs_type);
            HAS_FSTYPE = need_fs_type;
        }
        MOUNT_LIST.clone()
    }
}

fn read_file_system_list(need_fs_type: bool) -> Option<Box<MountEntry>> {
    let mut head: Option<Box<MountEntry>> = None;
    let mut tail = &mut head;

    if let Ok(mnt_iter) = MntIter::new() {
        for mnt in mnt_iter {
            let mountdir = PathBuf::from(mnt.mnt_dir);
            let fs_type = if need_fs_type {
                String::from(mnt.mnt_fstype)
            } else {
                String::new()
            };

            let dev = unsafe {
                let mut stat_buf: stat = mem::zeroed();
                if libc::stat(mnt.mnt_dir.as_ptr(), &mut stat_buf) == 0 {
                    stat_buf.st_dev
                } else {
                    continue;
                }
            };

            let entry = Box::new(MountEntry::new(dev, mountdir, fs_type));
            *tail = Some(entry);
            if let Some(ref mut t) = *tail {
                tail = &mut t.me_next;
            }
        }
    }

    head
}

fn filesystem_type(statp: &stat, path: &Path) -> String {
    static mut CURRENT_FSTYPE: Option<String> = None;
    static mut CURRENT_DEV: dev_t = 0;
    static mut FSTYPE_KNOWN: bool = false;

    unsafe {
        if let Some(ref fstype) = CURRENT_FSTYPE {
            if FSTYPE_KNOWN && statp.st_dev == CURRENT_DEV {
                return fstype.clone();
            }
            CURRENT_FSTYPE = None;
        }
        CURRENT_DEV = statp.st_dev;
        let fstype = file_system_type_uncached(statp, path);
        CURRENT_FSTYPE = Some(fstype.clone());
        fstype
    }
}

fn file_system_type_uncached(statp: &stat, path: &Path) -> String {
    let mut fstype_known = false;
    let mut best_type = None;

    if let Some(entries) = get_file_system_list(true) {
        let mut entry = Some(&*entries);
        while let Some(e) = entry {
            if e.me_dev == statp.st_dev {
                best_type = Some(e.me_type.clone());
                // Don't break, may have duplicates
            }
            entry = e.me_next.as_ref().map(|e| &**e);
        }
    }

    fstype_known = best_type.is_some();
    best_type.unwrap_or_else(|| "unknown".to_string())
}

fn is_used_fs_type(name: &str) -> bool {
    if name == "afs" {
        return true;
    }

    if let Some(entries) = get_file_system_list(false) {
        let mut entry = Some(&*entries);
        while let Some(e) = entry {
            if e.me_type == name {
                return true;
            }
            entry = e.me_next.as_ref().map(|e| &**e);
        }
    }

    false
}

fn get_mounted_devices() -> Vec<dev_t> {
    let mut devices = Vec::new();

    if let Some(entries) = read_file_system_list(false) {
        let mut entry = Some(&*entries);
        while let Some(e) = entry {
            devices.push(e.me_dev);
            entry = e.me_next.as_ref().map(|e| &**e);
        }
    }

    devices
}

// Note: AFS and other platform-specific functionality omitted for brevity
// Would need platform-specific implementations and unsafe blocks
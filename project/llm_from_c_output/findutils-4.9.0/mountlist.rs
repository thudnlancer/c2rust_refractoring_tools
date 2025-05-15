use std::ffi::{CStr, CString};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr;
use std::str;
use libc::{dev_t, major, minor, makedev};
use nix::sys::statvfs;
use nix::mount::{MntTab, MntTabEntry};
use std::collections::LinkedList;

#[derive(Debug)]
pub struct MountEntry {
    pub devname: String,
    pub mountdir: String,
    pub mntroot: Option<String>,
    pub fs_type: String,
    pub dev: dev_t,
    pub dummy: bool,
    pub remote: bool,
    pub type_malloced: bool,
    pub next: Option<Box<MountEntry>>,
}

impl MountEntry {
    pub fn new(
        devname: String,
        mountdir: String,
        mntroot: Option<String>,
        fs_type: String,
        dev: dev_t,
        dummy: bool,
        remote: bool,
        type_malloced: bool,
    ) -> Self {
        MountEntry {
            devname,
            mountdir,
            mntroot,
            fs_type,
            dev,
            dummy,
            remote,
            type_malloced,
            next: None,
        }
    }
}

pub fn read_file_system_list(need_fs_type: bool) -> io::Result<Option<Box<MountEntry>>> {
    let mut mount_list: Option<Box<MountEntry>> = None;
    let mut mtail = &mut mount_list;

    #[cfg(target_os = "linux")]
    {
        if let Ok(file) = File::open("/proc/self/mountinfo") {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 10 {
                    continue;
                }

                let dev_parts: Vec<&str> = parts[2].split(':').collect();
                if dev_parts.len() != 2 {
                    continue;
                }

                let devmaj = dev_parts[0].parse::<u32>().unwrap_or(0);
                let devmin = dev_parts[1].parse::<u32>().unwrap_or(0);
                let dev = makedev(devmaj, devmin);

                let mntroot = parts[3].to_string();
                let target = parts[4].to_string();
                let fstype = parts[8].to_string();
                let source = parts[9].to_string();

                let me = Box::new(MountEntry::new(
                    source,
                    target,
                    Some(mntroot),
                    fstype,
                    dev,
                    false, // dummy
                    false, // remote
                    true,  // type_malloced
                ));

                *mtail = Some(me);
                if let Some(ref mut entry) = *mtail {
                    mtail = &mut entry.next;
                }
            }
            return Ok(mount_list);
        }

        // Fallback to /proc/mounts
        if let Ok(file) = File::open("/proc/mounts") {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 3 {
                    continue;
                }

                let source = parts[0].to_string();
                let target = parts[1].to_string();
                let fstype = parts[2].to_string();

                let me = Box::new(MountEntry::new(
                    source,
                    target,
                    None,
                    fstype,
                    0,    // dev
                    false, // dummy
                    false, // remote
                    true,  // type_malloced
                ));

                *mtail = Some(me);
                if let Some(ref mut entry) = *mtail {
                    mtail = &mut entry.next;
                }
            }
            return Ok(mount_list);
        }
    }

    #[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "openbsd"))]
    {
        use libc::{getmntinfo, statfs};

        let mut mntbuf: *mut statfs = ptr::null_mut();
        let count = unsafe { getmntinfo(&mut mntbuf, libc::MNT_NOWAIT) };
        if count <= 0 {
            return Ok(None);
        }

        let mntbuf_slice = unsafe { std::slice::from_raw_parts(mntbuf, count as usize) };
        for mnt in mntbuf_slice {
            let fs_type = unsafe { CStr::from_ptr(mnt.f_fstypename.as_ptr()) }
                .to_str()
                .unwrap_or("")
                .to_string();

            let me = Box::new(MountEntry::new(
                unsafe { CStr::from_ptr(mnt.f_mntfromname.as_ptr()) }
                    .to_str()
                    .unwrap_or("")
                    .to_string(),
                unsafe { CStr::from_ptr(mnt.f_mntonname.as_ptr()) }
                    .to_str()
                    .unwrap_or("")
                    .to_string(),
                None,
                fs_type,
                0,    // dev
                false, // dummy
                false, // remote
                false, // type_malloced
            ));

            *mtail = Some(me);
            if let Some(ref mut entry) = *mtail {
                mtail = &mut entry.next;
            }
        }
        return Ok(mount_list);
    }

    // Other platforms would go here...

    Ok(None)
}

pub fn free_mount_entry(mut me: Box<MountEntry>) {
    // Rust's ownership system will automatically free all the memory
    // when the Box goes out of scope
    while let Some(next) = me.next.take() {
        me = next;
    }
}
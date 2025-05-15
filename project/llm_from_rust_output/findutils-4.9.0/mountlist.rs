use std::ffi::{CStr, CString, OsStr};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr;

#[derive(Debug)]
pub struct MountEntry {
    pub me_devname: String,
    pub me_mountdir: String,
    pub me_mntroot: Option<String>,
    pub me_type: String,
    pub me_dev: u64,
    pub me_dummy: bool,
    pub me_remote: bool,
    pub me_type_malloced: bool,
    pub me_next: Option<Box<MountEntry>>,
}

fn unescape_tab(s: &mut String) {
    let mut result = String::new();
    let mut chars = s.chars().collect::<Vec<_>>();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' && i + 3 < chars.len() {
            let octal = &chars[i+1..i+4];
            if octal.iter().all(|c| c.is_digit(8)) {
                let code = octal.iter().fold(0, |acc, c| 
                    acc * 8 + c.to_digit(8).unwrap());
                result.push(char::from_u32(code).unwrap_or('?'));
                i += 4;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    *s = result;
}

fn terminate_at_blank(s: &mut String) -> Option<()> {
    if let Some(pos) = s.find(' ') {
        s.truncate(pos);
        Some(())
    } else {
        None
    }
}

fn gnu_dev_makedev(major: u32, minor: u32) -> u64 {
    ((major & 0xfff) << 8) as u64 | 
    ((major & 0xfffff000) << 32) as u64 |
    ((minor & 0xff) << 0) as u64 | 
    ((minor & 0xffffff00) << 12) as u64
}

fn dev_from_mount_options(_options: &str) -> u64 {
    !0
}

fn is_dummy_fs(fs_type: &str) -> bool {
    matches!(fs_type,
        "autofs" | "proc" | "subfs" | "debugfs" | "devpts" | "fusectl" |
        "fuse.portal" | "mqueue" | "rpc_pipefs" | "sysfs" | "devfs" |
        "kernfs" | "ignore" | "none"
    )
}

fn is_remote_fs(devname: &str, fs_type: &str) -> bool {
    devname.contains(':') ||
    (devname.starts_with("//") && matches!(fs_type, "smbfs" | "smb3" | "cifs")) ||
    matches!(fs_type,
        "acfs" | "afs" | "coda" | "auristorfs" | "fhgfs" | "gpfs" |
        "ibrix" | "ocfs2" | "vxfs"
    ) ||
    devname == "-hosts"
}

pub fn read_file_system_list(need_fs_type: bool) -> Result<Option<Box<MountEntry>>, Error> {
    if let Ok(mount_list) = read_mountinfo() {
        Ok(mount_list)
    } else {
        read_mtab()
    }
}

fn read_mountinfo() -> Result<Option<Box<MountEntry>>, Error> {
    let file = File::open("/proc/self/mountinfo")?;
    let reader = BufReader::new(file);
    let mut mount_list = None;
    let mut mtail = &mut mount_list;

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.len() < 10 { continue; }

        let dev_parts: Vec<u32> = parts[2].split(':')
            .filter_map(|s| s.parse().ok())
            .collect();
        if dev_parts.len() != 2 { continue; }

        let mntroot = parts[3].to_string();
        let target = parts[4].to_string();

        let dash_pos = line.find(" - ").ok_or(ErrorKind::InvalidData)?;
        let rest = &line[dash_pos+3..];
        let mut rest_parts = rest.split_whitespace().collect::<Vec<_>>();
        if rest_parts.len() < 2 { continue; }

        let fstype = rest_parts[0].to_string();
        let source = rest_parts[1].to_string();

        let mut me = MountEntry {
            me_devname: source,
            me_mountdir: target,
            me_mntroot: Some(mntroot),
            me_type: fstype,
            me_dev: gnu_dev_makedev(dev_parts[0], dev_parts[1]),
            me_dummy: is_dummy_fs(&me.me_type),
            me_remote: is_remote_fs(&me.me_devname, &me.me_type),
            me_type_malloced: true,
            me_next: None,
        };

        unescape_tab(&mut me.me_devname);
        unescape_tab(&mut me.me_mountdir);
        unescape_tab(me.me_mntroot.as_mut().unwrap());
        unescape_tab(&mut me.me_type);

        *mtail = Some(Box::new(me));
        if let Some(ref mut entry) = *mtail {
            mtail = &mut entry.me_next;
        }
    }

    Ok(mount_list)
}

fn read_mtab() -> Result<Option<Box<MountEntry>>, Error> {
    let file = File::open("/etc/mtab")?;
    let reader = BufReader::new(file);
    let mut mount_list = None;
    let mut mtail = &mut mount_list;

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 { continue; }

        let bind = parts[3].contains("bind");

        let mut me = MountEntry {
            me_devname: parts[0].to_string(),
            me_mountdir: parts[1].to_string(),
            me_mntroot: None,
            me_type: parts[2].to_string(),
            me_dev: dev_from_mount_options(parts[3]),
            me_dummy: is_dummy_fs(&me.me_type) && !bind,
            me_remote: is_remote_fs(&me.me_devname, &me.me_type),
            me_type_malloced: true,
            me_next: None,
        };

        *mtail = Some(Box::new(me));
        if let Some(ref mut entry) = *mtail {
            mtail = &mut entry.me_next;
        }
    }

    Ok(mount_list)
}

pub fn free_mount_entry(mut me: Option<Box<MountEntry>>) {
    while let Some(entry) = me {
        me = entry.me_next;
    }
}
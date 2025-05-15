use std::ffi::CString;
use std::os::unix::fs::{MetadataExt, FileTypeExt};
use std::path::Path;
use std::fs::{metadata, symlink_metadata, read_link};
use std::time::SystemTime;
use libc::{stat, statvfs, dev_t, mode_t, ino_t, nlink_t, uid_t, gid_t, off_t, blkcnt_t, blksize_t, time_t};
use nix::sys::stat::{major, minor};
use std::collections::HashMap;
use std::ptr;

const S_IFMT: mode_t = 0o170000;
const S_IFREG: mode_t = 0o100000;
const S_IFBLK: mode_t = 0o060000;
const S_IFCHR: mode_t = 0o020000;
const S_IFDIR: mode_t = 0o040000;
const S_IFLNK: mode_t = 0o120000;
const S_IFSOCK: mode_t = 0o140000;
const S_IFIFO: mode_t = 0o010000;
const S_IRUSR: mode_t = 0o0400;
const S_IWUSR: mode_t = 0o0200;
const S_IXUSR: mode_t = 0o0100;
const S_IRGRP: mode_t = 0o0040;
const S_IWGRP: mode_t = 0o0020;
const S_IXGRP: mode_t = 0o0010;
const S_IROTH: mode_t = 0o0004;
const S_IWOTH: mode_t = 0o0002;
const S_IXOTH: mode_t = 0o0001;
const S_ISUID: mode_t = 0o4000;
const S_ISGID: mode_t = 0o2000;
const S_ISVTX: mode_t = 0o1000;

struct FTS;
struct FTSENT;

type AwkValue = HashMap<String, String>;
type AwkArray = HashMap<String, AwkValue>;

fn format_mode(fmode: mode_t) -> String {
    let mut outbuf = String::from("----------");
    
    let ftype_map = [
        (S_IFREG, '-'),
        (S_IFBLK, 'b'),
        (S_IFCHR, 'c'),
        (S_IFDIR, 'd'),
        (S_IFLNK, 'l'),
        (S_IFSOCK, 's'),
        (S_IFIFO, 'p'),
    ];
    
    let mode_map = [
        (S_IRUSR, 'r'), (S_IWUSR, 'w'), (S_IXUSR, 'x'),
        (S_IRGRP, 'r'), (S_IWGRP, 'w'), (S_IXGRP, 'x'),
        (S_IROTH, 'r'), (S_IWOTH, 'w'), (S_IXOTH, 'x'),
    ];
    
    let setuid_map = [
        (S_ISUID, 3, 's', 'S'),
        (S_ISGID, 6, 's', 'l'),
        (S_ISVTX, 9, 't', 'T'),
    ];
    
    // Set file type
    for (mask, charval) in &ftype_map {
        if (fmode & S_IFMT) == *mask {
            outbuf.replace_range(0..1, &charval.to_string());
            break;
        }
    }
    
    // Set permissions
    for (i, (mask, rep)) in mode_map.iter().enumerate() {
        if (fmode & mask) != 0 {
            outbuf.replace_range(i+1..i+2, &rep.to_string());
        }
    }
    
    // Set special bits
    for (mask, index, small_rep, big_rep) in &setuid_map {
        if (fmode & mask) != 0 {
            let idx = *index;
            let c = if outbuf.chars().nth(idx).unwrap() == 'x' {
                *small_rep
            } else {
                *big_rep
            };
            outbuf.replace_range(idx..idx+1, &c.to_string());
        }
    }
    
    outbuf
}

fn fill_stat_array(name: &str, array: &mut AwkArray, sbuf: &stat) {
    array.clear();
    
    array.insert("name".to_string(), AwkValue::from([("value".to_string(), name.to_string())]));
    array.insert("dev".to_string(), AwkValue::from([("value".to_string(), sbuf.st_dev.to_string())]));
    array.insert("ino".to_string(), AwkValue::from([("value".to_string(), sbuf.st_ino.to_string())]));
    array.insert("mode".to_string(), AwkValue::from([("value".to_string(), sbuf.st_mode.to_string())]));
    array.insert("nlink".to_string(), AwkValue::from([("value".to_string(), sbuf.st_nlink.to_string())]));
    array.insert("uid".to_string(), AwkValue::from([("value".to_string(), sbuf.st_uid.to_string())]));
    array.insert("gid".to_string(), AwkValue::from([("value".to_string(), sbuf.st_gid.to_string())]));
    array.insert("size".to_string(), AwkValue::from([("value".to_string(), sbuf.st_size.to_string())]));
    array.insert("blocks".to_string(), AwkValue::from([("value".to_string(), sbuf.st_blocks.to_string())]));
    array.insert("atime".to_string(), AwkValue::from([("value".to_string(), sbuf.st_atime.to_string())]));
    array.insert("mtime".to_string(), AwkValue::from([("value".to_string(), sbuf.st_mtime.to_string())]));
    array.insert("ctime".to_string(), AwkValue::from([("value".to_string(), sbuf.st_ctime.to_string())]));
    
    if (sbuf.st_mode & S_IFMT) == S_IFBLK || (sbuf.st_mode & S_IFMT) == S_IFCHR {
        array.insert("rdev".to_string(), AwkValue::from([("value".to_string(), sbuf.st_rdev.to_string())]));
        array.insert("major".to_string(), AwkValue::from([("value".to_string(), major(sbuf.st_rdev).to_string())]));
        array.insert("minor".to_string(), AwkValue::from([("value".to_string(), minor(sbuf.st_rdev).to_string())]));
    }
    
    array.insert("blksize".to_string(), AwkValue::from([("value".to_string(), sbuf.st_blksize.to_string())]));
    array.insert("devbsize".to_string(), AwkValue::from([("value".to_string(), "512".to_string())]));
    
    let pmode = format_mode(sbuf.st_mode);
    array.insert("pmode".to_string(), AwkValue::from([("value".to_string(), pmode)]));
    
    if (sbuf.st_mode & S_IFMT) == S_IFLNK {
        if let Ok(link) = read_link(name) {
            if let Some(link_str) = link.to_str() {
                array.insert("linkval".to_string(), AwkValue::from([("value".to_string(), link_str.to_string())]));
            }
        }
    }
    
    let type_map = [
        (S_IFREG, "file"),
        (S_IFBLK, "blockdev"),
        (S_IFCHR, "chardev"),
        (S_IFDIR, "directory"),
        (S_IFLNK, "symlink"),
        (S_IFSOCK, "socket"),
        (S_IFIFO, "fifo"),
    ];
    
    for (mask, type_str) in &type_map {
        if (sbuf.st_mode & S_IFMT) == *mask {
            array.insert("type".to_string(), AwkValue::from([("value".to_string(), type_str.to_string())]));
            break;
        }
    }
}

fn do_stat(path: &str, array: &mut AwkArray, follow_link: bool) -> i32 {
    let path_c = CString::new(path).unwrap();
    let mut sbuf: stat = unsafe { std::mem::zeroed() };
    
    let res = if follow_link {
        unsafe { stat(path_c.as_ptr(), &mut sbuf) }
    } else {
        unsafe { lstat(path_c.as_ptr(), &mut sbuf) }
    };
    
    if res == 0 {
        fill_stat_array(path, array, &sbuf);
        0
    } else {
        -1
    }
}

fn do_statvfs(path: &str, array: &mut AwkArray) -> i32 {
    let path_c = CString::new(path).unwrap();
    let mut vfsbuf: statvfs = unsafe { std::mem::zeroed() };
    
    let res = unsafe { statvfs(path_c.as_ptr(), &mut vfsbuf) };
    
    if res == 0 {
        array.insert("bsize".to_string(), AwkValue::from([("value".to_string(), vfsbuf.f_bsize.to_string())]));
        array.insert("frsize".to_string(), AwkValue::from([("value".to_string(), vfsbuf.f_frsize.to_string())]));
        array.insert("blocks".to_string(), AwkValue::from([("value".to_string(), vfsbuf.f_blocks.to_string())]));
        array.insert("bfree".to_string(), AwkValue::from([("value".to_string(), vfsbuf.f_bfree.to_string())]));
        array.insert("bavail".to_string(), AwkValue::from([("value".to_string(), vfsbuf.f_bavail.to_string())]));
        array.insert("files".to_string(), AwkValue::from([("value".to_string(), vfsbuf.f_files.to_string())]));
        array.insert("ffree".to_string(), AwkValue::from([("value".to_string(), vfsbuf.f_ffree.to_string())]));
        array.insert("favail".to_string(), AwkValue::from([("value".to_string(), vfsbuf.f_favail.to_string())]));
        array.insert("flag".to_string(), AwkValue::from([("value".to_string(), vfsbuf.f_flag.to_string())]));
        array.insert("namemax".to_string(), AwkValue::from([("value".to_string(), vfsbuf.f_namemax.to_string())]));
        0
    } else {
        -1
    }
}

fn do_chdir(path: &str) -> i32 {
    if let Ok(_) = std::env::set_current_dir(path) {
        0
    } else {
        -1
    }
}

struct FtsOptions {
    physical: bool,
    logical: bool,
    no_chdir: bool,
    seedot: bool,
    xdev: bool,
    non_recursive: bool,
}

impl FtsOptions {
    fn from_flags(flags: i32) -> Self {
        FtsOptions {
            physical: (flags & FTS_PHYSICAL) != 0,
            logical: (flags & FTS_LOGICAL) != 0,
            no_chdir: (flags & FTS_NOCHDIR) != 0,
            seedot: (flags & FTS_SEEDOT) != 0,
            xdev: (flags & FTS_XDEV) != 0,
            non_recursive: (flags & FTS_NON_RECURSIVE) != 0,
        }
    }
}

const FTS_PHYSICAL: i32 = 1;
const FTS_LOGICAL: i32 = 2;
const FTS_NOCHDIR: i32 = 4;
const FTS_SEEDOT: i32 = 8;
const FTS_XDEV: i32 = 16;
const FTS_NON_RECURSIVE: i32 = 32;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_mode() {
        assert_eq!(format_mode(S_IFREG | S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH), "-rw-r--r--");
        assert_eq!(format_mode(S_IFDIR | S_IRWXU | S_IRGRP | S_IXGRP | S_IROTH | S_IXOTH), "drwxr-xr-x");
        assert_eq!(format_mode(S_IFLNK | S_IRWXU | S_IRWXG | S_IRWXO), "lrwxrwxrwx");
        assert_eq!(format_mode(S_IFREG | S_ISUID | S_IRUSR | S_IWUSR | S_IXUSR), "-rws------");
    }
    
    #[test]
    fn test_do_stat() {
        let mut array = AwkArray::new();
        let res = do_stat("/", &mut array, false);
        assert_eq!(res, 0);
        assert!(array.contains_key("name"));
        assert!(array.contains_key("mode"));
        assert!(array.contains_key("type"));
    }
    
    #[test]
    fn test_do_chdir() {
        let res = do_chdir("/");
        assert_eq!(res, 0);
    }
}
use std::{
    ffi::{CStr, CString},
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    os::unix::fs::{symlink, OpenOptionsExt, PermissionsExt},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use libc::{
    c_char, c_int, c_long, c_uint, c_ulong, c_ushort, dev_t, gid_t, mode_t, off_t, size_t, time_t,
    uid_t,
};
use nix::{
    sys::stat::{self, SFlag},
    unistd::{chown, close, lchown},
};

struct CpioFileStat {
    c_magic: c_ushort,
    c_ino: u64,
    c_mode: mode_t,
    c_uid: uid_t,
    c_gid: gid_t,
    c_nlink: size_t,
    c_mtime: time_t,
    c_filesize: off_t,
    c_dev_maj: c_uint,
    c_dev_min: c_uint,
    c_rdev_maj: c_uint,
    c_rdev_min: c_uint,
    c_namesize: size_t,
    c_chksum: u32,
    c_name: CString,
    c_tar_linkname: Option<CString>,
}

impl CpioFileStat {
    fn new() -> Self {
        Self {
            c_magic: 0,
            c_ino: 0,
            c_mode: 0,
            c_uid: 0,
            c_gid: 0,
            c_nlink: 0,
            c_mtime: 0,
            c_filesize: 0,
            c_dev_maj: 0,
            c_dev_min: 0,
            c_rdev_maj: 0,
            c_rdev_min: 0,
            c_namesize: 0,
            c_chksum: 0,
            c_name: CString::new("").unwrap(),
            c_tar_linkname: None,
        }
    }

    fn set_name(&mut self, name: &CStr) {
        self.c_name = name.to_owned();
        self.c_namesize = name.to_bytes().len() as size_t;
    }
}

fn warn_junk_bytes(bytes_skipped: c_long) {
    eprintln!(
        "warning: skipped {} byte{} of junk",
        bytes_skipped,
        if bytes_skipped == 1 { "" } else { "s" }
    );
}

fn list_file(file_hdr: &CpioFileStat, in_file: &mut File) -> io::Result<()> {
    // Implementation of listing file details
    Ok(())
}

fn try_existing_file(
    file_hdr: &CpioFileStat,
    in_file: &mut File,
) -> io::Result<(bool, bool)> {
    // Implementation of checking existing file
    Ok((false, false))
}

fn copyin_regular_file(file_hdr: &CpioFileStat, in_file: &mut File, out_file: &mut File) -> io::Result<()> {
    // Implementation of copying regular file
    Ok(())
}

fn copyin_device(file_hdr: &CpioFileStat) -> io::Result<()> {
    // Implementation of creating device file
    Ok(())
}

fn copyin_link(file_hdr: &CpioFileStat, in_file: &mut File) -> io::Result<()> {
    // Implementation of creating symlink
    Ok(())
}

fn read_in_header(file_hdr: &mut CpioFileStat, in_file: &mut File) -> io::Result<()> {
    // Implementation of reading header
    Ok(())
}

fn process_copy_in() -> io::Result<()> {
    let mut file_hdr = CpioFileStat::new();
    let mut in_file = File::open("/dev/stdin")?;
    let mut out_file = if to_stdout_option {
        File::create("/dev/stdout")?
    } else {
        // Handle output file creation
        unimplemented!()
    };

    loop {
        read_in_header(&mut file_hdr, &mut in_file)?;

        if file_hdr.c_namesize == 0 {
            continue;
        }

        if file_hdr.c_name.as_bytes() == b"TRAILER!!!" {
            break;
        }

        if table_flag {
            list_file(&file_hdr, &mut in_file)?;
        } else if only_verify_crc_flag {
            // Verify CRC
        } else {
            match file_hdr.c_mode & SFlag::S_IFMT.bits() {
                SFlag::S_IFREG.bits() => {
                    copyin_regular_file(&file_hdr, &mut in_file, &mut out_file)?;
                }
                SFlag::S_IFDIR.bits() => {
                    // Create directory
                }
                SFlag::S_IFBLK.bits() | SFlag::S_IFCHR.bits() => {
                    copyin_device(&file_hdr)?;
                }
                SFlag::S_IFLNK.bits() => {
                    copyin_link(&file_hdr, &mut in_file)?;
                }
                _ => {
                    eprintln!("{}: unknown file type", file_hdr.c_name.to_string_lossy());
                }
            }
        }
    }

    Ok(())
}

// Global flags (would normally be in a config struct)
static mut to_stdout_option: bool = false;
static mut table_flag: bool = false;
static mut only_verify_crc_flag: bool = false;

fn main() -> io::Result<()> {
    process_copy_in()
}
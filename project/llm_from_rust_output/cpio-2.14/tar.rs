use std::ffi::{CStr, CString};
use std::mem;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct CpioFileStat {
    pub c_magic: u16,
    pub c_ino: u64,
    pub c_mode: u32,
    pub c_uid: u32,
    pub c_gid: u32,
    pub c_nlink: usize,
    pub c_mtime: i64,
    pub c_filesize: i64,
    pub c_dev_maj: u32,
    pub c_dev_min: u32,
    pub c_rdev_maj: u32,
    pub c_rdev_min: u32,
    pub c_namesize: usize,
    pub c_chksum: u32,
    pub c_name: String,
    pub c_name_buflen: usize,
    pub c_tar_linkname: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveFormat {
    Unknown,
    Binary,
    OldAscii,
    NewAscii,
    CrcAscii,
    Tar,
    Ustar,
    HpOldAscii,
    HpBinary,
}

#[derive(Debug, Clone)]
pub struct TarHeader {
    pub name: [u8; 100],
    pub mode: [u8; 8],
    pub uid: [u8; 8],
    pub gid: [u8; 8],
    pub size: [u8; 12],
    pub mtime: [u8; 12],
    pub chksum: [u8; 8],
    pub typeflag: u8,
    pub linkname: [u8; 100],
    pub magic: [u8; 6],
    pub version: [u8; 2],
    pub uname: [u8; 32],
    pub gname: [u8; 32],
    pub devmajor: [u8; 8],
    pub devminor: [u8; 8],
    pub prefix: [u8; 155],
}

#[derive(Debug, Clone)]
pub struct TarRecord {
    pub header: TarHeader,
}

impl TarRecord {
    pub fn new() -> Self {
        TarRecord {
            header: TarHeader {
                name: [0; 100],
                mode: [0; 8],
                uid: [0; 8],
                gid: [0; 8],
                size: [0; 12],
                mtime: [0; 12],
                chksum: [0; 8],
                typeflag: 0,
                linkname: [0; 100],
                magic: [0; 6],
                version: [0; 2],
                uname: [0; 32],
                gname: [0; 32],
                devmajor: [0; 8],
                devminor: [0; 8],
                prefix: [0; 155],
            },
        }
    }
}

pub fn stash_tar_linkname(linkname: &str) -> String {
    let mut hold = [0u8; 101];
    let bytes = linkname.as_bytes();
    let len = bytes.len().min(100);
    hold[..len].copy_from_slice(&bytes[..len]);
    hold[100] = 0;
    unsafe { CStr::from_ptr(hold.as_ptr() as *const i8).to_string_lossy().into_owned() }
}

pub fn split_long_name(name: &str, length: usize) -> usize {
    let mut i = if length > 155 { 155 + 2 } else { length };
    while i > 0 {
        if name.as_bytes()[i - 1] == b'/' {
            break;
        }
        i -= 1;
    }
    i
}

pub fn stash_tar_filename(prefix: Option<&str>, filename: &str) -> String {
    let mut hold = [0u8; 257];
    if prefix.is_none() || prefix.unwrap().is_empty() {
        let len = filename.len().min(100);
        hold[..len].copy_from_slice(&filename.as_bytes()[..len]);
        hold[100] = 0;
    } else {
        let prefix = prefix.unwrap();
        let prefix_len = prefix.len().min(155);
        hold[..prefix_len].copy_from_slice(&prefix.as_bytes()[..prefix_len]);
        hold[prefix_len] = b'/';
        let filename_len = filename.len().min(100);
        hold[prefix_len + 1..prefix_len + 1 + filename_len]
            .copy_from_slice(&filename.as_bytes()[..filename_len]);
        hold[155 + 100] = 0;
    }
    unsafe { CStr::from_ptr(hold.as_ptr() as *const i8).to_string_lossy().into_owned() }
}

pub fn to_oct_or_error(
    value: u64,
    digits: usize,
    field: &str,
    file: &str,
) -> Result<(), String> {
    // Implementation of to_ascii would go here
    Ok(())
}

pub fn tar_checksum(tar_hdr: &TarHeader) -> u32 {
    let mut sum = 0u32;
    let bytes = unsafe {
        std::slice::from_raw_parts(
            tar_hdr as *const _ as *const u8,
            mem::size_of::<TarHeader>(),
        )
    };
    
    // Sum all bytes except the checksum field
    let chksum_start = 148; // offset of chksum field
    let chksum_end = chksum_start + 8;
    
    for (i, &byte) in bytes.iter().enumerate() {
        if !(chksum_start..chksum_end).contains(&i) {
            sum += byte as u32;
        }
    }
    
    // Add spaces for checksum field
    sum += 8 * b' ' as u32;
    sum
}

// Additional functions would be implemented similarly, converting C patterns to Rust idioms
// while maintaining safety and functionality

pub fn write_out_tar_header(
    file_hdr: &CpioFileStat,
    out_des: i32,
    archive_format: ArchiveFormat,
    numeric_uid: i32,
) -> Result<(), String> {
    let mut tar_rec = TarRecord::new();
    let name_len = file_hdr.c_name.len();
    
    if name_len <= 100 {
        tar_rec.header.name[..name_len].copy_from_slice(file_hdr.c_name.as_bytes());
    } else {
        let prefix_len = split_long_name(&file_hdr.c_name, name_len);
        tar_rec.header.prefix[..prefix_len].copy_from_slice(&file_hdr.c_name.as_bytes()[..prefix_len]);
        tar_rec.header.name[..name_len - prefix_len - 1].copy_from_slice(
            &file_hdr.c_name.as_bytes()[prefix_len + 1..],
        );
    }
    
    // Implementation continues...
    Ok(())
}

// Other functions would follow similar patterns
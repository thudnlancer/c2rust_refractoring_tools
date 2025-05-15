use std::ffi::CString;
use std::os::unix::process::CommandExt;
use std::process::{Command, exit};
use std::ptr;

#[derive(Clone)]
pub struct Device {
    pub name: Option<CString>,
    pub drive: char,
    pub fat_bits: i32,
    pub mode: i32,
    pub tracks: u32,
    pub heads: u16,
    pub sectors: u16,
    pub hidden: u32,
    pub offset: i64,
    pub partition: u32,
    pub misc_flags: u32,
    pub ssize: u8,
    pub use_2m: u32,
    pub precmd: Option<CString>,
    pub file_nr: i32,
    pub blocksize: u32,
    pub codepage: u32,
    pub data_map: Option<CString>,
    pub tot_sectors: u32,
    pub sector_size: u16,
    pub postcmd: Option<CString>,
    pub cfg_filename: Option<CString>,
}

pub fn precmd(dev: &Device) {
    if let Some(cmd) = &dev.precmd {
        postcmd(cmd);
    }
}

pub fn postcmd(cmd: &CString) {
    match unsafe { libc::fork() } {
        -1 => {
            eprintln!("Could not fork");
            exit(1);
        }
        0 => {
            let args = ["sh", "-c", cmd.to_str().unwrap()];
            let err = Command::new("/bin/sh")
                .args(&args)
                .exec();
            eprintln!("Failed to execute command: {}", err);
            exit(1);
        }
        pid => {
            let mut status = 0;
            unsafe { libc::wait(&mut status) };
        }
    }
}
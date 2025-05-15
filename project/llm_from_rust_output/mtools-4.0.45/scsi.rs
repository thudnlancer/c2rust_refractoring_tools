use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScsiIoMode {
    Read = 0,
    Write = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SgIoHdr {
    interface_id: i32,
    dxfer_direction: i32,
    cmd_len: u8,
    mx_sb_len: u8,
    iovec_count: u16,
    dxfer_len: u32,
    dxferp: *mut libc::c_void,
    cmdp: *mut u8,
    sbp: *mut u8,
    timeout: u32,
    flags: u32,
    pack_id: i32,
    usr_ptr: *mut libc::c_void,
    status: u8,
    masked_status: u8,
    msg_status: u8,
    sb_len_wr: u8,
    host_status: u16,
    driver_status: u16,
    resid: i32,
    duration: u32,
    info: u32,
}

impl Default for SgIoHdr {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

pub fn scsi_max_length() -> u32 {
    8
}

pub fn scsi_open(name: &str) -> Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(libc::O_NONBLOCK | libc::O_RDWR)
        .open(name)
}

pub fn scsi_cmd(
    file: &File,
    cdb: &mut [u8],
    mode: ScsiIoMode,
    data: &mut [u8],
) -> Result<u8> {
    let mut hdr = SgIoHdr::default();
    
    hdr.interface_id = 'S' as i32;
    hdr.dxfer_direction = match mode {
        ScsiIoMode::Read => -3,
        ScsiIoMode::Write => -2,
    };
    hdr.cmd_len = cdb.len() as u8;
    hdr.dxfer_len = data.len() as u32;
    hdr.dxferp = data.as_mut_ptr() as *mut libc::c_void;
    hdr.cmdp = cdb.as_mut_ptr();
    hdr.timeout = !0;

    let fd = file.as_raw_fd();
    let res = unsafe {
        libc::ioctl(
            fd,
            0x2285,
            &mut hdr as *mut SgIoHdr,
        )
    };

    if res < 0 {
        Err(Error::last_os_error())
    } else {
        Ok(hdr.status & 0x3e)
    }
}
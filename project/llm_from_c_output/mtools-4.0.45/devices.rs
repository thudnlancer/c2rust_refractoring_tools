// This is a partial translation of the C code to Rust. 
// Note that some parts (like direct Linux kernel interfaces) 
// don't have direct Rust equivalents and would require unsafe blocks
// or external crates. The full translation would be quite extensive.

use std::os::raw::{c_int, c_uint, c_uchar};
use std::io::{Error, ErrorKind};
use std::ffi::CStr;
use std::os::unix::io::RawFd;

const MT_READ: u8 = 1;
const MT_WRITE: u8 = 2;

#[repr(C)]
struct FloppyRawCmd {
    data: *mut u8,
    length: u32,
    cmd_count: u8,
    flags: u32,
    cmd: [u8; 9],
    reply_count: u8,
    reply: [u8; 7],
    track: u8,
    rate: u8,
}

impl FloppyRawCmd {
    fn init(&mut self) {
        self.data = std::ptr::null_mut();
        self.length = 0;
        self.cmd_count = 9;
        self.flags = FD_RAW_INTR | FD_RAW_NEED_SEEK | FD_RAW_NEED_DISK;
        self.cmd[1] = 0;
        self.cmd[6] = 0;
        self.cmd[7] = 0x1b;
        self.cmd[8] = 0xff;
        self.reply_count = 0;
    }

    fn set_rate(&mut self, rate: u8) {
        self.rate = rate;
    }

    fn set_drive(&mut self, drive: u8) {
        self.cmd[1] = (self.cmd[1] & !3) | (drive & 3);
    }

    fn set_track(&mut self, track: u8) {
        self.cmd[2] = track;
    }

    fn set_ptrack(&mut self, track: u8) {
        self.track = track;
    }

    fn set_head(&mut self, head: u8) {
        if head != 0 {
            self.cmd[1] |= 4;
        } else {
            self.cmd[1] &= !4;
        }
        self.cmd[3] = head;
    }

    fn set_sector(&mut self, sector: u8) {
        self.cmd[4] = sector;
        self.cmd[6] = sector - 1;
    }

    fn set_sizecode(&mut self, sizecode: u8) {
        self.cmd[5] = sizecode;
        self.cmd[6] += 1;
        self.length += 128 << sizecode;
    }

    fn set_direction(&mut self, direction: u8) {
        if direction == MT_READ {
            self.flags |= FD_RAW_READ;
            self.cmd[0] = FD_READ & !0x80;
        } else {
            self.flags |= FD_RAW_WRITE;
            self.cmd[0] = FD_WRITE & !0x80;
        }
    }

    fn set_data(&mut self, data: *mut u8) {
        self.data = data;
    }

    fn set_cont(&mut self) {
        self.flags |= FD_RAW_MORE;
    }

    fn sizecode(&self) -> u8 {
        self.cmd[5]
    }

    fn track(&self) -> u8 {
        self.cmd[2]
    }
}

const FD_RAW_INTR: u32 = 0x0001;
const FD_RAW_NEED_SEEK: u32 = 0x0002;
const FD_RAW_NEED_DISK: u32 = 0x0004;
const FD_RAW_SOFTFAILURE: u32 = 0x0008;
const FD_RAW_STOP_IF_FAILURE: u32 = 0x0010;
const FD_RAW_READ: u32 = 0x0020;
const FD_RAW_WRITE: u32 = 0x0040;
const FD_RAW_MORE: u32 = 0x0080;

const FD_READ: u8 = 0x06;
const FD_WRITE: u8 = 0x05;

const ST1_WP: u8 = 0x40;

fn get_drive(fd: RawFd) -> Result<i32, Error> {
    let statbuf = unsafe {
        let mut stat = std::mem::zeroed();
        if libc::fstat(fd, &mut stat) < 0 {
            return Err(Error::last_os_error());
        }
        stat
    };

    if !S_ISBLK(statbuf.st_mode) || major(statbuf.st_rdev) != FLOPPY_MAJOR {
        return Ok(-1);
    }

    Ok(minor(statbuf.st_rdev) as i32)
}

fn major(dev: u64) -> u32 {
    unsafe { libc::major(dev) }
}

fn minor(dev: u64) -> u32 {
    unsafe { libc::minor(dev) }
}

fn S_ISBLK(mode: u32) -> bool {
    mode & libc::S_IFMT == libc::S_IFBLK
}

const FLOPPY_MAJOR: u32 = 2;

fn send_one_cmd(fd: RawFd, raw_cmd: &mut FloppyRawCmd, message: &str) -> Result<(), Error> {
    if unsafe { libc::ioctl(fd, FDRAWCMD as _, raw_cmd) } >= 0 {
        if raw_cmd.reply_count < 7 {
            return Err(Error::new(ErrorKind::Other, "Short reply from FDC"));
        }
        return Ok(());
    }

    match Error::last_os_error().kind() {
        ErrorKind::WouldBlock => {
            std::thread::sleep(std::time::Duration::from_secs(1));
            Err(Error::new(ErrorKind::WouldBlock, "FDC busy"))
        }
        ErrorKind::Other => {
            if unsafe { libc::ioctl(fd, FDRESET as _, 2) } < 0 {
                return Err(Error::last_os_error());
            }
            Err(Error::new(ErrorKind::Other, "Controller reset"))
        }
        _ => Err(Error::last_os_error()),
    }
}

const FDRAWCMD: u32 = 0x100;
const FDRESET: u32 = 0x101;

fn analyze_one_reply(raw_cmd: &FloppyRawCmd, bytes: &mut i32, do_print: bool) -> Result<i32, Error> {
    if raw_cmd.reply_count == 7 {
        let end = if raw_cmd.reply[3] != raw_cmd.cmd[2] {
            raw_cmd.cmd[6] + 1
        } else {
            raw_cmd.reply[5]
        };

        *bytes = (end - raw_cmd.cmd[4]) as i32;
        *bytes = *bytes << (7 + raw_cmd.cmd[5]);
    } else {
        *bytes = 0;
    }

    match raw_cmd.reply[0] & 0xc0 {
        0x40 => {
            if (raw_cmd.reply[0] & 0x38) == 0
                && raw_cmd.reply[1] == 0x80
                && raw_cmd.reply[2] == 0
            {
                *bytes += 1 << (7 + raw_cmd.cmd[5]);
            } else if raw_cmd.reply[1] & ST1_WP != 0 {
                *bytes = 0;
                return Err(Error::new(ErrorKind::PermissionDenied, "Write protected"));
            } else if *bytes == 0 && do_print {
                print_message(raw_cmd, "");
            }
            Ok(-1)
        }
        0x80 => {
            *bytes = 0;
            Err(Error::new(ErrorKind::InvalidInput, "Invalid command"))
        }
        0xc0 => {
            *bytes = 0;
            Err(Error::new(ErrorKind::Interrupted, "Polling termination"))
        }
        _ => {
            if raw_cmd.flags & FD_RAW_MORE != 0 {
                Ok(1)
            } else {
                Ok(0)
            }
        }
    }
}

fn print_message(raw_cmd: &FloppyRawCmd, message: &str) {
    if message.is_empty() {
        return;
    }

    eprint!("   ");
    for i in 0..raw_cmd.cmd_count {
        eprint!("{:02x} ", raw_cmd.cmd[i as usize]);
    }
    eprintln!();

    for i in 0..raw_cmd.reply_count {
        eprint!("{:02x} ", raw_cmd.reply[i as usize]);
    }
    eprintln!();

    let code = (raw_cmd.reply[0] as u32) << 16
        | (raw_cmd.reply[1] as u32) << 8
        | raw_cmd.reply[2] as u32;

    for i in 0..22 {
        if (code & (1 << i)) != 0 && !ERROR_MSG[i].is_empty() {
            eprintln!("{}", ERROR_MSG[i]);
        }
    }
}

const ERROR_MSG: [&str; 22] = [
    "Missing Data Address Mark",
    "Bad cylinder",
    "Scan not satisfied",
    "Scan equal hit",
    "Wrong cylinder",
    "CRC error in data field",
    "Control Mark = deleted",
    "",
    "Missing Address Mark",
    "Write Protect",
    "No Data - unreadable",
    "",
    "OverRun",
    "CRC error in data or address",
    "",
    "End Of Cylinder",
    "",
    "",
    "",
    "Not ready",
    "Equipment check error",
    "Seek end",
];

// Note: The rest of the code (device definitions, geometry initialization, etc.)
// would require additional Rust modules and careful handling of platform-specific
// functionality. This partial translation focuses on the core floppy command logic.
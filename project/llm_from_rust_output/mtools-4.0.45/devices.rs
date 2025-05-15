use std::{
    ffi::CString,
    io::{self, Write},
    os::unix::io::RawFd,
    ptr,
    time::Duration,
};

const FD_GET_SECTOR_SIZE: u32 = 0x1268;
const FD_GET_SIZE: u32 = 0x1260;
const HDIO_GETGEO: u32 = 0x0301;

#[derive(Debug, Clone, Copy)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct Stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: i32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atim: Timespec,
    pub st_mtim: Timespec,
    pub st_ctim: Timespec,
    pub __glibc_reserved: [i64; 3],
}

#[derive(Debug, Clone, Copy)]
pub struct Device {
    pub name: &'static str,
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
    pub data_map: Option<&'static str>,
    pub tot_sectors: u32,
    pub sector_size: u16,
    pub postcmd: Option<CString>,
    pub cfg_filename: Option<&'static str>,
}

#[derive(Debug, Clone, Copy)]
pub struct FloppyStruct {
    pub size: u32,
    pub sect: u32,
    pub head: u32,
    pub track: u32,
    pub stretch: u32,
    pub gap: u8,
    pub rate: u8,
    pub spec1: u8,
    pub fmt_gap: u8,
    pub name: Option<&'static str>,
}

#[derive(Debug, Clone, Copy)]
pub struct FloppyRawCmd {
    pub flags: u32,
    pub data: *mut libc::c_void,
    pub kernel_data: *mut libc::c_char,
    pub next: *mut FloppyRawCmd,
    pub length: i64,
    pub phys_length: i64,
    pub buffer_length: i32,
    pub rate: u8,
    pub cmd_count: u8,
    pub cmd: [u8; 16],
    pub reply_count: u8,
    pub reply: [u8; 16],
    pub track: i32,
    pub resultcode: i32,
    pub reserved1: i32,
    pub reserved2: i32,
}

pub type RawRequest = FloppyRawCmd;

#[derive(Debug, Clone, Copy)]
pub struct HdGeometry {
    pub heads: u8,
    pub sectors: u8,
    pub cylinders: u16,
    pub start: u64,
}

const ERROR_MSGS: [&str; 22] = [
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

pub const CONST_DEVICES: [Device; 5] = [
    Device {
        name: "/dev/fd0",
        drive: 'A',
        fat_bits: 0,
        mode: 0,
        tracks: 80,
        heads: 2,
        sectors: 18,
        hidden: 0,
        offset: 0,
        partition: 0,
        misc_flags: 0x10,
        ssize: 2,
        use_2m: 0,
        precmd: None,
        file_nr: 0,
        blocksize: 0,
        codepage: 0,
        data_map: None,
        tot_sectors: 0,
        sector_size: 0,
        postcmd: None,
        cfg_filename: None,
    },
    Device {
        name: "/dev/fd1",
        drive: 'B',
        fat_bits: 0,
        mode: 0,
        tracks: 0,
        heads: 0,
        sectors: 0,
        hidden: 0,
        offset: 0,
        partition: 0,
        misc_flags: 0,
        ssize: 2,
        use_2m: 0,
        precmd: None,
        file_nr: 0,
        blocksize: 0,
        codepage: 0,
        data_map: None,
        tot_sectors: 0,
        sector_size: 0,
        postcmd: None,
        cfg_filename: None,
    },
    Device {
        name: "/dev/sdb4",
        drive: 'J',
        fat_bits: 16,
        mode: 0,
        tracks: 0,
        heads: 0,
        sectors: 0,
        hidden: 0,
        offset: 0,
        partition: 0,
        misc_flags: 0x10,
        ssize: 2,
        use_2m: 0,
        precmd: None,
        file_nr: 0,
        blocksize: 0,
        codepage: 0,
        data_map: None,
        tot_sectors: 0,
        sector_size: 0,
        postcmd: None,
        cfg_filename: None,
    },
    Device {
        name: "/dev/sdb4",
        drive: 'Z',
        fat_bits: 16,
        mode: 0,
        tracks: 0,
        heads: 0,
        sectors: 0,
        hidden: 0,
        offset: 0,
        partition: 0,
        misc_flags: 0x10,
        ssize: 2,
        use_2m: 0,
        precmd: None,
        file_nr: 0,
        blocksize: 0,
        codepage: 0,
        data_map: None,
        tot_sectors: 0,
        sector_size: 0,
        postcmd: None,
        cfg_filename: None,
    },
    Device {
        name: "$DISPLAY",
        drive: 'X',
        fat_bits: 0,
        mode: 0,
        tracks: 0,
        heads: 0,
        sectors: 0,
        hidden: 0,
        offset: 0,
        partition: 0,
        misc_flags: 0x40,
        ssize: 2,
        use_2m: 0,
        precmd: None,
        file_nr: 0,
        blocksize: 0,
        codepage: 0,
        data_map: None,
        tot_sectors: 0,
        sector_size: 0,
        postcmd: None,
        cfg_filename: None,
    },
];

pub const NR_CONST_DEVICES: u32 = CONST_DEVICES.len() as u32;

fn gnu_dev_major(dev: u64) -> u32 {
    ((dev & 0xfff00) >> 8) as u32 | ((dev & 0xfffff00000000000) >> 32) as u32
}

fn gnu_dev_minor(dev: u64) -> u32 {
    (dev & 0xff) as u32 | ((dev & 0xffffff00000) >> 12) as u32
}

fn compare_geom(dev: &Device, orig_dev: Option<&Device>) -> bool {
    if let Some(orig) = orig_dev {
        if orig.misc_flags & 0x10 != 0 {
            return false;
        }
    }

    if orig_dev.is_none() 
        || orig_dev.unwrap().tracks == 0 
        || dev.tracks == 0 
    {
        return false;
    }

    orig_dev.unwrap().tracks != dev.tracks
        || orig_dev.unwrap().heads as i32 != dev.heads as i32
        || orig_dev.unwrap().sectors as i32 != dev.sectors as i32
}

fn print_message(raw_cmd: &RawRequest, message: Option<&str>) {
    if message.is_none() {
        return;
    }

    let stderr = io::stderr();
    let mut handle = stderr.lock();

    write!(handle, "   ").unwrap();
    for i in 0..raw_cmd.cmd_count as usize {
        write!(handle, "{:02x} ", raw_cmd.cmd[i]).unwrap();
    }
    writeln!(handle).unwrap();

    for i in 0..raw_cmd.reply_count as usize {
        write!(handle, "{:02x} ", raw_cmd.reply[i]).unwrap();
    }
    writeln!(handle).unwrap();

    let code = ((raw_cmd.reply[0] as i32) << 16)
        + ((raw_cmd.reply[1] as i32) << 8)
        + raw_cmd.reply[2] as i32;

    for (i, msg) in ERROR_MSGS.iter().enumerate() {
        if code & (1 << i) != 0 && !msg.is_empty() {
            writeln!(handle, "{}", msg).unwrap();
        }
    }
}

pub fn send_one_cmd(fd: RawFd, raw_cmd: &mut RawRequest, message: Option<&str>) -> i32 {
    unsafe {
        let request = 0x58 | (2 << 8);
        if libc::ioctl(fd, request as libc::c_ulong, raw_cmd) >= 0 {
            if raw_cmd.reply_count < 7 {
                eprintln!("Short reply from FDC");
                return -1;
            }
            return 0;
        }

        match io::Error::last_os_error().raw_os_error().unwrap() {
            16 => {
                eprintln!("FDC busy, sleeping for a second");
                std::thread::sleep(Duration::from_secs(1));
                1
            }
            5 => {
                eprintln!("resetting controller");
                let reset_request = 0x54 | (2 << 8);
                if libc::ioctl(fd, reset_request as libc::c_ulong, 2) < 0 {
                    perror("reset");
                    -1
                } else {
                    1
                }
            }
            _ => {
                perror(message.unwrap_or("unknown error"));
                -1
            }
        }
    }
}

fn perror(msg: &str) {
    eprintln!("{}: {}", msg, io::Error::last_os_error());
}

pub fn analyze_one_reply(raw_cmd: &RawRequest, bytes: &mut i32, do_print: bool) -> i32 {
    if raw_cmd.reply_count == 7 {
        let end = if raw_cmd.reply[3] != raw_cmd.cmd[2] {
            raw_cmd.cmd[6] as i32 + 1
        } else {
            raw_cmd.reply[5] as i32
        };
        *bytes = end - raw_cmd.cmd[4] as i32;
        *bytes <<= 7 + raw_cmd.cmd[5] as i32;
    } else {
        *bytes = 0;
    }

    match raw_cmd.reply[0] & 0xc0 {
        64 => {
            if raw_cmd.reply[0] & 0x38 == 0
                && raw_cmd.reply[1] == 0x80
                && raw_cmd.reply[2] == 0
            {
                *bytes += 1 << (7 + raw_cmd.cmd[5] as i32);
            } else {
                if raw_cmd.reply[1] & 0x2 != 0 {
                    *bytes = 0;
                    eprintln!("This disk is write protected");
                    return -1;
                }
                if *bytes == 0 && do_print {
                    print_message(raw_cmd, Some(""));
                }
                return -1;
            }
        }
        128 => {
            *bytes = 0;
            eprintln!("invalid command given");
            -1
        }
        192 => {
            *bytes = 0;
            eprintln!("abnormal termination caused by polling");
            -1
        }
        _ => 0,
    };

    if raw_cmd.flags & 0x100 != 0 {
        1
    } else {
        0
    }
}

fn set_2m(floppy: &mut FloppyStruct, value: u32) {
    floppy.rate = if value & 0x7f != 0 {
        floppy.rate | 0x4
    } else {
        floppy.rate & !0x4
    };
}

fn set_ssize(floppy: &mut FloppyStruct, value: i32) {
    let v = (((value & 7) + 6) % 8) << 3;
    floppy.rate = (floppy.rate & !0x38) | v as u8;
}

fn set_parameters(fd: RawFd, floppy: &FloppyStruct, buf: &Stat) -> i32 {
    if gnu_dev_minor(buf.st_rdev) & 0x7f > 3 {
        return 1;
    }
    unsafe {
        let request = 0x42 | (2 << 8) | (std::mem::size_of::<FloppyStruct>() as u32) << (8 + 8);
        libc::ioctl(fd, request as libc::c_ulong, floppy)
    }
}

fn get_parameters(fd: RawFd, floppy: &mut FloppyStruct) -> i32 {
    unsafe {
        let request = 0x4 | (2 << 8) | (std::mem::size_of::<FloppyStruct>() as u32) << (8 + 8);
        libc::ioctl(fd, request as libc::c_ulong, floppy)
    }
}

fn ulong_to_sectors(raw_sect: u64) -> u32 {
    if raw_sect > (i64::MAX as u64).wrapping_mul(2).wrapping_add(1) {
        eprintln!("Too many sectors for FAT {:8x}", raw_sect);
        std::process::exit(1);
    }
    raw_sect as u32
}

pub fn get_sector_size(fd: RawFd) -> i32 {
    let mut sec_size = 0;
    unsafe {
        if libc::ioctl(fd, FD_GET_SECTOR_SIZE as libc::c_ulong, &mut sec_size) != 0 || sec_size <= 0 {
            perror("Could not get sector size of device");
            return -1;
        }
    }
    if sec_size > 4096 {
        sec_size = 4096;
    }
    sec_size
}

fn get_block_geom(fd: RawFd, dev: &mut Device) -> i32 {
    let mut geom = HdGeometry {
        heads: 0,
        sectors: 0,
        cylinders: 0,
        start: 0,
    };
    let mut size = 0u64;

    unsafe {
        if libc::ioctl(fd, HDIO_GETGEO as libc::c_ulong, &mut geom) < 0 {
            perror("Could not get geometry of device");
            return -1;
        }

        if libc::ioctl(fd, FD_GET_SIZE as libc::c_ulong, &mut size) < 0 {
            perror("Could not get size of device");
            return -1;
        }
    }

    let sec_size = get_sector_size(fd);
    if sec_size < 0 {
        return -1;
    }

    dev.ssize = 0;
    while dev.ssize < 0x7f && (128 << dev.ssize) < sec_size as u8 {
        dev.ssize += 1;
    }

    if dev.heads == 0 {
        dev.heads = geom.heads as u16;
    }
    if dev.sectors == 0 {
        dev.sectors = geom.sectors as u16;
    }

    let sect_per_track = (dev.heads as u32) * (dev.sectors as u32);
    if dev.hidden == 0 {
        let hidden = (geom.start % sect_per_track as u64) as u32;
        if hidden != 0 && hidden != dev.sectors as u32 {
            eprintln!(
                "Hidden ({}) does not match sectors ({})",
                hidden,
                dev.sectors
            );
            return -1;
        }
        dev.hidden = hidden;
    }

    if dev.tracks == 0 {
        dev.tracks = ulong_to_sectors(
            (size + (dev.hidden % sect_per_track) as u64) / sect_per_track as u64
        );
    }

    0
}

pub fn init_geom(fd: RawFd, dev: &mut Device, orig_dev: Option<&Device>, statbuf: &Stat) -> i32 {
    if statbuf.st_mode & 0o170000 != 0o60000 || gnu_dev_major(statbuf.st_rdev) != 2 {
        if get_block_geom(fd, dev) < 0 {
            return -1;
        }
        return compare_geom(dev, orig_dev) as i32;
    }

    if statbuf.st_mode &
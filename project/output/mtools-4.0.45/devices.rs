#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn exit(_: i32) -> !;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
    fn sleep(__seconds: u32) -> u32;
    fn ioctl(__fd: i32, __request: u64, _: ...) -> i32;
    fn __errno_location() -> *mut i32;
    fn strerror(_: i32) -> *mut i8;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
pub type off_t = __off_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type mt_off_t = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device {
    pub name: *const i8,
    pub drive: i8,
    pub fat_bits: i32,
    pub mode: i32,
    pub tracks: u32,
    pub heads: uint16_t,
    pub sectors: uint16_t,
    pub hidden: u32,
    pub offset: mt_off_t,
    pub partition: u32,
    pub misc_flags: u32,
    pub ssize: uint8_t,
    pub use_2m: u32,
    pub precmd: *mut i8,
    pub file_nr: i32,
    pub blocksize: u32,
    pub codepage: u32,
    pub data_map: *const i8,
    pub tot_sectors: uint32_t,
    pub sector_size: uint16_t,
    pub postcmd: *mut i8,
    pub cfg_filename: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floppy_struct {
    pub size: u32,
    pub sect: u32,
    pub head: u32,
    pub track: u32,
    pub stretch: u32,
    pub gap: u8,
    pub rate: u8,
    pub spec1: u8,
    pub fmt_gap: u8,
    pub name: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floppy_raw_cmd {
    pub flags: u32,
    pub data: *mut libc::c_void,
    pub kernel_data: *mut i8,
    pub next: *mut floppy_raw_cmd,
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
pub type RawRequest_t = floppy_raw_cmd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hd_geometry {
    pub heads: u8,
    pub sectors: u8,
    pub cylinders: libc::c_ushort,
    pub start: u64,
}
#[inline]
unsafe extern "C" fn gnu_dev_major(mut __dev: __dev_t) -> u32 {
    let mut __major: u32 = 0;
    __major = ((__dev & 0xfff00 as u32 as __dev_t) >> 8 as i32) as u32;
    __major = (__major as u64 | (__dev & 0xfffff00000000000 as u64) >> 32 as i32) as u32;
    return __major;
}
#[inline]
unsafe extern "C" fn gnu_dev_minor(mut __dev: __dev_t) -> u32 {
    let mut __minor: u32 = 0;
    __minor = ((__dev & 0xff as u32 as __dev_t) >> 0 as i32) as u32;
    __minor = (__minor as u64 | (__dev & 0xffffff00000 as u64) >> 12 as i32) as u32;
    return __minor;
}
unsafe extern "C" fn compare_geom(
    mut dev: *mut device,
    mut orig_dev: *mut device,
) -> i32 {
    if !orig_dev.is_null() && (*orig_dev).misc_flags & 0x10 as u32 != 0 {
        return 0 as i32;
    }
    if orig_dev.is_null() || (*orig_dev).tracks == 0 || dev.is_null()
        || (*dev).tracks == 0
    {
        return 0 as i32;
    }
    return ((*orig_dev).tracks != (*dev).tracks
        || (*orig_dev).heads as i32 != (*dev).heads as i32
        || (*orig_dev).sectors as i32 != (*dev).sectors as i32) as i32;
}
static mut error_msg: [*const i8; 22] = [
    b"Missing Data Address Mark\0" as *const u8 as *const i8,
    b"Bad cylinder\0" as *const u8 as *const i8,
    b"Scan not satisfied\0" as *const u8 as *const i8,
    b"Scan equal hit\0" as *const u8 as *const i8,
    b"Wrong cylinder\0" as *const u8 as *const i8,
    b"CRC error in data field\0" as *const u8 as *const i8,
    b"Control Mark = deleted\0" as *const u8 as *const i8,
    0 as *const i8,
    b"Missing Address Mark\0" as *const u8 as *const i8,
    b"Write Protect\0" as *const u8 as *const i8,
    b"No Data - unreadable\0" as *const u8 as *const i8,
    0 as *const i8,
    b"OverRun\0" as *const u8 as *const i8,
    b"CRC error in data or address\0" as *const u8 as *const i8,
    0 as *const i8,
    b"End Of Cylinder\0" as *const u8 as *const i8,
    0 as *const i8,
    0 as *const i8,
    0 as *const i8,
    b"Not ready\0" as *const u8 as *const i8,
    b"Equipment check error\0" as *const u8 as *const i8,
    b"Seek end\0" as *const u8 as *const i8,
];
#[inline]
unsafe extern "C" fn print_message(
    mut raw_cmd: *mut RawRequest_t,
    mut message: *const i8,
) {
    let mut i: i32 = 0;
    let mut code: i32 = 0;
    if message.is_null() {
        return;
    }
    fprintf(stderr, b"   \0" as *const u8 as *const i8);
    i = 0 as i32;
    while i < (*raw_cmd).cmd_count as i32 {
        fprintf(
            stderr,
            b"%2.2x \0" as *const u8 as *const i8,
            (*raw_cmd).cmd[i as usize] as i32,
        );
        i += 1;
        i;
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    i = 0 as i32;
    while i < (*raw_cmd).reply_count as i32 {
        fprintf(
            stderr,
            b"%2.2x \0" as *const u8 as *const i8,
            (*raw_cmd).reply[i as usize] as i32,
        );
        i += 1;
        i;
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    code = (((*raw_cmd).reply[0 as i32 as usize] as i32) << 16 as i32)
        + (((*raw_cmd).reply[1 as i32 as usize] as i32) << 8 as i32)
        + (*raw_cmd).reply[2 as i32 as usize] as i32;
    i = 0 as i32;
    while i < 22 as i32 {
        if code & (1 as i32) << i != 0 && !(error_msg[i as usize]).is_null() {
            fprintf(stderr, b"%s\n\0" as *const u8 as *const i8, error_msg[i as usize]);
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn send_one_cmd(
    mut fd: i32,
    mut raw_cmd: *mut RawRequest_t,
    mut message: *const i8,
) -> i32 {
    if ioctl(
        fd,
        ((0 as u32) << 0 as i32 + 8 as i32 + 8 as i32 + 14 as i32
            | ((2 as i32) << 0 as i32 + 8 as i32) as u32
            | ((0x58 as i32) << 0 as i32) as u32
            | ((0 as i32) << 0 as i32 + 8 as i32 + 8 as i32) as u32) as u64,
        raw_cmd,
    ) >= 0 as i32
    {
        if ((*raw_cmd).reply_count as i32) < 7 as i32 {
            fprintf(stderr, b"Short reply from FDC\n\0" as *const u8 as *const i8);
            return -(1 as i32);
        }
        return 0 as i32;
    }
    match *__errno_location() {
        16 => {
            fprintf(
                stderr,
                b"FDC busy, sleeping for a second\n\0" as *const u8 as *const i8,
            );
            sleep(1 as i32 as u32);
            return 1 as i32;
        }
        5 => {
            fprintf(stderr, b"resetting controller\n\0" as *const u8 as *const i8);
            if ioctl(
                fd,
                ((0 as u32) << 0 as i32 + 8 as i32 + 8 as i32 + 14 as i32
                    | ((2 as i32) << 0 as i32 + 8 as i32) as u32
                    | ((0x54 as i32) << 0 as i32) as u32
                    | ((0 as i32) << 0 as i32 + 8 as i32 + 8 as i32) as u32) as u64,
                2 as i32,
            ) < 0 as i32
            {
                perror(b"reset\0" as *const u8 as *const i8);
                return -(1 as i32);
            }
            return 1 as i32;
        }
        _ => {
            perror(message);
            return -(1 as i32);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn analyze_one_reply(
    mut raw_cmd: *mut RawRequest_t,
    mut bytes: *mut i32,
    mut do_print: i32,
) -> i32 {
    if (*raw_cmd).reply_count as i32 == 7 as i32 {
        let mut end: i32 = 0;
        if (*raw_cmd).reply[3 as i32 as usize] as i32
            != (*raw_cmd).cmd[2 as i32 as usize] as i32
        {
            end = (*raw_cmd).cmd[6 as i32 as usize] as i32 + 1 as i32;
        } else {
            end = (*raw_cmd).reply[5 as i32 as usize] as i32;
        }
        *bytes = end - (*raw_cmd).cmd[4 as i32 as usize] as i32;
        *bytes = *bytes << 7 as i32 + (*raw_cmd).cmd[5 as i32 as usize] as i32;
    } else {
        *bytes = 0 as i32;
    }
    match (*raw_cmd).reply[0 as i32 as usize] as i32 & 0xc0 as i32 {
        64 => {
            if (*raw_cmd).reply[0 as i32 as usize] as i32 & 0x38 as i32 == 0 as i32
                && (*raw_cmd).reply[1 as i32 as usize] as i32 == 0x80 as i32
                && (*raw_cmd).reply[2 as i32 as usize] as i32 == 0 as i32
            {
                *bytes
                    += (1 as i32) << 7 as i32 + (*raw_cmd).cmd[5 as i32 as usize] as i32;
            } else {
                if (*raw_cmd).reply[1 as i32 as usize] as i32 & 0x2 as i32 != 0 {
                    *bytes = 0 as i32;
                    fprintf(
                        stderr,
                        b"This disk is write protected\n\0" as *const u8 as *const i8,
                    );
                    return -(1 as i32);
                }
                if *bytes == 0 && do_print != 0 {
                    print_message(raw_cmd, b"\0" as *const u8 as *const i8);
                }
                return -(1 as i32);
            }
        }
        128 => {
            *bytes = 0 as i32;
            fprintf(stderr, b"invalid command given\n\0" as *const u8 as *const i8);
            return -(1 as i32);
        }
        192 => {
            *bytes = 0 as i32;
            fprintf(
                stderr,
                b"abnormal termination caused by polling\n\0" as *const u8 as *const i8,
            );
            return -(1 as i32);
        }
        _ => {}
    }
    if (*raw_cmd).flags & 0x100 as i32 as u32 != 0 {
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub static mut const_devices: [device; 5] = [
    {
        let mut init = device {
            name: b"/dev/fd0\0" as *const u8 as *const i8,
            drive: 'A' as i32 as i8,
            fat_bits: 0 as i32,
            mode: 0 as i32,
            tracks: 80 as i32 as u32,
            heads: 2 as i32 as uint16_t,
            sectors: 18 as i32 as uint16_t,
            hidden: 0 as i32 as u32,
            offset: 0 as i64,
            partition: 0 as i32 as u32,
            misc_flags: 0x10 as u32,
            ssize: 0x2 as i32 as uint8_t,
            use_2m: 0 as i32 as u32,
            precmd: 0 as *const i8 as *mut i8,
            file_nr: 0 as i32,
            blocksize: 0 as i32 as u32,
            codepage: 0 as i32 as u32,
            data_map: 0 as *const i8,
            tot_sectors: 0 as i32 as uint32_t,
            sector_size: 0 as i32 as uint16_t,
            postcmd: 0 as *const i8 as *mut i8,
            cfg_filename: 0 as *const i8,
        };
        init
    },
    {
        let mut init = device {
            name: b"/dev/fd1\0" as *const u8 as *const i8,
            drive: 'B' as i32 as i8,
            fat_bits: 0 as i32,
            mode: 0 as i32,
            tracks: 0 as i32 as u32,
            heads: 0 as i32 as uint16_t,
            sectors: 0 as i32 as uint16_t,
            hidden: 0 as i32 as u32,
            offset: 0 as i64,
            partition: 0 as i32 as u32,
            misc_flags: 0 as i32 as u32,
            ssize: 0x2 as i32 as uint8_t,
            use_2m: 0 as i32 as u32,
            precmd: 0 as *const i8 as *mut i8,
            file_nr: 0 as i32,
            blocksize: 0 as i32 as u32,
            codepage: 0 as i32 as u32,
            data_map: 0 as *const i8,
            tot_sectors: 0 as i32 as uint32_t,
            sector_size: 0 as i32 as uint16_t,
            postcmd: 0 as *const i8 as *mut i8,
            cfg_filename: 0 as *const i8,
        };
        init
    },
    {
        let mut init = device {
            name: b"/dev/sdb4\0" as *const u8 as *const i8,
            drive: 'J' as i32 as i8,
            fat_bits: 16 as i32,
            mode: 0 as i32,
            tracks: 0 as i32 as u32,
            heads: 0 as i32 as uint16_t,
            sectors: 0 as i32 as uint16_t,
            hidden: 0 as i32 as u32,
            offset: 0 as i64,
            partition: 0 as i32 as u32,
            misc_flags: 0x10 as u32,
            ssize: 0x2 as i32 as uint8_t,
            use_2m: 0 as i32 as u32,
            precmd: 0 as *const i8 as *mut i8,
            file_nr: 0 as i32,
            blocksize: 0 as i32 as u32,
            codepage: 0 as i32 as u32,
            data_map: 0 as *const i8,
            tot_sectors: 0 as i32 as uint32_t,
            sector_size: 0 as i32 as uint16_t,
            postcmd: 0 as *const i8 as *mut i8,
            cfg_filename: 0 as *const i8,
        };
        init
    },
    {
        let mut init = device {
            name: b"/dev/sdb4\0" as *const u8 as *const i8,
            drive: 'Z' as i32 as i8,
            fat_bits: 16 as i32,
            mode: 0 as i32,
            tracks: 0 as i32 as u32,
            heads: 0 as i32 as uint16_t,
            sectors: 0 as i32 as uint16_t,
            hidden: 0 as i32 as u32,
            offset: 0 as i64,
            partition: 0 as i32 as u32,
            misc_flags: 0x10 as u32,
            ssize: 0x2 as i32 as uint8_t,
            use_2m: 0 as i32 as u32,
            precmd: 0 as *const i8 as *mut i8,
            file_nr: 0 as i32,
            blocksize: 0 as i32 as u32,
            codepage: 0 as i32 as u32,
            data_map: 0 as *const i8,
            tot_sectors: 0 as i32 as uint32_t,
            sector_size: 0 as i32 as uint16_t,
            postcmd: 0 as *const i8 as *mut i8,
            cfg_filename: 0 as *const i8,
        };
        init
    },
    {
        let mut init = device {
            name: b"$DISPLAY\0" as *const u8 as *const i8,
            drive: 'X' as i32 as i8,
            fat_bits: 0 as i32,
            mode: 0 as i32,
            tracks: 0 as i32 as u32,
            heads: 0 as i32 as uint16_t,
            sectors: 0 as i32 as uint16_t,
            hidden: 0 as i32 as u32,
            offset: 0 as i64,
            partition: 0 as i32 as u32,
            misc_flags: 0x40 as u32,
            ssize: 0x2 as i32 as uint8_t,
            use_2m: 0 as i32 as u32,
            precmd: 0 as *const i8 as *mut i8,
            file_nr: 0 as i32,
            blocksize: 0 as i32 as u32,
            codepage: 0 as i32 as u32,
            data_map: 0 as *const i8,
            tot_sectors: 0 as i32 as uint32_t,
            sector_size: 0 as i32 as uint16_t,
            postcmd: 0 as *const i8 as *mut i8,
            cfg_filename: 0 as *const i8,
        };
        init
    },
];
#[inline]
unsafe extern "C" fn set_2m(mut floppy: *mut floppy_struct, mut value: u32) {
    let mut v: uint8_t = 0;
    if value & 0x7f as i32 as u32 != 0 {
        v = 0x4 as i32 as uint8_t;
    } else {
        v = 0 as i32 as uint8_t;
    }
    (*floppy).rate = ((*floppy).rate as i32 & !(0x4 as i32) | v as i32) as u8;
}
#[inline]
unsafe extern "C" fn set_ssize(mut floppy: *mut floppy_struct, mut value: i32) {
    let mut v: uint8_t = ((((value & 7 as i32) + 6 as i32) % 8 as i32) << 3 as i32)
        as uint8_t;
    (*floppy).rate = ((*floppy).rate as i32 & !(0x38 as i32) | v as i32) as u8;
}
#[inline]
unsafe extern "C" fn set_parameters(
    mut fd: i32,
    mut floppy: *mut floppy_struct,
    mut buf: *mut stat,
) -> i32 {
    if gnu_dev_minor((*buf).st_rdev) & 0x7f as i32 as u32 > 3 as i32 as u32 {
        return 1 as i32;
    }
    return ioctl(
        fd,
        ((1 as u32) << 0 as i32 + 8 as i32 + 8 as i32 + 14 as i32
            | ((2 as i32) << 0 as i32 + 8 as i32) as u32
            | ((0x42 as i32) << 0 as i32) as u32) as u64
            | (::core::mem::size_of::<floppy_struct>() as u64)
                << 0 as i32 + 8 as i32 + 8 as i32,
        floppy,
    );
}
#[inline]
unsafe extern "C" fn get_parameters(mut fd: i32, mut floppy: *mut floppy_struct) -> i32 {
    return ioctl(
        fd,
        ((2 as u32) << 0 as i32 + 8 as i32 + 8 as i32 + 14 as i32
            | ((2 as i32) << 0 as i32 + 8 as i32) as u32
            | ((0x4 as i32) << 0 as i32) as u32) as u64
            | (::core::mem::size_of::<floppy_struct>() as u64)
                << 0 as i32 + 8 as i32 + 8 as i32,
        floppy,
    );
}
unsafe extern "C" fn ulong_to_sectors(mut raw_sect: u64) -> uint32_t {
    if raw_sect
        > (9223372036854775807 as i64 as u64)
            .wrapping_mul(2 as u64)
            .wrapping_add(1 as u64)
    {
        fprintf(
            stderr,
            b"Too many sectors for FAT %8lx\n\0" as *const u8 as *const i8,
            raw_sect,
        );
        exit(1 as i32);
    }
    return raw_sect as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn get_sector_size(mut fd: i32) -> i32 {
    let mut sec_size: i32 = 0;
    if ioctl(
        fd,
        ((0 as u32) << 0 as i32 + 8 as i32 + 8 as i32 + 14 as i32
            | ((0x12 as i32) << 0 as i32 + 8 as i32) as u32
            | ((104 as i32) << 0 as i32) as u32
            | ((0 as i32) << 0 as i32 + 8 as i32 + 8 as i32) as u32) as u64,
        &mut sec_size as *mut i32,
    ) != 0 as i32 || sec_size <= 0 as i32
    {
        fprintf(
            stderr,
            b"Could not get sector size of device (%s)\0" as *const u8 as *const i8,
            strerror(*__errno_location()),
        );
        return -(1 as i32);
    }
    if sec_size > 4096 as i32 {
        sec_size = 4096 as i32;
    }
    return sec_size;
}
unsafe extern "C" fn get_block_geom(mut fd: i32, mut dev: *mut device) -> i32 {
    let mut geom: hd_geometry = hd_geometry {
        heads: 0,
        sectors: 0,
        cylinders: 0,
        start: 0,
    };
    let mut sec_size: i32 = 0;
    let mut size: u64 = 0;
    let mut heads: uint16_t = (*dev).heads;
    let mut sectors: uint16_t = (*dev).sectors;
    let mut sect_per_track: uint32_t = 0;
    if ioctl(fd, 0x301 as i32 as u64, &mut geom as *mut hd_geometry) < 0 as i32 {
        fprintf(
            stderr,
            b"Could not get geometry of device (%s)\0" as *const u8 as *const i8,
            strerror(*__errno_location()),
        );
        return -(1 as i32);
    }
    if ioctl(
        fd,
        ((0 as u32) << 0 as i32 + 8 as i32 + 8 as i32 + 14 as i32
            | ((0x12 as i32) << 0 as i32 + 8 as i32) as u32
            | ((96 as i32) << 0 as i32) as u32
            | ((0 as i32) << 0 as i32 + 8 as i32 + 8 as i32) as u32) as u64,
        &mut size as *mut u64,
    ) < 0 as i32
    {
        fprintf(
            stderr,
            b"Could not get size of device (%s)\0" as *const u8 as *const i8,
            strerror(*__errno_location()),
        );
        return -(1 as i32);
    }
    sec_size = get_sector_size(fd);
    if sec_size < 0 as i32 {
        return -(1 as i32);
    }
    (*dev).ssize = 0 as i32 as uint8_t;
    while ((*dev).ssize as i32) < 0x7f as i32
        && ((128 as i32) << (*dev).ssize as i32) < sec_size
    {
        (*dev).ssize = ((*dev).ssize).wrapping_add(1);
        (*dev).ssize;
    }
    if heads == 0 {
        heads = geom.heads as uint16_t;
    }
    if sectors == 0 {
        sectors = geom.sectors as uint16_t;
    }
    sect_per_track = (heads as i32 * sectors as i32) as uint32_t;
    if (*dev).hidden == 0 {
        let mut hidden: uint32_t = 0;
        hidden = (geom.start).wrapping_rem(sect_per_track as u64) as uint32_t;
        if hidden != 0 && hidden != sectors as u32 {
            fprintf(
                stderr,
                b"Hidden (%d) does not match sectors (%d)\n\0" as *const u8 as *const i8,
                hidden,
                sectors as i32,
            );
            return -(1 as i32);
        }
        (*dev).hidden = hidden;
    }
    (*dev).heads = heads;
    (*dev).sectors = sectors;
    if (*dev).tracks == 0 {
        (*dev).tracks = ulong_to_sectors(
            size
                .wrapping_add(((*dev).hidden).wrapping_rem(sect_per_track) as u64)
                .wrapping_div(sect_per_track as u64),
        );
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn init_geom(
    mut fd: i32,
    mut dev: *mut device,
    mut orig_dev: *mut device,
    mut statbuf: *mut stat,
) -> i32 {
    let mut floppy: floppy_struct = floppy_struct {
        size: 0,
        sect: 0,
        head: 0,
        track: 0,
        stretch: 0,
        gap: 0,
        rate: 0,
        spec1: 0,
        fmt_gap: 0,
        name: 0 as *const i8,
    };
    let mut change: i32 = 0;
    if (*statbuf).st_mode & 0o170000 as i32 as u32 == 0o60000 as i32 as u32
        && gnu_dev_major((*statbuf).st_rdev) != 2 as i32 as u32
    {
        get_block_geom(fd, dev);
        return compare_geom(dev, orig_dev);
    }
    if !((*statbuf).st_mode & 0o170000 as i32 as u32 == 0o60000 as i32 as u32
        && gnu_dev_major((*statbuf).st_rdev) == 2 as i32 as u32)
    {
        return compare_geom(dev, orig_dev);
    }
    if get_parameters(fd, &mut floppy) != 0 {
        return 1 as i32;
    }
    change = 0 as i32;
    if (*dev).sectors as i32 != 0 && (*dev).sectors as u32 != floppy.sect {
        floppy.sect = (*dev).sectors as u32;
        change = 1 as i32;
    } else {
        (*dev).sectors = floppy.sect as uint16_t;
    }
    if (*dev).heads as i32 != 0 && (*dev).heads as u32 != floppy.head {
        floppy.head = (*dev).heads as u32;
        change = 1 as i32;
    } else {
        (*dev).heads = floppy.head as uint16_t;
    }
    if (*dev).tracks != 0 && (*dev).tracks != floppy.track {
        floppy.track = (*dev).tracks;
        change = 1 as i32;
    } else {
        (*dev).tracks = floppy.track;
    }
    if (*dev).use_2m != 0
        && (*dev).use_2m
            != (if floppy.rate as i32 & 0x4 as i32 != 0 {
                0xff as i32
            } else {
                0x80 as i32
            }) as u32
    {
        set_2m(&mut floppy, (*dev).use_2m);
        change = 1 as i32;
    } else {
        (*dev).use_2m = (if floppy.rate as i32 & 0x4 as i32 != 0 {
            0xff as i32
        } else {
            0x80 as i32
        }) as u32;
    }
    if (*dev).ssize as i32 & 0x80 as i32 == 0 {
        (*dev).ssize = 0 as i32 as uint8_t;
    }
    if (*dev).ssize as i32 != 0
        && (*dev).ssize as i32
            != (((floppy.rate as i32 & 0x38 as i32) >> 3 as i32) + 2 as i32) % 8 as i32
                + 128 as i32
    {
        set_ssize(&mut floppy, (*dev).ssize as i32);
        change = 1 as i32;
    } else {
        (*dev).ssize = ((((floppy.rate as i32 & 0x38 as i32) >> 3 as i32) + 2 as i32)
            % 8 as i32) as uint8_t;
    }
    if change == 0 {
        return 0 as i32;
    }
    floppy.size = (((*dev).sectors as i32 * (*dev).heads as i32) as u32)
        .wrapping_mul((*dev).tracks);
    if (*dev).tracks > 41 as i32 as u32 {
        floppy.stretch = 0 as i32 as u32;
    } else {
        floppy.stretch = 1 as i32 as u32;
    }
    return set_parameters(fd, &mut floppy, statbuf);
}
#[no_mangle]
pub static mut nr_const_devices: u32 = 0;
unsafe extern "C" fn run_static_initializers() {
    nr_const_devices = (::core::mem::size_of::<[device; 5]>() as u64)
        .wrapping_div(::core::mem::size_of::<device>() as u64) as u32;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
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
    fn printf(_: *const i8, _: ...) -> i32;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn perror(__s: *const i8);
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn close(__fd: i32) -> i32;
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn getopt(___argc: i32, ___argv: *const *mut i8, __shortopts: *const i8) -> i32;
    fn getpass(__prompt: *const i8) -> *mut i8;
    fn ioctl(__fd: i32, __request: u64, _: ...) -> i32;
    fn setmntent(__file: *const i8, __mode: *const i8) -> *mut FILE;
    fn getmntent(__stream: *mut FILE) -> *mut mntent;
    fn endmntent(__stream: *mut FILE) -> i32;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn __errno_location() -> *mut i32;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn strncasecmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn set_cmd_line_image(img: *mut i8);
    fn postcmd(cmd: *const i8);
    fn drop_privs();
    fn reclaim_privs();
    fn helpFlag(_: i32, _: *mut *mut i8) -> i32;
    fn get_real_uid() -> uid_t;
    fn precmd(dev: *mut device);
    static mut mversion: *const i8;
    static mut mdate: *const i8;
    fn expand(_: *const i8, _: *mut i8) -> *const i8;
    fn closeExec(fd: i32);
    static mut devices: *mut device;
    static mut progname: *const i8;
    fn scsi_cmd(
        fd: i32,
        cdb: *mut u8,
        clen: uint8_t,
        mode: scsi_io_mode_t,
        data: *mut libc::c_void,
        len: uint32_t,
        extra_data: *mut libc::c_void,
    ) -> i32;
    fn scsi_open(
        name: *const i8,
        flags: i32,
        mode: i32,
        extra_data: *mut *mut libc::c_void,
    ) -> i32;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = i32;
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
pub type uid_t = __uid_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mntent {
    pub mnt_fsname: *mut i8,
    pub mnt_dir: *mut i8,
    pub mnt_type: *mut i8,
    pub mnt_opts: *mut i8,
    pub mnt_freq: i32,
    pub mnt_passno: i32,
}
pub type mt_off_t = off_t;
pub type device_t = device;
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
pub type scsi_io_mode_t = u32;
pub const SCSI_IO_WRITE: scsi_io_mode_t = 1;
pub const SCSI_IO_READ: scsi_io_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub type_0: i8,
    pub type_modifier: i8,
    pub scsi_version: i8,
    pub data_format: i8,
    pub length: i8,
    pub reserved1: [i8; 2],
    pub capabilities: i8,
    pub vendor: [i8; 8],
    pub product: [i8; 16],
    pub revision: [i8; 4],
    pub vendor_specific: [i8; 20],
    pub reserved2: [i8; 40],
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
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn ch_toupper(mut ch: i8) -> i8 {
    return ({
        let mut __res: i32 = 0;
        if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
            if 0 != 0 {
                let mut __c: i32 = ch as u8 as i32;
                __res = if __c < -(128 as i32) || __c > 255 as i32 {
                    __c
                } else {
                    *(*__ctype_toupper_loc()).offset(__c as isize)
                };
            } else {
                __res = toupper(ch as u8 as i32);
            }
        } else {
            __res = *(*__ctype_toupper_loc()).offset(ch as u8 as i32 as isize);
        }
        __res
    }) as i8;
}
unsafe extern "C" fn zip_cmd(
    mut priv_0: i32,
    mut fd: i32,
    mut cdb: *mut u8,
    mut clen: uint8_t,
    mut mode: scsi_io_mode_t,
    mut data: *mut libc::c_void,
    mut len: uint32_t,
    mut extra_data: *mut libc::c_void,
) -> i32 {
    let mut r: i32 = 0;
    if priv_0 != 0 {
        reclaim_privs();
    }
    r = scsi_cmd(fd, cdb, clen, mode, data, len, extra_data);
    if priv_0 != 0 {
        drop_privs();
    }
    return r;
}
unsafe extern "C" fn test_mounted(mut dev: *mut i8) -> i32 {
    let mut mnt: *mut mntent = 0 as *mut mntent;
    let mut st_dev: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut st_mnt: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut mtab: *mut FILE = 0 as *mut FILE;
    if stat(dev, &mut st_dev) != 0 {
        fprintf(
            stderr,
            b"%s: stat(%s) failed: %s.\n\0" as *const u8 as *const i8,
            progname,
            dev,
            strerror(*__errno_location()),
        );
        exit(1 as i32);
    }
    if !(st_dev.st_mode & 0o170000 as i32 as u32 == 0o60000 as i32 as u32) {
        return 0 as i32;
    }
    mtab = setmntent(
        b"/etc/mtab\0" as *const u8 as *const i8,
        b"r\0" as *const u8 as *const i8,
    );
    if mtab.is_null() {
        fprintf(
            stderr,
            b"%s: can't open %s.\n\0" as *const u8 as *const i8,
            progname,
            b"/etc/mtab\0" as *const u8 as *const i8,
        );
        exit(1 as i32);
    }
    loop {
        mnt = getmntent(mtab);
        if mnt.is_null() {
            break;
        }
        if ((*mnt).mnt_fsname).is_null()
            || strcmp((*mnt).mnt_type, b"swap\0" as *const u8 as *const i8) == 0
            || strcmp((*mnt).mnt_type, b"nfs\0" as *const u8 as *const i8) == 0
            || strcmp((*mnt).mnt_type, b"proc\0" as *const u8 as *const i8) == 0
            || strcmp((*mnt).mnt_type, b"smbfs\0" as *const u8 as *const i8) == 0
            || strcmp((*mnt).mnt_type, b"ignore\0" as *const u8 as *const i8) == 0
        {
            continue;
        }
        if stat((*mnt).mnt_fsname, &mut st_mnt) != 0 {
            continue;
        }
        if st_mnt.st_mode & 0o170000 as i32 as u32 == 0o60000 as i32 as u32 {
            if gnu_dev_major(st_mnt.st_rdev) == gnu_dev_major(st_dev.st_rdev)
                && gnu_dev_minor(st_mnt.st_rdev) >= gnu_dev_minor(st_dev.st_rdev)
                && gnu_dev_minor(st_mnt.st_rdev)
                    <= (gnu_dev_minor(st_dev.st_rdev)).wrapping_add(15 as i32 as u32)
            {
                fprintf(
                    stderr,
                    b"Device %s%d is mounted on %s.\n\0" as *const u8 as *const i8,
                    dev,
                    (gnu_dev_minor(st_mnt.st_rdev))
                        .wrapping_sub(gnu_dev_minor(st_dev.st_rdev)),
                    (*mnt).mnt_dir,
                );
                endmntent(mtab);
                return 1 as i32;
            }
        }
    }
    endmntent(mtab);
    return 0 as i32;
}
unsafe extern "C" fn usage(mut ret: i32) -> ! {
    fprintf(
        stderr,
        b"Mtools version %s, dated %s\n\0" as *const u8 as *const i8,
        mversion,
        mdate,
    );
    fprintf(
        stderr,
        b"Usage: %s [-V] [-q] [-e] [-u] [-r|-w|-p|-x] [drive:]\n\t-q print status\n\t-e eject disk\n\t-f eject disk even when mounted\n\t-r write protected (read-only)\n\t-w not write-protected (read-write)\n\t-p password write protected\n\t-x password protected\n\t-u unprotect till disk ejecting\n\0"
            as *const u8 as *const i8,
        progname,
    );
    exit(ret);
}
unsafe extern "C" fn get_zip_status(
    mut priv_0: i32,
    mut fd: i32,
    mut extra_data: *mut libc::c_void,
) -> uint8_t {
    let mut status: [u8; 128] = [0; 128];
    let mut cdb: [u8; 6] = [
        0x6 as i32 as u8,
        0 as i32 as u8,
        0x2 as i32 as u8,
        0 as i32 as u8,
        ::core::mem::size_of::<[u8; 128]>() as u64 as u8,
        0 as i32 as u8,
    ];
    if zip_cmd(
        priv_0,
        fd,
        cdb.as_mut_ptr(),
        6 as i32 as uint8_t,
        SCSI_IO_READ,
        status.as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[u8; 128]>() as u64 as uint32_t,
        extra_data,
    ) == -(1 as i32)
    {
        perror(b"status: \0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    return (status[21 as i32 as usize] as i32 & 0xf as i32) as uint8_t;
}
unsafe extern "C" fn short_command(
    mut priv_0: i32,
    mut fd: i32,
    mut cmd1: uint8_t,
    mut cmd2: uint8_t,
    mut cmd3: uint8_t,
    mut data: *const i8,
    mut extra_data: *mut libc::c_void,
) -> i32 {
    let mut cdb: [uint8_t; 6] = [
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
        0 as i32 as uint8_t,
    ];
    cdb[0 as i32 as usize] = cmd1;
    cdb[1 as i32 as usize] = cmd2;
    cdb[4 as i32 as usize] = cmd3;
    return zip_cmd(
        priv_0,
        fd,
        cdb.as_mut_ptr(),
        6 as i32 as uint8_t,
        SCSI_IO_WRITE,
        data as *mut i8 as *mut libc::c_void,
        if !data.is_null() { strlen(data) as uint32_t } else { 0 as i32 as u32 },
        extra_data,
    );
}
unsafe extern "C" fn iomega_command(
    mut priv_0: i32,
    mut fd: i32,
    mut mode: uint8_t,
    mut data: *const i8,
    mut extra_data: *mut libc::c_void,
) -> i32 {
    return short_command(
        priv_0,
        fd,
        0xc as i32 as uint8_t,
        mode,
        (if !data.is_null() { strlen(data) as uint8_t as i32 } else { 0 as i32 })
            as uint8_t,
        data,
        extra_data,
    );
}
unsafe extern "C" fn door_command(
    mut priv_0: i32,
    mut fd: i32,
    mut cmd1: uint8_t,
    mut cmd2: uint8_t,
    mut extra_data: *mut libc::c_void,
) -> i32 {
    return short_command(
        priv_0,
        fd,
        cmd1,
        0 as i32 as uint8_t,
        cmd2,
        0 as *const i8,
        extra_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mzip(mut argc: i32, mut argv: *mut *mut i8, mut type_0: i32) {
    let mut extra_data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut c: i32 = 0;
    let mut drive: i8 = 0;
    let mut dev: *mut device_t = 0 as *mut device_t;
    let mut fd: i32 = -(1 as i32);
    let mut name: [i8; 2048] = [0; 2048];
    let mut request: i32 = 0 as i32;
    let mut newMode: uint8_t = 0 as i32 as uint8_t;
    let mut oldMode: uint8_t = 0 as i32 as uint8_t;
    if helpFlag(argc, argv) != 0 {
        usage(0 as i32);
    }
    loop {
        c = getopt(argc, argv, b"i:efpqrwxuh\0" as *const u8 as *const i8);
        if !(c != -(1 as i32)) {
            break;
        }
        match c {
            105 => {
                set_cmd_line_image(optarg);
            }
            102 => {
                if get_real_uid() != 0 {
                    fprintf(
                        stderr,
                        b"Only root can use force. Sorry.\n\0" as *const u8 as *const i8,
                    );
                    exit(1 as i32);
                }
                request |= (1 as i32) << 3 as i32;
            }
            101 => {
                request |= (1 as i32) << 1 as i32;
            }
            113 => {
                request |= (1 as i32) << 0 as i32;
            }
            112 => {
                if request & (1 as i32) << 2 as i32 != 0 {
                    usage(1 as i32);
                }
                request |= (1 as i32) << 2 as i32;
                newMode = 3 as i32 as uint8_t;
            }
            114 => {
                if request & (1 as i32) << 2 as i32 != 0 {
                    usage(1 as i32);
                }
                request |= (1 as i32) << 2 as i32;
                newMode = 2 as i32 as uint8_t;
            }
            119 => {
                if request & (1 as i32) << 2 as i32 != 0 {
                    usage(1 as i32);
                }
                request |= (1 as i32) << 2 as i32;
                newMode = 0 as i32 as uint8_t;
            }
            120 => {
                if request & (1 as i32) << 2 as i32 != 0 {
                    usage(1 as i32);
                }
                request |= (1 as i32) << 2 as i32;
                newMode = 5 as i32 as uint8_t;
            }
            117 => {
                if request & (1 as i32) << 2 as i32 != 0 {
                    usage(1 as i32);
                }
                request |= (1 as i32) << 2 as i32;
                newMode = 8 as i32 as uint8_t;
            }
            104 => {
                usage(0 as i32);
            }
            _ => {
                usage(1 as i32);
            }
        }
    }
    if request == 0 as i32 {
        request = (1 as i32) << 0 as i32;
    }
    if argc - optind > 1 as i32
        || argc - optind == 1 as i32
            && (*(*argv.offset(optind as isize)).offset(0 as i32 as isize) == 0
                || *(*argv.offset(optind as isize)).offset(1 as i32 as isize) as i32
                    != ':' as i32)
    {
        usage(1 as i32);
    }
    drive = ch_toupper(
        (if argc - optind == 1 as i32 {
            *(*argv.offset((argc - 1 as i32) as isize)).offset(0 as i32 as isize) as i32
        } else {
            ':' as i32
        }) as i8,
    );
    dev = devices;
    while !((*dev).name).is_null() {
        let mut cdb: [u8; 6] = [
            0 as i32 as u8,
            0 as i32 as u8,
            0 as i32 as u8,
            0 as i32 as u8,
            0 as i32 as u8,
            0 as i32 as u8,
        ];
        let mut inq_data: C2RustUnnamed = C2RustUnnamed {
            type_0: 0,
            type_modifier: 0,
            scsi_version: 0,
            data_format: 0,
            length: 0,
            reserved1: [0; 2],
            capabilities: 0,
            vendor: [0; 8],
            product: [0; 16],
            revision: [0; 4],
            vendor_specific: [0; 20],
            reserved2: [0; 40],
        };
        if !((*dev).drive as i32 != drive as i32) {
            expand((*dev).name, name.as_mut_ptr());
            if request & ((1 as i32) << 2 as i32 | (1 as i32) << 1 as i32) != 0
                && request & (1 as i32) << 3 as i32 == 0
                && test_mounted(name.as_mut_ptr()) != 0
            {
                fprintf(
                    stderr,
                    b"Can't change status of/eject mounted device\n\0" as *const u8
                        as *const i8,
                );
                exit(1 as i32);
            }
            precmd(dev);
            if !dev.is_null() && (*dev).misc_flags & 0x2 as u32 != 0 {
                reclaim_privs();
            }
            fd = scsi_open(
                name.as_mut_ptr(),
                0 as i32 | 0o4000 as i32,
                0o644 as i32,
                &mut extra_data,
            );
            if !dev.is_null() && (*dev).misc_flags & 0x2 as u32 != 0 {
                drop_privs();
            }
            if !(fd == -(1 as i32)) {
                closeExec(fd);
                if request & ((1 as i32) << 2 as i32 | (1 as i32) << 0 as i32) == 0 {
                    break;
                }
                cdb[0 as i32 as usize] = 0x12 as i32 as u8;
                cdb[4 as i32 as usize] = ::core::mem::size_of::<C2RustUnnamed>() as u64
                    as u8;
                if zip_cmd(
                    (!dev.is_null() && (*dev).misc_flags & 0x2 as u32 != 0) as i32,
                    fd,
                    cdb.as_mut_ptr(),
                    6 as i32 as uint8_t,
                    SCSI_IO_READ,
                    &mut inq_data as *mut C2RustUnnamed as *mut libc::c_void,
                    ::core::mem::size_of::<C2RustUnnamed>() as u64 as uint32_t,
                    extra_data,
                ) != 0 as i32
                {
                    close(fd);
                } else {
                    if !(strncasecmp(
                        b"IOMEGA  \0" as *const u8 as *const i8,
                        (inq_data.vendor).as_mut_ptr(),
                        ::core::mem::size_of::<[i8; 8]>() as u64,
                    ) != 0
                        || strncasecmp(
                            b"ZIP 100         \0" as *const u8 as *const i8,
                            (inq_data.product).as_mut_ptr(),
                            ::core::mem::size_of::<[i8; 16]>() as u64,
                        ) != 0
                            && strncasecmp(
                                b"ZIP 100 PLUS    \0" as *const u8 as *const i8,
                                (inq_data.product).as_mut_ptr(),
                                ::core::mem::size_of::<[i8; 16]>() as u64,
                            ) != 0
                            && strncasecmp(
                                b"ZIP 250         \0" as *const u8 as *const i8,
                                (inq_data.product).as_mut_ptr(),
                                ::core::mem::size_of::<[i8; 16]>() as u64,
                            ) != 0
                            && strncasecmp(
                                b"ZIP 750         \0" as *const u8 as *const i8,
                                (inq_data.product).as_mut_ptr(),
                                ::core::mem::size_of::<[i8; 16]>() as u64,
                            ) != 0
                            && strncasecmp(
                                b"JAZ 1GB         \0" as *const u8 as *const i8,
                                (inq_data.product).as_mut_ptr(),
                                ::core::mem::size_of::<[i8; 16]>() as u64,
                            ) != 0
                            && strncasecmp(
                                b"JAZ 2GB         \0" as *const u8 as *const i8,
                                (inq_data.product).as_mut_ptr(),
                                ::core::mem::size_of::<[i8; 16]>() as u64,
                            ) != 0)
                    {
                        break;
                    }
                    fprintf(
                        stderr,
                        b"Skipping drive with vendor='\0" as *const u8 as *const i8,
                    );
                    fwrite(
                        (inq_data.vendor).as_mut_ptr() as *const libc::c_void,
                        1 as i32 as size_t,
                        ::core::mem::size_of::<[i8; 8]>() as u64,
                        stderr,
                    );
                    fprintf(stderr, b"' product='\0" as *const u8 as *const i8);
                    fwrite(
                        (inq_data.product).as_mut_ptr() as *const libc::c_void,
                        1 as i32 as size_t,
                        ::core::mem::size_of::<[i8; 16]>() as u64,
                        stderr,
                    );
                    fprintf(stderr, b"'\n\0" as *const u8 as *const i8);
                    close(fd);
                }
            }
        }
        dev = dev.offset(1);
        dev;
    }
    if (*dev).drive as i32 == 0 as i32 {
        fprintf(
            stderr,
            b"%s: drive '%c:' is not a Zip or Jaz drive\n\0" as *const u8 as *const i8,
            *argv.offset(0 as i32 as isize),
            drive as i32,
        );
        exit(1 as i32);
    }
    if request & ((1 as i32) << 2 as i32 | (1 as i32) << 0 as i32) != 0 {
        oldMode = get_zip_status(
            (!dev.is_null() && (*dev).misc_flags & 0x2 as u32 != 0) as i32,
            fd,
            extra_data,
        );
    }
    if request & (1 as i32) << 2 as i32 != 0 {
        if newMode as i32 == 8 as i32 && oldMode as i32 & 8 as i32 != 0 {
            request &= !((1 as i32) << 2 as i32);
        }
        if newMode as i32 & 0x1 as i32 == 0 && newMode as i32 == oldMode as i32 {
            request &= !((1 as i32) << 2 as i32);
        }
    }
    if request & (1 as i32) << 2 as i32 != 0 {
        let mut ret: i32 = 0;
        let mut unlockMode: uint8_t = 0;
        let mut unlockMask: uint8_t = 0;
        let mut passwd: *const i8 = 0 as *const i8;
        let mut dummy: [i8; 1] = [0; 1];
        if newMode as i32 == 8 as i32 {
            unlockMode = (newMode as i32 | oldMode as i32) as uint8_t;
            unlockMask = 9 as i32 as uint8_t;
        } else {
            unlockMode = (newMode as i32 & !(0x5 as i32)) as uint8_t;
            unlockMask = 1 as i32 as uint8_t;
        }
        if oldMode as i32 & unlockMask as i32 == 1 as i32 {
            let mut s: *mut i8 = 0 as *mut i8;
            passwd = b"APlaceForYourStuff\0" as *const u8 as *const i8;
            s = strchr(passwd, '\n' as i32);
            if !s.is_null() {
                *s = '\0' as i32 as i8;
            }
            iomega_command(
                (!dev.is_null() && (*dev).misc_flags & 0x2 as u32 != 0) as i32,
                fd,
                unlockMode,
                passwd,
                extra_data,
            );
        }
        if get_zip_status(
            (!dev.is_null() && (*dev).misc_flags & 0x2 as u32 != 0) as i32,
            fd,
            extra_data,
        ) as i32 & unlockMask as i32 == 1 as i32
        {
            let mut s_0: *mut i8 = 0 as *mut i8;
            passwd = getpass(b"Password: \0" as *const u8 as *const i8);
            s_0 = strchr(passwd, '\n' as i32);
            if !s_0.is_null() {
                *s_0 = '\0' as i32 as i8;
            }
            ret = iomega_command(
                (!dev.is_null() && (*dev).misc_flags & 0x2 as u32 != 0) as i32,
                fd,
                unlockMode,
                passwd,
                extra_data,
            );
            if ret != 0 {
                if ret == -(1 as i32) {
                    perror(b"passwd: \0" as *const u8 as *const i8);
                } else {
                    fprintf(stderr, b"wrong password\n\0" as *const u8 as *const i8);
                }
                exit(1 as i32);
            }
            if get_zip_status(
                (!dev.is_null() && (*dev).misc_flags & 0x2 as u32 != 0) as i32,
                fd,
                extra_data,
            ) as i32 & unlockMask as i32 == 1 as i32
            {
                fprintf(stderr, b"wrong password\n\0" as *const u8 as *const i8);
                exit(1 as i32);
            }
        }
        if newMode as i32 & 0x1 as i32 != 0 {
            let mut first_try: [i8; 34] = [0; 34];
            passwd = getpass(b"Enter new password:\0" as *const u8 as *const i8);
            strncpy(first_try.as_mut_ptr(), passwd, 33 as i32 as u64);
            passwd = getpass(b"Re-type new password:\0" as *const u8 as *const i8);
            if strncmp(first_try.as_mut_ptr(), passwd, 33 as i32 as u64) != 0 {
                fprintf(
                    stderr,
                    b"You misspelled it. Password not set.\n\0" as *const u8 as *const i8,
                );
                exit(1 as i32);
            }
        } else {
            passwd = dummy.as_mut_ptr();
            dummy[0 as i32 as usize] = '\0' as i32 as i8;
        }
        if newMode as i32 == 8 as i32 {
            newMode = (newMode as i32 | oldMode as i32) as uint8_t;
        }
        ret = iomega_command(
            (!dev.is_null() && (*dev).misc_flags & 0x2 as u32 != 0) as i32,
            fd,
            newMode,
            passwd,
            extra_data,
        );
        if ret != 0 {
            if ret == -(1 as i32) {
                perror(b"set passwd: \0" as *const u8 as *const i8);
            } else {
                fprintf(stderr, b"password not changed\n\0" as *const u8 as *const i8);
            }
            exit(1 as i32);
        }
        ioctl(
            fd,
            ((0 as u32) << 0 as i32 + 8 as i32 + 8 as i32 + 14 as i32
                | ((0x12 as i32) << 0 as i32 + 8 as i32) as u32
                | ((95 as i32) << 0 as i32) as u32
                | ((0 as i32) << 0 as i32 + 8 as i32 + 8 as i32) as u32) as u64,
        );
    }
    if request & (1 as i32) << 0 as i32 != 0 {
        let mut unlocked: *const i8 = 0 as *const i8;
        if oldMode as i32 & 8 as i32 != 0 {
            unlocked = b" and unlocked until eject\0" as *const u8 as *const i8;
        } else {
            unlocked = b"\0" as *const u8 as *const i8;
        }
        match oldMode as i32 & !(8 as i32) {
            0 => {
                printf(
                    b"Drive '%c:' is not write-protected\n\0" as *const u8 as *const i8,
                    drive as i32,
                );
            }
            2 => {
                printf(
                    b"Drive '%c:' is write-protected%s\n\0" as *const u8 as *const i8,
                    drive as i32,
                    unlocked,
                );
            }
            3 => {
                printf(
                    b"Drive '%c:' is password write-protected%s\n\0" as *const u8
                        as *const i8,
                    drive as i32,
                    unlocked,
                );
            }
            5 => {
                printf(
                    b"Drive '%c:' is password protected%s\n\0" as *const u8 as *const i8,
                    drive as i32,
                    unlocked,
                );
            }
            _ => {
                printf(
                    b"Unknown protection mode %d of drive '%c:'\n\0" as *const u8
                        as *const i8,
                    oldMode as i32,
                    drive as i32,
                );
            }
        }
    }
    if request & (1 as i32) << 1 as i32 != 0 {
        if request & (1 as i32) << 3 as i32 != 0 {
            if door_command(
                (!dev.is_null() && (*dev).misc_flags & 0x2 as u32 != 0) as i32,
                fd,
                0x1e as i32 as uint8_t,
                0 as i32 as uint8_t,
                extra_data,
            ) < 0 as i32
            {
                perror(b"door unlock: \0" as *const u8 as *const i8);
                exit(1 as i32);
            }
        }
        if door_command(
            (!dev.is_null() && (*dev).misc_flags & 0x2 as u32 != 0) as i32,
            fd,
            0x1b as i32 as uint8_t,
            1 as i32 as uint8_t,
            extra_data,
        ) < 0 as i32
        {
            perror(b"stop motor: \0" as *const u8 as *const i8);
            exit(1 as i32);
        }
        if door_command(
            (!dev.is_null() && (*dev).misc_flags & 0x2 as u32 != 0) as i32,
            fd,
            0x1b as i32 as uint8_t,
            2 as i32 as uint8_t,
            extra_data,
        ) < 0 as i32
        {
            perror(b"eject: \0" as *const u8 as *const i8);
            exit(1 as i32);
        }
        if door_command(
            (!dev.is_null() && (*dev).misc_flags & 0x2 as u32 != 0) as i32,
            fd,
            0x1b as i32 as uint8_t,
            2 as i32 as uint8_t,
            extra_data,
        ) < 0 as i32
        {
            perror(b"second eject: \0" as *const u8 as *const i8);
            exit(1 as i32);
        }
    }
    close(fd);
    postcmd((*dev).postcmd);
    exit(0 as i32);
}
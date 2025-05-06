#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type doscp_t;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: i32) -> !;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
    fn close(__fd: i32) -> i32;
    fn __errno_location() -> *mut i32;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strerror(_: i32) -> *mut i8;
    fn truncMtOffTo32u(off: mt_off_t) -> uint32_t;
    fn log_2(_: u32) -> u32;
    fn set_geom_noop(
        Stream: *mut Stream_t,
        dev: *mut device_t,
        orig_dev: *mut device_t,
    ) -> i32;
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn postcmd(cmd: *const i8);
    fn precmd(dev: *mut device);
    fn reclaim_privs();
    fn closeExec(fd: i32);
    fn printOom();
    fn drop_privs();
    fn scsi_max_length() -> u32;
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
    fn LockDevice(
        fd: i32,
        dev: *mut device,
        locked: i32,
        lockMode: i32,
        errmsg: *mut i8,
    ) -> i32;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __ssize_t = i64;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = u64;
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
pub struct Stream_t {
    pub Class: *mut Class_t,
    pub refs: i32,
    pub Next: *mut Stream_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Class_t {
    pub read: Option<unsafe extern "C" fn(*mut Stream_t, *mut i8, size_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut Stream_t, *mut i8, size_t) -> ssize_t>,
    pub pread: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut i8, mt_off_t, size_t) -> ssize_t,
    >,
    pub pwrite: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut i8, mt_off_t, size_t) -> ssize_t,
    >,
    pub flush: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
    pub freeFunc: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
    pub set_geom: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut device_t, *mut device_t) -> i32,
    >,
    pub get_data: Option<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut time_t,
            *mut mt_off_t,
            *mut i32,
            *mut uint32_t,
        ) -> i32,
    >,
    pub pre_allocate: Option<unsafe extern "C" fn(*mut Stream_t, mt_off_t) -> i32>,
    pub get_dosConvert: Option<unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t>,
    pub discard: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
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
pub type smt_off_t = mt_off_t;
pub type scsi_io_mode_t = u32;
pub const SCSI_IO_WRITE: scsi_io_mode_t = 1;
pub const SCSI_IO_READ: scsi_io_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScsiDevice_t {
    pub head: Stream_t,
    pub fd: i32,
    pub privileged: i32,
    pub scsi_sector_size: uint32_t,
    pub device_size: mt_off_t,
    pub tot_sectors: uint32_t,
    pub extra_data: *mut libc::c_void,
    pub postcmd: *const i8,
}
unsafe extern "C" fn scsi_init(mut This: *mut ScsiDevice_t) -> i32 {
    let mut fd: i32 = (*This).fd;
    let mut cdb: [u8; 10] = [0; 10];
    let mut buf: [u8; 8] = [0; 8];
    memset(
        cdb.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<[u8; 10]>() as u64,
    );
    memset(
        buf.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<[u8; 8]>() as u64,
    );
    cdb[0 as i32 as usize] = 0x25 as i32 as u8;
    if scsi_cmd(
        fd,
        cdb.as_mut_ptr(),
        ::core::mem::size_of::<[u8; 10]>() as u64 as uint8_t,
        SCSI_IO_READ,
        buf.as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[u8; 8]>() as u64 as uint32_t,
        (*This).extra_data,
    ) == 0 as i32
    {
        (*This).tot_sectors = (buf[0 as i32 as usize] as u32) << 24 as i32
            | (buf[1 as i32 as usize] as u32) << 16 as i32
            | (buf[2 as i32 as usize] as u32) << 8 as i32
            | buf[3 as i32 as usize] as u32;
        if (*This).tot_sectors < 4294967295 as u32 {
            (*This).tot_sectors = ((*This).tot_sectors).wrapping_add(1);
            (*This).tot_sectors;
        }
        (*This).scsi_sector_size = (buf[5 as i32 as usize] as u32) << 16 as i32
            | (buf[6 as i32 as usize] as u32) << 8 as i32
            | buf[7 as i32 as usize] as u32;
        if (*This).scsi_sector_size != 512 as i32 as u32 {
            fprintf(
                stderr,
                b"  (scsi_sector_size=%d)\n\0" as *const u8 as *const i8,
                (*This).scsi_sector_size,
            );
        }
        return 0 as i32;
    } else {
        return -(1 as i32)
    };
}
unsafe extern "C" fn bytesToSectors(
    mut bytes: size_t,
    mut sector_size: uint32_t,
) -> uint32_t {
    let mut sectors: size_t = bytes.wrapping_div(sector_size as u64);
    if bytes.wrapping_rem(sector_size as u64) != 0 {
        sectors = sectors.wrapping_add(1);
        sectors;
    }
    if sectors > 4294967295 as u32 as u64 {
        return 4294967295 as u32
    } else {
        return sectors as uint32_t
    };
}
unsafe extern "C" fn scsi_io(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut where_0: mt_off_t,
    mut len: size_t,
    mut rwcmd: scsi_io_mode_t,
) -> ssize_t {
    let mut firstblock: u32 = 0;
    let mut nsect: u32 = 0;
    let mut clen: uint8_t = 0;
    let mut r: i32 = 0;
    let mut max: u32 = 0;
    let mut offset: uint32_t = 0;
    let mut cdb: [u8; 10] = [0; 10];
    let mut This: *mut ScsiDevice_t = Stream as *mut ScsiDevice_t;
    firstblock = truncMtOffTo32u(where_0 / (*This).scsi_sector_size as mt_off_t);
    offset = (where_0 % (*This).scsi_sector_size as i64) as uint32_t;
    nsect = bytesToSectors((offset as u64).wrapping_add(len), (*This).scsi_sector_size);
    if len > 512 as i32 as u64 {
        while nsect.wrapping_mul((*This).scsi_sector_size) as u64 > len {
            nsect = nsect.wrapping_sub(1);
            nsect;
        }
        if nsect == 0 {
            fprintf(stderr, b"Scsi buffer too small\n\0" as *const u8 as *const i8);
            exit(1 as i32);
        }
        if rwcmd as u32 == SCSI_IO_WRITE as i32 as u32 && offset != 0 {
            fprintf(stderr, b"Unaligned write\n\0" as *const u8 as *const i8);
            exit(1 as i32);
        }
    }
    max = scsi_max_length();
    if nsect > max {
        nsect = max;
    }
    memset(
        cdb.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<[u8; 10]>() as u64,
    );
    match rwcmd as u32 {
        0 => {
            cdb[0 as i32 as usize] = 0x8 as i32 as u8;
        }
        1 => {
            cdb[0 as i32 as usize] = 0xa as i32 as u8;
        }
        _ => {}
    }
    cdb[1 as i32 as usize] = 0 as i32 as u8;
    if firstblock > 0x1fffff as i32 as u32 || nsect > 0xff as i32 as u32 {
        cdb[0 as i32 as usize] = (cdb[0 as i32 as usize] as i32 | 0x20 as i32) as u8;
        clen = 10 as i32 as uint8_t;
        cdb[2 as i32 as usize] = ((firstblock >> 24 as i32) as u8 as i32 & 0xff as i32)
            as u8;
        cdb[3 as i32 as usize] = ((firstblock >> 16 as i32) as u8 as i32 & 0xff as i32)
            as u8;
        cdb[4 as i32 as usize] = ((firstblock >> 8 as i32) as u8 as i32 & 0xff as i32)
            as u8;
        cdb[5 as i32 as usize] = (firstblock as u8 as i32 & 0xff as i32) as u8;
        cdb[6 as i32 as usize] = 0 as i32 as u8;
        cdb[7 as i32 as usize] = ((nsect >> 8 as i32) as u8 as i32 & 0xff as i32) as u8;
        cdb[8 as i32 as usize] = (nsect as u8 as i32 & 0xff as i32) as u8;
        cdb[9 as i32 as usize] = 0 as i32 as u8;
    } else {
        clen = 6 as i32 as uint8_t;
        cdb[1 as i32 as usize] = (cdb[1 as i32 as usize] as i32
            | (firstblock >> 16 as i32 & 0x1f as i32 as u32) as u8 as i32) as u8;
        cdb[2 as i32 as usize] = (firstblock >> 8 as i32 & 0xff as i32 as u32) as u8;
        cdb[3 as i32 as usize] = (firstblock as u8 as i32 & 0xff as i32) as u8;
        cdb[4 as i32 as usize] = nsect as u8;
        cdb[5 as i32 as usize] = 0 as i32 as u8;
    }
    if (*This).privileged != 0 {
        reclaim_privs();
    }
    r = scsi_cmd(
        (*This).fd,
        cdb.as_mut_ptr(),
        clen,
        rwcmd,
        buf as *mut libc::c_void,
        nsect.wrapping_mul((*This).scsi_sector_size),
        (*This).extra_data,
    );
    if (*This).privileged != 0 {
        drop_privs();
    }
    if r != 0 {
        perror(
            if rwcmd as u32 == SCSI_IO_READ as i32 as u32 {
                b"SCMD_READ\0" as *const u8 as *const i8
            } else {
                b"SCMD_WRITE\0" as *const u8 as *const i8
            },
        );
        return -(1 as i32) as ssize_t;
    }
    if offset > 0 as i32 as u32 {
        memmove(
            buf as *mut libc::c_void,
            buf.offset(offset as isize) as *const libc::c_void,
            nsect.wrapping_mul((*This).scsi_sector_size).wrapping_sub(offset) as u64,
        );
    }
    if len == 256 as i32 as u64 {
        return 256 as i32 as ssize_t
    } else if len == 512 as i32 as u64 {
        return 512 as i32 as ssize_t
    } else {
        return nsect.wrapping_mul((*This).scsi_sector_size).wrapping_sub(offset)
            as ssize_t
    };
}
unsafe extern "C" fn scsi_pread(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut where_0: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    return scsi_io(Stream, buf, where_0, len, SCSI_IO_READ);
}
unsafe extern "C" fn scsi_pwrite(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut where_0: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    return scsi_io(Stream, buf, where_0, len, SCSI_IO_WRITE);
}
unsafe extern "C" fn scsi_get_data(
    mut Stream: *mut Stream_t,
    mut date: *mut time_t,
    mut size: *mut mt_off_t,
    mut type_0: *mut i32,
    mut address: *mut uint32_t,
) -> i32 {
    let mut This: *mut ScsiDevice_t = Stream as *mut ScsiDevice_t;
    if !date.is_null() || !type_0.is_null() || !address.is_null() {
        fprintf(stderr, b"Get_data call not supported\n\0" as *const u8 as *const i8);
    }
    if !size.is_null() {
        *size = (*This).device_size;
    }
    return 0 as i32;
}
unsafe extern "C" fn scsi_free(mut Stream: *mut Stream_t) -> i32 {
    let mut This: *mut ScsiDevice_t = Stream as *mut ScsiDevice_t;
    if (*This).fd > 2 as i32 {
        let mut ret: i32 = close((*This).fd);
        postcmd((*This).postcmd);
        return ret;
    } else {
        return 0 as i32
    };
}
static mut ScsiDeviceClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: None,
            write: None,
            pread: Some(
                scsi_pread
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                scsi_pwrite
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            flush: None,
            freeFunc: Some(scsi_free as unsafe extern "C" fn(*mut Stream_t) -> i32),
            set_geom: Some(
                set_geom_noop
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut device_t,
                        *mut device_t,
                    ) -> i32,
            ),
            get_data: Some(
                scsi_get_data
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut time_t,
                        *mut mt_off_t,
                        *mut i32,
                        *mut uint32_t,
                    ) -> i32,
            ),
            pre_allocate: None,
            get_dosConvert: None,
            discard: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn OpenScsi(
    mut dev: *mut device,
    mut name: *const i8,
    mut mode: i32,
    mut errmsg: *mut i8,
    mut mode2: i32,
    mut locked: i32,
    mut lockMode: i32,
    mut maxSize: *mut mt_off_t,
) -> *mut Stream_t {
    let mut ret: i32 = 0;
    let mut This: *mut ScsiDevice_t = 0 as *mut ScsiDevice_t;
    if !(!dev.is_null() && (*dev).misc_flags & 0x1 as u32 != 0) {
        return 0 as *mut Stream_t;
    }
    This = calloc(1 as i32 as u64, ::core::mem::size_of::<ScsiDevice_t>() as u64)
        as *mut ScsiDevice_t;
    if This.is_null() {
        printOom();
        return 0 as *mut Stream_t;
    }
    memset(
        This as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<ScsiDevice_t>() as u64,
    );
    init_head(&mut (*This).head, &mut ScsiDeviceClass, 0 as *mut Stream_t);
    (*This).scsi_sector_size = 512 as i32 as uint32_t;
    if !dev.is_null() {
        if mode2 & 1 as i32 == 0 {
            (*This).privileged = (!dev.is_null() && (*dev).misc_flags & 0x2 as u32 != 0)
                as i32;
        }
        mode |= (*dev).mode;
    }
    precmd(dev);
    if !dev.is_null() {
        (*This).postcmd = (*dev).postcmd;
    }
    if !dev.is_null() && (*dev).misc_flags & 0x2 as u32 != 0 && mode2 & 1 as i32 == 0 {
        reclaim_privs();
    }
    (*This).fd = scsi_open(
        name,
        mode,
        if !dev.is_null() && (*dev).misc_flags & 0x4 as u32 != 0 {
            0o444 as i32
        } else {
            0o666 as i32
        },
        &mut (*This).extra_data,
    );
    if !dev.is_null() && (*dev).misc_flags & 0x2 as u32 != 0 && mode2 & 1 as i32 == 0 {
        drop_privs();
    }
    if (*This).fd < 0 as i32 {
        if !errmsg.is_null() {
            snprintf(
                errmsg,
                199 as i32 as u64,
                b"Can't open %s: %s\0" as *const u8 as *const i8,
                name,
                strerror(*__errno_location()),
            );
        }
    } else {
        if !dev.is_null() && (*dev).misc_flags & 0x2 as u32 != 0 && mode2 & 1 as i32 == 0
        {
            closeExec((*This).fd);
        }
        if !(LockDevice((*This).fd, dev, locked, lockMode, errmsg) < 0 as i32) {
            if !maxSize.is_null() {
                *maxSize = (((1 as i32 as mt_off_t)
                    << (if ((31 as i32 as u32)
                        .wrapping_add(log_2((*This).scsi_sector_size))
                        .wrapping_sub(1 as i32 as u32) as u64)
                        < (::core::mem::size_of::<mt_off_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)
                    {
                        (31 as i32 as u32)
                            .wrapping_add(log_2((*This).scsi_sector_size))
                            .wrapping_sub(1 as i32 as u32) as u64
                    } else {
                        (::core::mem::size_of::<mt_off_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)
                    })) - 1 as i32 as i64) << 1 as i32 | 1 as i32 as i64;
            }
            if (*This).privileged != 0 {
                reclaim_privs();
            }
            ret = scsi_init(This);
            if (*This).privileged != 0 {
                drop_privs();
            }
            if !(ret < 0 as i32) {
                (*dev).tot_sectors = (*This).tot_sectors;
                return &mut (*This).head;
            }
        }
        close((*This).fd);
        postcmd((*This).postcmd);
    }
    free(This as *mut i8 as *mut libc::c_void);
    return 0 as *mut Stream_t;
}
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
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn __errno_location() -> *mut i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strerror(_: i32) -> *mut i8;
    static max_off_t_seek: mt_off_t;
    fn reclaim_privs();
    fn drop_privs();
    fn closeExec(fd: i32);
    fn printOom();
    fn lock_dev(fd: i32, mode: i32, dev: *mut device) -> i32;
    fn precmd(dev: *mut device);
    fn postcmd(cmd: *const i8);
    fn mt_lseek(fd: i32, where_0: mt_off_t, whence: i32) -> i32;
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn init_geom(
        fd: i32,
        dev: *mut device,
        orig_dev: *mut device,
        statbuf: *mut stat,
    ) -> i32;
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
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SimpleFile_t {
    pub head: Stream_t,
    pub statbuf: stat,
    pub fd: i32,
    pub lastwhere: mt_off_t,
    pub seekable: i32,
    pub privileged: i32,
    pub postcmd: *const i8,
}
pub type iofn = Option<unsafe extern "C" fn(i32, *mut libc::c_void, size_t) -> ssize_t>;
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
unsafe extern "C" fn file_io(
    mut This: *mut SimpleFile_t,
    mut buf: *mut i8,
    mut where_0: mt_off_t,
    mut len: size_t,
    mut io: iofn,
) -> ssize_t {
    let mut ret: ssize_t = 0;
    if (*This).seekable != 0 && where_0 != (*This).lastwhere {
        if mt_lseek((*This).fd, where_0, 0 as i32) < 0 as i32 {
            perror(b"seek\0" as *const u8 as *const i8);
            return -(1 as i32) as ssize_t;
        }
        (*This).lastwhere = where_0;
    }
    ret = io
        .expect("non-null function pointer")((*This).fd, buf as *mut libc::c_void, len);
    if ret == -(1 as i32) as i64 {
        perror(b"plain_io read/write\0" as *const u8 as *const i8);
        return -(1 as i32) as ssize_t;
    }
    (*This).lastwhere = where_0 + ret;
    return ret;
}
unsafe extern "C" fn file_read(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut SimpleFile_t = Stream as *mut SimpleFile_t;
    return file_io(
        This,
        buf,
        (*This).lastwhere,
        len,
        Some(read as unsafe extern "C" fn(i32, *mut libc::c_void, size_t) -> ssize_t),
    );
}
unsafe extern "C" fn file_write(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut SimpleFile_t = Stream as *mut SimpleFile_t;
    return file_io(
        This,
        buf,
        (*This).lastwhere,
        len,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32, *const libc::c_void, size_t) -> ssize_t>,
            iofn,
        >(
            Some(
                write
                    as unsafe extern "C" fn(i32, *const libc::c_void, size_t) -> ssize_t,
            ),
        ),
    );
}
unsafe extern "C" fn file_pread(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut where_0: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut SimpleFile_t = Stream as *mut SimpleFile_t;
    return file_io(
        This,
        buf,
        where_0,
        len,
        Some(read as unsafe extern "C" fn(i32, *mut libc::c_void, size_t) -> ssize_t),
    );
}
unsafe extern "C" fn file_pwrite(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut where_0: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut SimpleFile_t = Stream as *mut SimpleFile_t;
    return file_io(
        This,
        buf,
        where_0,
        len,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32, *const libc::c_void, size_t) -> ssize_t>,
            iofn,
        >(
            Some(
                write
                    as unsafe extern "C" fn(i32, *const libc::c_void, size_t) -> ssize_t,
            ),
        ),
    );
}
unsafe extern "C" fn file_flush(mut Stream: *mut Stream_t) -> i32 {
    return 0 as i32;
}
unsafe extern "C" fn file_free(mut Stream: *mut Stream_t) -> i32 {
    let mut This: *mut SimpleFile_t = Stream as *mut SimpleFile_t;
    if (*This).fd > 2 as i32 {
        let mut ret: i32 = close((*This).fd);
        postcmd((*This).postcmd);
        return ret;
    } else {
        return 0 as i32
    };
}
unsafe extern "C" fn init_geom_with_reg(
    mut fd: i32,
    mut dev: *mut device,
    mut orig_dev: *mut device,
    mut statbuf: *mut stat,
) -> i32 {
    if (*statbuf).st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32 {
        let mut sectors: mt_off_t = 0;
        if (*statbuf).st_size == 0 as i32 as i64 {
            return 0 as i32;
        }
        sectors = (*statbuf).st_size
            / (if (*dev).sector_size as i32 != 0 {
                (*dev).sector_size as i32
            } else {
                512 as i32
            }) as mt_off_t;
        (*dev).tot_sectors = if sectors > 4294967295 as u32 as i64 {
            4294967295 as u32
        } else {
            sectors as uint32_t
        };
        return 0 as i32;
    } else {
        return init_geom(fd, dev, orig_dev, statbuf)
    };
}
unsafe extern "C" fn file_geom(
    mut Stream: *mut Stream_t,
    mut dev: *mut device,
    mut orig_dev: *mut device,
) -> i32 {
    let mut ret: i32 = 0;
    let mut This: *mut SimpleFile_t = Stream as *mut SimpleFile_t;
    if (*dev).sector_size as i32 != 0 && (*dev).sector_size as i32 != 512 as i32 {
        (*dev).sectors = ((*dev).sectors as i32 * (*dev).sector_size as i32 / 512 as i32)
            as uint16_t;
    }
    ret = init_geom_with_reg((*This).fd, dev, orig_dev, &mut (*This).statbuf);
    if (*dev).sector_size as i32 != 0 && (*dev).sector_size as i32 != 512 as i32 {
        (*dev).sectors = ((*dev).sectors as i32 * 512 as i32 / (*dev).sector_size as i32)
            as uint16_t;
    }
    return ret;
}
unsafe extern "C" fn file_data(
    mut Stream: *mut Stream_t,
    mut date: *mut time_t,
    mut size: *mut mt_off_t,
    mut type_0: *mut i32,
    mut address: *mut uint32_t,
) -> i32 {
    let mut This: *mut SimpleFile_t = Stream as *mut SimpleFile_t;
    if !date.is_null() {
        *date = (*This).statbuf.st_mtim.tv_sec;
    }
    if !size.is_null() {
        *size = (*This).statbuf.st_size;
    }
    if !type_0.is_null() {
        *type_0 = ((*This).statbuf.st_mode & 0o170000 as i32 as u32
            == 0o40000 as i32 as u32) as i32;
    }
    if !address.is_null() {
        *address = 0 as i32 as uint32_t;
    }
    return 0 as i32;
}
unsafe extern "C" fn file_discard(mut Stream: *mut Stream_t) -> i32 {
    return 0 as i32;
}
static mut SimpleFileClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: Some(
                file_read
                    as unsafe extern "C" fn(*mut Stream_t, *mut i8, size_t) -> ssize_t,
            ),
            write: Some(
                file_write
                    as unsafe extern "C" fn(*mut Stream_t, *mut i8, size_t) -> ssize_t,
            ),
            pread: Some(
                file_pread
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                file_pwrite
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            flush: Some(file_flush as unsafe extern "C" fn(*mut Stream_t) -> i32),
            freeFunc: Some(file_free as unsafe extern "C" fn(*mut Stream_t) -> i32),
            set_geom: Some(
                file_geom
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut device,
                        *mut device,
                    ) -> i32,
            ),
            get_data: Some(
                file_data
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
            discard: Some(file_discard as unsafe extern "C" fn(*mut Stream_t) -> i32),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn LockDevice(
    mut fd: i32,
    mut dev: *mut device,
    mut locked: i32,
    mut lockMode: i32,
    mut errmsg: *mut i8,
) -> i32 {
    if locked != 0
        && lock_dev(fd, (lockMode & 0o3 as i32 == 0o2 as i32) as i32, dev) != 0
    {
        if !errmsg.is_null() {
            snprintf(
                errmsg,
                199 as i32 as u64,
                b"plain floppy: device \"%s\" busy (%s):\0" as *const u8 as *const i8,
                if !dev.is_null() {
                    (*dev).name
                } else {
                    b"unknown\0" as *const u8 as *const i8
                },
                strerror(*__errno_location()),
            );
        }
        if *__errno_location() != 95 as i32 || lockMode & 0o3 as i32 == 0o2 as i32 {
            return -(1 as i32);
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn SimpleFileOpen(
    mut dev: *mut device,
    mut orig_dev: *mut device,
    mut name: *const i8,
    mut mode: i32,
    mut errmsg: *mut i8,
    mut mode2: i32,
    mut locked: i32,
    mut maxSize: *mut mt_off_t,
) -> *mut Stream_t {
    return SimpleFileOpenWithLm(
        dev,
        orig_dev,
        name,
        mode,
        errmsg,
        mode2,
        locked,
        mode,
        maxSize,
        0 as *mut i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SimpleFileOpenWithLm(
    mut dev: *mut device,
    mut orig_dev: *mut device,
    mut name: *const i8,
    mut mode: i32,
    mut errmsg: *mut i8,
    mut mode2: i32,
    mut locked: i32,
    mut lockMode: i32,
    mut maxSize: *mut mt_off_t,
    mut geomFailure: *mut i32,
) -> *mut Stream_t {
    let mut current_block: u64;
    let mut This: *mut SimpleFile_t = 0 as *mut SimpleFile_t;
    if !dev.is_null() && (*dev).misc_flags & 0x1 as u32 != 0 {
        return 0 as *mut Stream_t;
    }
    This = calloc(1 as i32 as u64, ::core::mem::size_of::<SimpleFile_t>() as u64)
        as *mut SimpleFile_t;
    if This.is_null() {
        printOom();
        return 0 as *mut Stream_t;
    }
    memset(
        This as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<SimpleFile_t>() as u64,
    );
    (*This).seekable = 1 as i32;
    init_head(&mut (*This).head, &mut SimpleFileClass, 0 as *mut Stream_t);
    if name.is_null() || strcmp(name, b"-\0" as *const u8 as *const i8) == 0 as i32 {
        if mode == 0 as i32 {
            (*This).fd = 0 as i32;
        } else {
            (*This).fd = 1 as i32;
        }
        (*This).seekable = 0 as i32;
        if fstat((*This).fd, &mut (*This).statbuf) < 0 as i32 {
            free(This as *mut i8 as *mut libc::c_void);
            if !errmsg.is_null() {
                snprintf(
                    errmsg,
                    199 as i32 as u64,
                    b"Can't stat -: %s\0" as *const u8 as *const i8,
                    strerror(*__errno_location()),
                );
            }
            return 0 as *mut Stream_t;
        }
        return &mut (*This).head;
    }
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
    (*This).fd = open(
        name,
        mode | 0 as i32 | 0 as i32,
        if !dev.is_null() && (*dev).misc_flags & 0x4 as u32 != 0 {
            0o444 as i32
        } else {
            0o666 as i32
        },
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
        if fstat((*This).fd, &mut (*This).statbuf) < 0 as i32 {
            if !errmsg.is_null() {
                snprintf(
                    errmsg,
                    199 as i32 as u64,
                    b"Can't stat %s: %s\0" as *const u8 as *const i8,
                    name,
                    strerror(*__errno_location()),
                );
            }
        } else if !(LockDevice((*This).fd, dev, locked, lockMode, errmsg) < 0 as i32) {
            if !dev.is_null() {
                *__errno_location() = 0 as i32;
                if (!(!dev.is_null() && (*dev).misc_flags & 0x10 as u32 != 0)
                    && (*dev).tracks != 0 || mode2 & 4 as i32 != 0)
                    && init_geom_with_reg(
                        (*This).fd,
                        dev,
                        orig_dev,
                        &mut (*This).statbuf,
                    ) != 0
                {
                    if !geomFailure.is_null()
                        && (*__errno_location() == 9 as i32
                            || *__errno_location() == 1 as i32)
                    {
                        *geomFailure = 1 as i32;
                        return 0 as *mut Stream_t;
                    } else if !errmsg.is_null() {
                        sprintf(
                            errmsg,
                            b"init: set default params\0" as *const u8 as *const i8,
                        );
                    }
                    current_block = 4581956841549642823;
                } else {
                    current_block = 1622411330066726685;
                }
            } else {
                current_block = 1622411330066726685;
            }
            match current_block {
                4581956841549642823 => {}
                _ => {
                    if !maxSize.is_null() {
                        *maxSize = max_off_t_seek;
                    }
                    (*This).lastwhere = 0 as i32 as mt_off_t;
                    return &mut (*This).head;
                }
            }
        }
        close((*This).fd);
        postcmd((*This).postcmd);
    }
    free(This as *mut i8 as *mut libc::c_void);
    return 0 as *mut Stream_t;
}
#[no_mangle]
pub unsafe extern "C" fn get_fd(mut Stream: *mut Stream_t) -> i32 {
    let mut clazz: *mut Class_t = 0 as *mut Class_t;
    let mut This: *mut SimpleFile_t = Stream as *mut SimpleFile_t;
    clazz = (*This).head.Class;
    if clazz != &mut SimpleFileClass as *mut Class_t {
        return -(1 as i32)
    } else {
        return (*This).fd
    };
}
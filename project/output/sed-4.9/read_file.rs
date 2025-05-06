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
    fn fileno(__stream: *mut FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
    fn ftello(__stream: *mut FILE) -> __off_t;
    fn setvbuf(__stream: *mut FILE, __buf: *mut i8, __modes: i32, __n: size_t) -> i32;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn rpl_free(ptr: *mut libc::c_void);
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
    fn __errno_location() -> *mut i32;
}
pub type size_t = u64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
#[no_mangle]
pub unsafe extern "C" fn fread_file(
    mut stream: *mut FILE,
    mut flags: i32,
    mut length: *mut size_t,
) -> *mut i8 {
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut alloc: size_t = 8192 as i32 as size_t;
    let mut st: stat = stat {
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
    if fstat(fileno(stream), &mut st) >= 0 as i32
        && st.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32
    {
        let mut pos: off_t = ftello(stream);
        if pos >= 0 as i32 as i64 && pos < st.st_size {
            let mut alloc_off: off_t = st.st_size - pos;
            if (9223372036854775807 as i64 - 1 as i32 as i64) < alloc_off {
                *__errno_location() = 12 as i32;
                return 0 as *mut i8;
            }
            alloc = (alloc_off + 1 as i32 as i64) as size_t;
        }
    }
    buf = malloc(alloc) as *mut i8;
    if buf.is_null() {
        return 0 as *mut i8;
    }
    let mut size: size_t = 0 as i32 as size_t;
    let mut save_errno: i32 = 0;
    loop {
        let mut requested: size_t = alloc.wrapping_sub(size);
        let mut count: size_t = fread(
            buf.offset(size as isize) as *mut libc::c_void,
            1 as i32 as size_t,
            requested,
            stream,
        );
        size = (size as u64).wrapping_add(count) as size_t as size_t;
        if count != requested {
            save_errno = *__errno_location();
            if ferror(stream) != 0 {
                break;
            }
            if size < alloc.wrapping_sub(1 as i32 as u64) {
                if flags & 0x2 as i32 != 0 {
                    let mut smaller_buf: *mut i8 = malloc(
                        size.wrapping_add(1 as i32 as u64),
                    ) as *mut i8;
                    if smaller_buf.is_null() {
                        explicit_bzero(
                            buf.offset(size as isize) as *mut libc::c_void,
                            alloc.wrapping_sub(size),
                        );
                    } else {
                        memcpy(
                            smaller_buf as *mut libc::c_void,
                            buf as *const libc::c_void,
                            size,
                        );
                        explicit_bzero(buf as *mut libc::c_void, alloc);
                        rpl_free(buf as *mut libc::c_void);
                        buf = smaller_buf;
                    }
                } else {
                    let mut smaller_buf_0: *mut i8 = realloc(
                        buf as *mut libc::c_void,
                        size.wrapping_add(1 as i32 as u64),
                    ) as *mut i8;
                    if !smaller_buf_0.is_null() {
                        buf = smaller_buf_0;
                    }
                }
            }
            *buf.offset(size as isize) = '\0' as i32 as i8;
            *length = size;
            return buf;
        } else {
            let mut new_buf: *mut i8 = 0 as *mut i8;
            let mut save_alloc: size_t = alloc;
            if alloc == 9223372036854775807 as i64 as u64 {
                save_errno = 12 as i32;
                break;
            } else {
                if alloc
                    < (9223372036854775807 as i64 as u64)
                        .wrapping_sub(alloc.wrapping_div(2 as i32 as u64))
                {
                    alloc = alloc.wrapping_add(alloc.wrapping_div(2 as i32 as u64));
                } else {
                    alloc = 9223372036854775807 as i64 as size_t;
                }
                if flags & 0x2 as i32 != 0 {
                    new_buf = malloc(alloc) as *mut i8;
                    if new_buf.is_null() {
                        save_errno = *__errno_location();
                        break;
                    } else {
                        memcpy(
                            new_buf as *mut libc::c_void,
                            buf as *const libc::c_void,
                            save_alloc,
                        );
                        explicit_bzero(buf as *mut libc::c_void, save_alloc);
                        rpl_free(buf as *mut libc::c_void);
                    }
                } else {
                    new_buf = realloc(buf as *mut libc::c_void, alloc) as *mut i8;
                    if new_buf.is_null() {
                        save_errno = *__errno_location();
                        break;
                    }
                }
                buf = new_buf;
            }
        }
    }
    if flags & 0x2 as i32 != 0 {
        explicit_bzero(buf as *mut libc::c_void, alloc);
    }
    rpl_free(buf as *mut libc::c_void);
    *__errno_location() = save_errno;
    return 0 as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn read_file(
    mut filename: *const i8,
    mut flags: i32,
    mut length: *mut size_t,
) -> *mut i8 {
    let mut mode: *const i8 = if flags & 0x1 as i32 != 0 {
        b"rbe\0" as *const u8 as *const i8
    } else {
        b"re\0" as *const u8 as *const i8
    };
    let mut stream: *mut FILE = fopen(filename, mode);
    let mut out: *mut i8 = 0 as *mut i8;
    if stream.is_null() {
        return 0 as *mut i8;
    }
    if flags & 0x2 as i32 != 0 {
        setvbuf(stream, 0 as *mut i8, 2 as i32, 0 as i32 as size_t);
    }
    out = fread_file(stream, flags, length);
    if fclose(stream) != 0 as i32 {
        if !out.is_null() {
            if flags & 0x2 as i32 != 0 {
                explicit_bzero(out as *mut libc::c_void, *length);
            }
            rpl_free(out as *mut libc::c_void);
        }
        return 0 as *mut i8;
    }
    return out;
}
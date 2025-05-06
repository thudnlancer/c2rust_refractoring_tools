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
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fdopen(__fd: i32, __modes: *const i8) -> *mut FILE;
    fn __errno_location() -> *mut i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn close(__fd: i32) -> i32;
}
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
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
unsafe extern "C" fn orig_fopen(
    mut filename: *const i8,
    mut mode: *const i8,
) -> *mut FILE {
    return fopen(filename, mode);
}
#[no_mangle]
pub unsafe extern "C" fn rpl_fopen(
    mut filename: *const i8,
    mut mode: *const i8,
) -> *mut FILE {
    let mut open_direction: i32 = 0;
    let mut open_flags: i32 = 0;
    let mut open_flags_gnu: bool = false;
    let mut fdopen_mode_buf: [i8; 81] = [0; 81];
    open_direction = 0 as i32;
    open_flags = 0 as i32;
    open_flags_gnu = 0 as i32 != 0;
    let mut p: *const i8 = mode;
    let mut q: *mut i8 = fdopen_mode_buf.as_mut_ptr();
    while *p as i32 != '\0' as i32 {
        match *p as i32 {
            114 => {
                open_direction = 0 as i32;
                if q < fdopen_mode_buf.as_mut_ptr().offset(80 as i32 as isize) {
                    let fresh0 = q;
                    q = q.offset(1);
                    *fresh0 = *p;
                }
            }
            119 => {
                open_direction = 0o1 as i32;
                open_flags |= 0o100 as i32 | 0o1000 as i32;
                if q < fdopen_mode_buf.as_mut_ptr().offset(80 as i32 as isize) {
                    let fresh1 = q;
                    q = q.offset(1);
                    *fresh1 = *p;
                }
            }
            97 => {
                open_direction = 0o1 as i32;
                open_flags |= 0o100 as i32 | 0o2000 as i32;
                if q < fdopen_mode_buf.as_mut_ptr().offset(80 as i32 as isize) {
                    let fresh2 = q;
                    q = q.offset(1);
                    *fresh2 = *p;
                }
            }
            98 => {
                open_flags |= 0 as i32;
                if q < fdopen_mode_buf.as_mut_ptr().offset(80 as i32 as isize) {
                    let fresh3 = q;
                    q = q.offset(1);
                    *fresh3 = *p;
                }
            }
            43 => {
                open_direction = 0o2 as i32;
                if q < fdopen_mode_buf.as_mut_ptr().offset(80 as i32 as isize) {
                    let fresh4 = q;
                    q = q.offset(1);
                    *fresh4 = *p;
                }
            }
            120 => {
                open_flags |= 0o200 as i32;
                open_flags_gnu = 1 as i32 != 0;
            }
            101 => {
                open_flags |= 0o2000000 as i32;
                open_flags_gnu = 1 as i32 != 0;
            }
            _ => {
                let mut len: size_t = strlen(p);
                if len
                    > fdopen_mode_buf
                        .as_mut_ptr()
                        .offset(80 as i32 as isize)
                        .offset_from(q) as i64 as u64
                {
                    len = fdopen_mode_buf
                        .as_mut_ptr()
                        .offset(80 as i32 as isize)
                        .offset_from(q) as i64 as size_t;
                }
                memcpy(q as *mut libc::c_void, p as *const libc::c_void, len);
                q = q.offset(len as isize);
                break;
            }
        }
        p = p.offset(1);
        p;
    }
    *q = '\0' as i32 as i8;
    if open_flags_gnu {
        let mut fd: i32 = 0;
        let mut fp: *mut FILE = 0 as *mut FILE;
        fd = open(
            filename,
            open_direction | open_flags,
            0o400 as i32 | 0o200 as i32 | 0o400 as i32 >> 3 as i32
                | 0o200 as i32 >> 3 as i32 | 0o400 as i32 >> 3 as i32 >> 3 as i32
                | 0o200 as i32 >> 3 as i32 >> 3 as i32,
        );
        if fd < 0 as i32 {
            return 0 as *mut FILE;
        }
        fp = fdopen(fd, fdopen_mode_buf.as_mut_ptr());
        if fp.is_null() {
            let mut saved_errno: i32 = *__errno_location();
            close(fd);
            *__errno_location() = saved_errno;
        }
        return fp;
    }
    return orig_fopen(filename, mode);
}
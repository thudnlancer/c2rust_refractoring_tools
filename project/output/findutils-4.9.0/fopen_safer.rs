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
    fn fileno(__stream: *mut FILE) -> i32;
    fn rpl_fclose(stream: *mut FILE) -> i32;
    fn __errno_location() -> *mut i32;
    fn close(__fd: i32) -> i32;
    fn dup_safer(_: i32) -> i32;
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
#[no_mangle]
pub unsafe extern "C" fn fopen_safer(
    mut file: *const i8,
    mut mode: *const i8,
) -> *mut FILE {
    let mut fp: *mut FILE = fopen(file, mode);
    if !fp.is_null() {
        let mut fd: i32 = fileno(fp);
        if 0 as i32 <= fd && fd <= 2 as i32 {
            let mut f: i32 = dup_safer(fd);
            if f < 0 as i32 {
                let mut e: i32 = *__errno_location();
                rpl_fclose(fp);
                *__errno_location() = e;
                return 0 as *mut FILE;
            }
            if rpl_fclose(fp) != 0 as i32
                || {
                    fp = fdopen(f, mode);
                    fp.is_null()
                }
            {
                let mut e_0: i32 = *__errno_location();
                close(f);
                *__errno_location() = e_0;
                return 0 as *mut FILE;
            }
        }
    }
    return fp;
}
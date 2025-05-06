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
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
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
pub unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    ptr = malloc(size);
    if ptr.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"xmalloc(): couldn't allocate %d bytes\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            size,
        );
        exit(1 as i32);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn xcalloc(
    mut num: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    ptr = calloc(num, size);
    if ptr.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"xcalloc(): couldn't allocate %d bytes\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            size,
        );
        exit(1 as i32);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn xrealloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut nptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if ptr.is_null() {
        return xmalloc(size);
    }
    nptr = realloc(ptr, size);
    if nptr.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"xrealloc(): couldn't reallocate %d bytes\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            size,
        );
        exit(1 as i32);
    }
    return nptr;
}
#[no_mangle]
pub unsafe extern "C" fn xfree(mut ptr: *mut libc::c_void) {
    if ptr.is_null() {
        return;
    }
    free(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn xstrdup(mut str: *mut i8) -> *mut i8 {
    let mut tmp: *mut i8 = 0 as *mut i8;
    tmp = xmalloc((strlen(str)).wrapping_add(1 as i32 as u64)) as *mut i8;
    strcpy(tmp, str);
    return tmp;
}
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
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn version_etc(
        stream: *mut FILE,
        command_name: *const i8,
        package: *const i8,
        version: *const i8,
        _: ...
    );
    fn rpl_fflush(gl_stream: *mut FILE) -> i32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
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
pub static mut version_string: *const i8 = b"4.9.0\0" as *const u8 as *const i8;
#[no_mangle]
pub unsafe extern "C" fn display_findutils_version(mut official_name: *const i8) {
    rpl_fflush(stderr);
    version_etc(
        stdout,
        official_name,
        b"GNU findutils\0" as *const u8 as *const i8,
        version_string,
        dcgettext(
            0 as *const i8,
            b"Eric B. Decker\0" as *const u8 as *const i8,
            5 as i32,
        ),
        dcgettext(
            0 as *const i8,
            b"James Youngman\0" as *const u8 as *const i8,
            5 as i32,
        ),
        dcgettext(0 as *const i8, b"Kevin Dalley\0" as *const u8 as *const i8, 5 as i32),
        0 as *mut libc::c_void as *const i8,
    );
}
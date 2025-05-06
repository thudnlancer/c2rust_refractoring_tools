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
    fn printf(_: *const i8, _: ...) -> i32;
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    fn endgrent();
    fn getgrent() -> *mut group;
}
pub type size_t = u64;
pub type __gid_t = u32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut i8,
    pub gr_passwd: *mut i8,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut i8,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: i32) -> i32 {
    return _IO_putc(__c, stdout);
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut g: *mut group = 0 as *mut group;
    let mut i: i32 = 0;
    loop {
        g = getgrent();
        if g.is_null() {
            break;
        }
        printf(
            b"%s:%s:%ld:\0" as *const u8 as *const i8,
            (*g).gr_name,
            (*g).gr_passwd,
            (*g).gr_gid as i64,
        );
        i = 0 as i32;
        while !(*((*g).gr_mem).offset(i as isize)).is_null() {
            printf(b"%s\0" as *const u8 as *const i8, *((*g).gr_mem).offset(i as isize));
            if !(*((*g).gr_mem).offset((i + 1 as i32) as isize)).is_null() {
                putchar(',' as i32);
            }
            i += 1;
            i;
        }
        putchar('\n' as i32);
    }
    endgrent();
    return 0 as i32;
}
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        )
    }
}
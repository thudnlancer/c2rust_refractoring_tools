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
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
}
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type off_t = __off_t;
pub type size_t = u64;
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
pub type mt_off_t = off_t;
#[no_mangle]
pub static mut max_off_t_31: mt_off_t = 0;
static mut max_off_t_32: mt_off_t = 0;
#[no_mangle]
pub static mut max_off_t_41: mt_off_t = 0;
#[no_mangle]
pub static mut max_off_t_seek: mt_off_t = 0;
#[no_mangle]
pub unsafe extern "C" fn fileTooBig(mut off: mt_off_t) -> i32 {
    return (off & !max_off_t_32 != 0 as i32 as i64) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn truncBytes32(mut off: mt_off_t) -> off_t {
    if fileTooBig(off) != 0 {
        fprintf(stderr, b"Internal error, offset too big\n\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    return off;
}
#[no_mangle]
pub unsafe extern "C" fn truncMtOffTo32u(mut off: mt_off_t) -> uint32_t {
    if fileTooBig(off) != 0 {
        fprintf(stderr, b"Internal error, offset too big\n\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    return off as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn truncSizeTo32u(mut siz: size_t) -> uint32_t {
    if siz > 4294967295 as u32 as u64 {
        fprintf(stderr, b"Internal error, size too big\n\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    return siz as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn mt_lseek(
    mut fd: i32,
    mut where_0: mt_off_t,
    mut whence: i32,
) -> i32 {
    if lseek(fd, where_0, whence) >= 0 as i32 as i64 {
        return 0 as i32
    } else {
        return 1 as i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn log_2(mut size: u32) -> u32 {
    let mut i: u32 = 0;
    i = 0 as i32 as u32;
    while i < 24 as i32 as u32 {
        if (1 as u32) << i == size {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 24 as i32 as u32;
}
unsafe extern "C" fn run_static_initializers() {
    max_off_t_31 = (((1 as i32 as mt_off_t)
        << (if ((31 as i32 - 1 as i32) as u64)
            < (::core::mem::size_of::<mt_off_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(2 as i32 as u64)
        {
            (31 as i32 - 1 as i32) as u64
        } else {
            (::core::mem::size_of::<mt_off_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(2 as i32 as u64)
        })) - 1 as i32 as i64) << 1 as i32 | 1 as i32 as i64;
    max_off_t_32 = (((1 as i32 as mt_off_t)
        << (if ((32 as i32 - 1 as i32) as u64)
            < (::core::mem::size_of::<mt_off_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(2 as i32 as u64)
        {
            (32 as i32 - 1 as i32) as u64
        } else {
            (::core::mem::size_of::<mt_off_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(2 as i32 as u64)
        })) - 1 as i32 as i64) << 1 as i32 | 1 as i32 as i64;
    max_off_t_41 = (((1 as i32 as mt_off_t)
        << (if ((41 as i32 - 1 as i32) as u64)
            < (::core::mem::size_of::<mt_off_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(2 as i32 as u64)
        {
            (41 as i32 - 1 as i32) as u64
        } else {
            (::core::mem::size_of::<mt_off_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(2 as i32 as u64)
        })) - 1 as i32 as i64) << 1 as i32 | 1 as i32 as i64;
    max_off_t_seek = (((1 as i32 as mt_off_t)
        << (if (::core::mem::size_of::<off_t>() as u64)
            .wrapping_mul(8 as i32 as u64)
            .wrapping_sub(1 as i32 as u64)
            .wrapping_sub(1 as i32 as u64)
            < (::core::mem::size_of::<mt_off_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(2 as i32 as u64)
        {
            (::core::mem::size_of::<off_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(1 as i32 as u64)
                .wrapping_sub(1 as i32 as u64)
        } else {
            (::core::mem::size_of::<mt_off_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(2 as i32 as u64)
        })) - 1 as i32 as i64) << 1 as i32 | 1 as i32 as i64;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
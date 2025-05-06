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
    fn nettle_knuth_lfib_random(ctx: *mut knuth_lfib_ctx, n: size_t, dst: *mut uint8_t);
    fn nettle_knuth_lfib_init(ctx: *mut knuth_lfib_ctx, seed: uint32_t);
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct knuth_lfib_ctx {
    pub x: [uint32_t; 100],
    pub index: u32,
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
pub type time_t = __time_t;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
unsafe extern "C" fn usage() {
    fprintf(stderr, b"Usage: lfib-stream [SEED]\n\0" as *const u8 as *const i8);
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut ctx: knuth_lfib_ctx = knuth_lfib_ctx {
        x: [0; 100],
        index: 0,
    };
    let mut seed: uint32_t = 0;
    if argc == 1 as i32 {
        seed = time(0 as *mut time_t) as uint32_t;
    } else if argc == 2 as i32 {
        seed = atoi(*argv.offset(1 as i32 as isize)) as uint32_t;
        if seed == 0 {
            usage();
            return 1 as i32;
        }
    } else {
        usage();
        return 1 as i32;
    }
    nettle_knuth_lfib_init(&mut ctx, seed);
    loop {
        let mut buffer: [uint8_t; 500] = [0; 500];
        nettle_knuth_lfib_random(
            &mut ctx,
            ::core::mem::size_of::<[uint8_t; 500]>() as u64,
            buffer.as_mut_ptr(),
        );
        if fwrite(
            buffer.as_mut_ptr() as *const libc::c_void,
            1 as i32 as size_t,
            ::core::mem::size_of::<[uint8_t; 500]>() as u64,
            stdout,
        ) < ::core::mem::size_of::<[uint8_t; 500]>() as u64 || fflush(stdout) < 0 as i32
        {
            return 1 as i32;
        }
    };
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
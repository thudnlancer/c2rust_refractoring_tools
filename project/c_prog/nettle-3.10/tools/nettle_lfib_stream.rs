use ::libc;
extern "C" {
    fn nettle_knuth_lfib_random(ctx: *mut knuth_lfib_ctx, n: size_t, dst: *mut uint8_t);
    fn nettle_knuth_lfib_init(ctx: *mut knuth_lfib_ctx, seed: uint32_t);
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn strtol(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_long;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct knuth_lfib_ctx {
    pub x: [uint32_t; 100],
    pub index: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type time_t = __time_t;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
unsafe extern "C" fn usage() {
    fprintf(
        stderr,
        b"Usage: lfib-stream [SEED]\n\0" as *const u8 as *const libc::c_char,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut ctx: knuth_lfib_ctx = knuth_lfib_ctx {
        x: [0; 100],
        index: 0,
    };
    let mut seed: uint32_t = 0;
    if argc == 1 as libc::c_int {
        seed = time(0 as *mut time_t) as uint32_t;
    } else if argc == 2 as libc::c_int {
        seed = atoi(*argv.offset(1 as libc::c_int as isize)) as uint32_t;
        if seed == 0 {
            usage();
            return 1 as libc::c_int;
        }
    } else {
        usage();
        return 1 as libc::c_int;
    }
    nettle_knuth_lfib_init(&mut ctx, seed);
    loop {
        let mut buffer: [uint8_t; 500] = [0; 500];
        nettle_knuth_lfib_random(
            &mut ctx,
            ::core::mem::size_of::<[uint8_t; 500]>() as libc::c_ulong,
            buffer.as_mut_ptr(),
        );
        if fwrite(
            buffer.as_mut_ptr() as *const libc::c_void,
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<[uint8_t; 500]>() as libc::c_ulong,
            stdout,
        ) < ::core::mem::size_of::<[uint8_t; 500]>() as libc::c_ulong
            || fflush(stdout) < 0 as libc::c_int
        {
            return 1 as libc::c_int;
        }
    };
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
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
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}

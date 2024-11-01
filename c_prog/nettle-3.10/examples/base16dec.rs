#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn nettle_base16_decode_init(ctx: *mut base16_decode_ctx);
    fn nettle_base16_decode_update(
        ctx: *mut base16_decode_ctx,
        dst_length: *mut size_t,
        dst: *mut uint8_t,
        src_length: size_t,
        src: *const libc::c_char,
    ) -> libc::c_int;
    fn nettle_base16_decode_final(ctx: *mut base16_decode_ctx) -> libc::c_int;
    fn xalloc(size: size_t) -> *mut libc::c_void;
    fn werror(format: *const libc::c_char, _: ...);
    fn write_data(f: *mut FILE, size: size_t, data: *const libc::c_void) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base16_decode_ctx {
    pub word: libc::c_uchar,
    pub bits: libc::c_uchar,
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut buffer: *mut libc::c_char = xalloc(16392 as libc::c_int as size_t)
        as *mut libc::c_char;
    let mut result: *mut uint8_t = xalloc(
        ((16392 as libc::c_int + 1 as libc::c_int) / 2 as libc::c_int) as size_t,
    ) as *mut uint8_t;
    let mut b16_ctx: base16_decode_ctx = base16_decode_ctx {
        word: 0,
        bits: 0,
    };
    nettle_base16_decode_init(&mut b16_ctx);
    loop {
        let mut nbytes: libc::c_int = 0;
        let mut decoded_bytes: size_t = 0;
        nbytes = fread(
            buffer as *mut libc::c_void,
            1 as libc::c_int as size_t,
            16392 as libc::c_int as size_t,
            stdin,
        ) as libc::c_int;
        if nbytes < 16392 as libc::c_int && ferror(stdin) != 0 {
            werror(
                b"Error reading file: %s\n\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return 1 as libc::c_int;
        }
        if nettle_base16_decode_update(
            &mut b16_ctx,
            &mut decoded_bytes,
            result,
            nbytes as size_t,
            buffer,
        ) == 0
        {
            werror(
                b"Error decoding input (not base16?)\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        if write_data(stdout, decoded_bytes, result as *const libc::c_void) == 0 {
            werror(
                b"Error writing file: %s\n\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return 1 as libc::c_int;
        }
        if !(nbytes < 16392 as libc::c_int) {
            continue;
        }
        if nettle_base16_decode_final(&mut b16_ctx) == 0 {
            werror(
                b"Decoding did not finish properly.\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        break;
    }
    if fflush(stdout) != 0 as libc::c_int {
        werror(
            b"Error writing file: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return 1 as libc::c_int;
    }
    free(buffer as *mut libc::c_void);
    free(result as *mut libc::c_void);
    return 0 as libc::c_int;
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

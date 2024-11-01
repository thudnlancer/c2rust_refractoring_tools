#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
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
    fn nettle_base64_encode_init(ctx: *mut base64_encode_ctx);
    fn nettle_base64_encode_update(
        ctx: *mut base64_encode_ctx,
        dst: *mut libc::c_char,
        length: size_t,
        src: *const uint8_t,
    ) -> size_t;
    fn nettle_base64_encode_final(
        ctx: *mut base64_encode_ctx,
        dst: *mut libc::c_char,
    ) -> size_t;
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
pub struct base64_encode_ctx {
    pub alphabet: *const libc::c_char,
    pub word: libc::c_ushort,
    pub bits: libc::c_uchar,
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut b64_ctx: base64_encode_ctx = base64_encode_ctx {
        alphabet: 0 as *const libc::c_char,
        word: 0,
        bits: 0,
    };
    nettle_base64_encode_init(&mut b64_ctx);
    loop {
        let mut buffer: [uint8_t; 54] = [0; 54];
        let mut result: [libc::c_char; 76] = [0; 76];
        let mut nbytes: libc::c_uint = 0;
        let mut encoded_bytes: libc::c_int = 0;
        nbytes = fread(
            buffer.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as size_t,
            54 as libc::c_int as size_t,
            stdin,
        ) as libc::c_uint;
        encoded_bytes = nettle_base64_encode_update(
            &mut b64_ctx,
            result.as_mut_ptr(),
            nbytes as size_t,
            buffer.as_mut_ptr(),
        ) as libc::c_int;
        if nbytes < 54 as libc::c_int as libc::c_uint {
            if ferror(stdin) != 0 {
                werror(
                    b"Error reading file: %s\n\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                return 1 as libc::c_int;
            }
            encoded_bytes = (encoded_bytes as libc::c_ulong)
                .wrapping_add(
                    nettle_base64_encode_final(
                        &mut b64_ctx,
                        result.as_mut_ptr().offset(encoded_bytes as isize),
                    ),
                ) as libc::c_int as libc::c_int;
            let fresh0 = encoded_bytes;
            encoded_bytes = encoded_bytes + 1;
            result[fresh0 as usize] = '\n' as i32 as libc::c_char;
            if write_data(
                stdout,
                encoded_bytes as size_t,
                result.as_mut_ptr() as *const libc::c_void,
            ) == 0 || fflush(stdout) != 0 as libc::c_int
            {
                werror(
                    b"Error writing file: %s\n\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
                return 1 as libc::c_int;
            }
            return 0 as libc::c_int;
        }
        let fresh1 = encoded_bytes;
        encoded_bytes = encoded_bytes + 1;
        result[fresh1 as usize] = '\n' as i32 as libc::c_char;
        if write_data(
            stdout,
            encoded_bytes as size_t,
            result.as_mut_ptr() as *const libc::c_void,
        ) == 0
        {
            werror(
                b"Error writing file: %s\n\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
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

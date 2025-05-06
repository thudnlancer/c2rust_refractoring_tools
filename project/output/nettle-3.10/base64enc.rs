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
    fn __errno_location() -> *mut i32;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn ferror(__stream: *mut FILE) -> i32;
    fn strerror(_: i32) -> *mut i8;
    fn nettle_base64_encode_init(ctx: *mut base64_encode_ctx);
    fn nettle_base64_encode_update(
        ctx: *mut base64_encode_ctx,
        dst: *mut i8,
        length: size_t,
        src: *const uint8_t,
    ) -> size_t;
    fn nettle_base64_encode_final(ctx: *mut base64_encode_ctx, dst: *mut i8) -> size_t;
    fn werror(format: *const i8, _: ...);
    fn write_data(f: *mut FILE, size: size_t, data: *const libc::c_void) -> i32;
}
pub type size_t = u64;
pub type __uint8_t = u8;
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
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_encode_ctx {
    pub alphabet: *const i8,
    pub word: libc::c_ushort,
    pub bits: u8,
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut b64_ctx: base64_encode_ctx = base64_encode_ctx {
        alphabet: 0 as *const i8,
        word: 0,
        bits: 0,
    };
    nettle_base64_encode_init(&mut b64_ctx);
    loop {
        let mut buffer: [uint8_t; 54] = [0; 54];
        let mut result: [i8; 76] = [0; 76];
        let mut nbytes: u32 = 0;
        let mut encoded_bytes: i32 = 0;
        nbytes = fread(
            buffer.as_mut_ptr() as *mut libc::c_void,
            1 as i32 as size_t,
            54 as i32 as size_t,
            stdin,
        ) as u32;
        encoded_bytes = nettle_base64_encode_update(
            &mut b64_ctx,
            result.as_mut_ptr(),
            nbytes as size_t,
            buffer.as_mut_ptr(),
        ) as i32;
        if nbytes < 54 as i32 as u32 {
            if ferror(stdin) != 0 {
                werror(
                    b"Error reading file: %s\n\0" as *const u8 as *const i8,
                    strerror(*__errno_location()),
                );
                return 1 as i32;
            }
            encoded_bytes = (encoded_bytes as u64)
                .wrapping_add(
                    nettle_base64_encode_final(
                        &mut b64_ctx,
                        result.as_mut_ptr().offset(encoded_bytes as isize),
                    ),
                ) as i32 as i32;
            let fresh0 = encoded_bytes;
            encoded_bytes = encoded_bytes + 1;
            result[fresh0 as usize] = '\n' as i32 as i8;
            if write_data(
                stdout,
                encoded_bytes as size_t,
                result.as_mut_ptr() as *const libc::c_void,
            ) == 0 || fflush(stdout) != 0 as i32
            {
                werror(
                    b"Error writing file: %s\n\0" as *const u8 as *const i8,
                    strerror(*__errno_location()),
                );
                return 1 as i32;
            }
            return 0 as i32;
        }
        let fresh1 = encoded_bytes;
        encoded_bytes = encoded_bytes + 1;
        result[fresh1 as usize] = '\n' as i32 as i8;
        if write_data(
            stdout,
            encoded_bytes as size_t,
            result.as_mut_ptr() as *const libc::c_void,
        ) == 0
        {
            werror(
                b"Error writing file: %s\n\0" as *const u8 as *const i8,
                strerror(*__errno_location()),
            );
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
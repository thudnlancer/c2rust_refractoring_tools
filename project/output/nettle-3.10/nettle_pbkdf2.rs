#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    fn __errno_location() -> *mut i32;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn ferror(__stream: *mut FILE) -> i32;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn strdup(_: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn nettle_pbkdf2_hmac_sha256(
        key_length: size_t,
        key: *const uint8_t,
        iterations: u32,
        salt_length: size_t,
        salt: *const uint8_t,
        length: size_t,
        dst: *mut uint8_t,
    );
    fn nettle_base16_encode_update(dst: *mut i8, length: size_t, src: *const uint8_t);
    fn nettle_base16_decode_init(ctx: *mut base16_decode_ctx);
    fn nettle_base16_decode_update(
        ctx: *mut base16_decode_ctx,
        dst_length: *mut size_t,
        dst: *mut uint8_t,
        src_length: size_t,
        src: *const i8,
    ) -> i32;
    fn nettle_base16_decode_final(ctx: *mut base16_decode_ctx) -> i32;
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn getopt_long(
        ___argc: i32,
        ___argv: *const *mut i8,
        __shortopts: *const i8,
        __longopts: *const option,
        __longind: *mut i32,
    ) -> i32;
    fn die(format: *const i8, _: ...) -> !;
    fn xalloc(size: size_t) -> *mut libc::c_void;
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
pub struct base16_decode_ctx {
    pub word: u8,
    pub bits: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    OPT_HEX_SALT = 770,
    OPT_RAW = 769,
    OPT_HELP = 768,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::OPT_HEX_SALT => 770,
            C2RustUnnamed::OPT_RAW => 769,
            C2RustUnnamed::OPT_HELP => 768,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            770 => C2RustUnnamed::OPT_HEX_SALT,
            769 => C2RustUnnamed::OPT_RAW,
            768 => C2RustUnnamed::OPT_HELP,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
unsafe extern "C" fn usage(mut f: *mut FILE) {
    fprintf(
        f,
        b"Usage: nettle-pbkdf2 [OPTIONS] SALT\nOptions:\n  --help                 Show this help.\n  -V, --version          Show version information.\n  -i, --iterations=COUNT Desired iteration count (default %d).\n  -l, --length=LENGTH    Desired output length (octets, default %d)\n  --raw                  Raw binary output.\n  --hex-salt             Use hex encoding for the salt.\n\0"
            as *const u8 as *const i8,
        10000 as i32,
        16 as i32,
    );
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut iterations: u32 = 10000 as i32 as u32;
    let mut output_length: u32 = 16 as i32 as u32;
    let mut password: [i8; 1024] = [0; 1024];
    let mut password_length: size_t = 0;
    let mut output: *mut uint8_t = 0 as *mut uint8_t;
    let mut salt_length: size_t = 0;
    let mut salt: *mut i8 = 0 as *mut i8;
    let mut raw: i32 = 0 as i32;
    let mut hex_salt: i32 = 0 as i32;
    let mut c: i32 = 0;
    static mut options: [option; 7] = [
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: C2RustUnnamed::OPT_HELP as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'V' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"length\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'l' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"iterations\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'i' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"raw\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: C2RustUnnamed::OPT_RAW as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"hex-salt\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: C2RustUnnamed::OPT_HEX_SALT as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 0 as i32,
            };
            init
        },
    ];
    loop {
        c = getopt_long(
            argc,
            argv,
            b"Vl:i:\0" as *const u8 as *const i8,
            options.as_ptr(),
            0 as *mut i32,
        );
        if !(c != -(1 as i32)) {
            break;
        }
        match c {
            63 => {
                usage(stderr);
                return 1 as i32;
            }
            768 => {
                usage(stdout);
                return 0 as i32;
            }
            86 => {
                printf(b"nettle-pbkdf2 (nettle 3.10)\n\0" as *const u8 as *const i8);
                return 0 as i32;
            }
            108 => {
                let mut arg: i32 = 0;
                arg = atoi(optarg);
                if arg <= 0 as i32 {
                    die(
                        b"Invalid length argument: `%s'\n\0" as *const u8 as *const i8,
                        optarg,
                    );
                }
                output_length = arg as u32;
            }
            105 => {
                let mut arg_0: i32 = 0;
                arg_0 = atoi(optarg);
                if arg_0 <= 0 as i32 {
                    die(
                        b"Invalid iteration count: `%s'\n\0" as *const u8 as *const i8,
                        optarg,
                    );
                }
                iterations = arg_0 as u32;
            }
            769 => {
                raw = 1 as i32;
            }
            770 => {
                hex_salt = 1 as i32;
            }
            _ => {
                abort();
            }
        }
    }
    argv = argv.offset(optind as isize);
    argc -= optind;
    if argc != 1 as i32 {
        usage(stderr);
        return 1 as i32;
    }
    salt = strdup(*argv.offset(0 as i32 as isize));
    if salt.is_null() {
        die(b"strdup failed: Virtual memory exhausted.\n\0" as *const u8 as *const i8);
    }
    salt_length = strlen(*argv.offset(0 as i32 as isize));
    if hex_salt != 0 {
        let mut base16: base16_decode_ctx = base16_decode_ctx {
            word: 0,
            bits: 0,
        };
        nettle_base16_decode_init(&mut base16);
        if nettle_base16_decode_update(
            &mut base16,
            &mut salt_length,
            salt as *mut uint8_t,
            salt_length,
            salt,
        ) == 0 || nettle_base16_decode_final(&mut base16) == 0
        {
            die(b"Invalid salt (expecting hex encoding).\n\0" as *const u8 as *const i8);
        }
    }
    password_length = fread(
        password.as_mut_ptr() as *mut libc::c_void,
        1 as i32 as size_t,
        ::core::mem::size_of::<[i8; 1024]>() as u64,
        stdin,
    );
    if password_length == ::core::mem::size_of::<[i8; 1024]>() as u64 {
        die(
            b"Password input too long. Current limit is %d characters.\n\0" as *const u8
                as *const i8,
            ::core::mem::size_of::<[i8; 1024]>() as u64 as i32 - 1 as i32,
        );
    }
    if ferror(stdin) != 0 {
        die(
            b"Reading password input failed: %s.\n\0" as *const u8 as *const i8,
            strerror(*__errno_location()),
        );
    }
    output = xalloc(output_length as size_t) as *mut uint8_t;
    nettle_pbkdf2_hmac_sha256(
        password_length,
        password.as_mut_ptr() as *const uint8_t,
        iterations,
        salt_length,
        salt as *const uint8_t,
        output_length as size_t,
        output,
    );
    free(salt as *mut libc::c_void);
    if raw != 0 {
        fwrite(
            output as *const libc::c_void,
            output_length as size_t,
            1 as i32 as size_t,
            stdout,
        );
    } else {
        let mut i: u32 = 0;
        let mut hex: [i8; 17] = [0; 17];
        i = 0 as i32 as u32;
        while i.wrapping_add(8 as i32 as u32) < output_length {
            nettle_base16_encode_update(
                hex.as_mut_ptr(),
                8 as i32 as size_t,
                output.offset(i as isize),
            );
            hex[(8 as i32 * 2 as i32) as usize] = 0 as i32 as i8;
            printf(
                b"%s%c\0" as *const u8 as *const i8,
                hex.as_mut_ptr(),
                if i.wrapping_rem(64 as i32 as u32) == 56 as i32 as u32 {
                    '\n' as i32
                } else {
                    ' ' as i32
                },
            );
            i = i.wrapping_add(8 as i32 as u32);
        }
        nettle_base16_encode_update(
            hex.as_mut_ptr(),
            output_length.wrapping_sub(i) as size_t,
            output.offset(i as isize),
        );
        hex[output_length.wrapping_sub(i).wrapping_mul(2 as i32 as u32) as usize] = 0
            as i32 as i8;
        printf(b"%s\n\0" as *const u8 as *const i8, hex.as_mut_ptr());
    }
    free(output as *mut libc::c_void);
    if fflush(stdout) != 0 as i32 {
        die(
            b"Write failed: %s\n\0" as *const u8 as *const i8,
            strerror(*__errno_location()),
        );
    }
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
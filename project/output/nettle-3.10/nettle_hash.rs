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
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
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
    fn feof(__stream: *mut FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn strerror(_: i32) -> *mut i8;
    fn nettle_get_hashes() -> *const *const nettle_hash;
    fn nettle_lookup_hash(name: *const i8) -> *const nettle_hash;
    fn nettle_base16_encode_update(dst: *mut i8, length: size_t, src: *const uint8_t);
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
pub type nettle_hash_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type nettle_hash_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *const uint8_t,
) -> ();
pub type nettle_hash_digest_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_hash {
    pub name: *const i8,
    pub context_size: u32,
    pub digest_size: u32,
    pub block_size: u32,
    pub init: Option<nettle_hash_init_func>,
    pub update: Option<nettle_hash_update_func>,
    pub digest: Option<nettle_hash_digest_func>,
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
    OPT_LIST = 770,
    OPT_RAW = 769,
    OPT_HELP = 768,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::OPT_LIST => 770,
            C2RustUnnamed::OPT_RAW => 769,
            C2RustUnnamed::OPT_HELP => 768,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            770 => C2RustUnnamed::OPT_LIST,
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
unsafe extern "C" fn list_algorithms() {
    let mut i: u32 = 0;
    let mut alg: *const nettle_hash = 0 as *const nettle_hash;
    printf(
        b"%10s digestsize (internal block size, context size), in units of octets\n\0"
            as *const u8 as *const i8,
        b"name\0" as *const u8 as *const i8,
    );
    i = 0 as i32 as u32;
    loop {
        alg = *(nettle_get_hashes()).offset(i as isize);
        if alg.is_null() {
            break;
        }
        printf(
            b"%10s %d (%d, %d)\n\0" as *const u8 as *const i8,
            (*alg).name,
            (*alg).digest_size,
            (*alg).block_size,
            (*alg).context_size,
        );
        i = i.wrapping_add(1);
        i;
    };
}
unsafe extern "C" fn hash_file(
    mut hash: *const nettle_hash,
    mut ctx: *mut libc::c_void,
    mut f: *mut FILE,
) -> i32 {
    loop {
        let mut buffer: [uint8_t; 16384] = [0; 16384];
        let mut res: size_t = fread(
            buffer.as_mut_ptr() as *mut libc::c_void,
            1 as i32 as size_t,
            ::core::mem::size_of::<[uint8_t; 16384]>() as u64,
            f,
        );
        if ferror(f) != 0 {
            return 0 as i32;
        }
        ((*hash).update)
            .expect("non-null function pointer")(ctx, res, buffer.as_mut_ptr());
        if feof(f) != 0 {
            return 1 as i32;
        }
    };
}
unsafe extern "C" fn digest_file(
    mut alg: *const nettle_hash,
    mut digest_length: u32,
    mut raw: i32,
    mut f: *mut FILE,
) -> i32 {
    let mut ctx: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut digest: *mut uint8_t = 0 as *mut uint8_t;
    ctx = xalloc((*alg).context_size as size_t);
    ((*alg).init).expect("non-null function pointer")(ctx);
    if hash_file(alg, ctx, f) == 0 {
        free(ctx);
        return 0 as i32;
    }
    digest = xalloc(digest_length as size_t) as *mut uint8_t;
    ((*alg).digest)
        .expect("non-null function pointer")(ctx, digest_length as size_t, digest);
    free(ctx);
    if raw != 0 {
        fwrite(
            digest as *const libc::c_void,
            digest_length as size_t,
            1 as i32 as size_t,
            stdout,
        );
    } else {
        let mut i: u32 = 0;
        let mut hex: [i8; 17] = [0; 17];
        i = 0 as i32 as u32;
        while i.wrapping_add(8 as i32 as u32) < digest_length {
            nettle_base16_encode_update(
                hex.as_mut_ptr(),
                8 as i32 as size_t,
                digest.offset(i as isize),
            );
            hex[(8 as i32 * 2 as i32) as usize] = 0 as i32 as i8;
            printf(b"%s \0" as *const u8 as *const i8, hex.as_mut_ptr());
            i = i.wrapping_add(8 as i32 as u32);
        }
        nettle_base16_encode_update(
            hex.as_mut_ptr(),
            digest_length.wrapping_sub(i) as size_t,
            digest.offset(i as isize),
        );
        hex[digest_length.wrapping_sub(i).wrapping_mul(2 as i32 as u32) as usize] = 0
            as i32 as i8;
        printf(b"%s %s\n\0" as *const u8 as *const i8, hex.as_mut_ptr(), (*alg).name);
    }
    free(digest as *mut libc::c_void);
    return 1 as i32;
}
unsafe extern "C" fn usage(mut f: *mut FILE) {
    fprintf(
        f,
        b"Usage: nettle-hash -a ALGORITHM [OPTIONS] [FILE ...]\nOptions:\n  --help              Show this help.\n  -V, --version       Show version information.\n  --list              List supported hash algorithms.\n  -a, --algorithm=ALG Hash algorithm to use.\n  -l, --length=LENGTH Desired digest length (octets)\n  --raw               Raw binary output.\n\0"
            as *const u8 as *const i8,
    );
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut alg_name: *const i8 = 0 as *const i8;
    let mut alg: *const nettle_hash = 0 as *const nettle_hash;
    let mut length: u32 = 0 as i32 as u32;
    let mut raw: i32 = 0 as i32;
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
                name: b"algorithm\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'a' as i32,
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
                name: b"list\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: C2RustUnnamed::OPT_LIST as i32,
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
            b"Va:l:\0" as *const u8 as *const i8,
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
                printf(b"nettle-hash (nettle 3.10)\n\0" as *const u8 as *const i8);
                return 0 as i32;
            }
            97 => {
                alg_name = optarg;
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
                length = arg as u32;
            }
            769 => {
                raw = 1 as i32;
            }
            770 => {
                list_algorithms();
                return 0 as i32;
            }
            _ => {
                abort();
            }
        }
    }
    if alg_name.is_null() {
        die(
            b"Algorithm argument (-a option) is mandatory.\nSee nettle-hash --help for further information.\n\0"
                as *const u8 as *const i8,
        );
    }
    alg = nettle_lookup_hash(alg_name);
    if alg.is_null() {
        die(
            b"Hash algorithm `%s' not supported or .\nUse nettle-hash --list to list available algorithms.\n\0"
                as *const u8 as *const i8,
            alg_name,
        );
    }
    if length == 0 as i32 as u32 {
        length = (*alg).digest_size;
    } else if length > (*alg).digest_size {
        die(
            b"Length argument %d too large for selected algorithm.\n\0" as *const u8
                as *const i8,
            length,
        );
    }
    argv = argv.offset(optind as isize);
    argc -= optind;
    if argc == 0 as i32 {
        digest_file(alg, length, raw, stdin);
    } else {
        let mut i: i32 = 0;
        i = 0 as i32;
        while i < argc {
            let mut f: *mut FILE = fopen(
                *argv.offset(i as isize),
                b"rb\0" as *const u8 as *const i8,
            );
            if f.is_null() {
                die(
                    b"Cannot open `%s': %s\n\0" as *const u8 as *const i8,
                    *argv.offset(i as isize),
                    strerror(*__errno_location()),
                );
            }
            printf(b"%s: \0" as *const u8 as *const i8, *argv.offset(i as isize));
            if digest_file(alg, length, raw, f) == 0 {
                die(
                    b"Reading `%s' failed: %s\n\0" as *const u8 as *const i8,
                    *argv.offset(i as isize),
                    strerror(*__errno_location()),
                );
            }
            fclose(f);
            i += 1;
            i;
        }
    }
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
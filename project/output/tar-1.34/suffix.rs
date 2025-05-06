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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    static mut use_compress_program_option: *const i8;
}
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compression_suffix {
    pub suffix: *const i8,
    pub length: size_t,
    pub program: *const i8,
}
static mut compression_suffixes: [compression_suffix; 19] = [compression_suffix {
    suffix: 0 as *const i8,
    length: 0,
    program: 0 as *const i8,
}; 19];
unsafe extern "C" fn find_compression_suffix(
    mut name: *const i8,
    mut ret_len: *mut size_t,
) -> *const compression_suffix {
    let mut suf: *mut i8 = strrchr(name, '.' as i32);
    if !suf.is_null() {
        let mut len: size_t = 0;
        let mut p: *mut compression_suffix = 0 as *mut compression_suffix;
        suf = suf.offset(1);
        suf;
        len = strlen(suf);
        p = compression_suffixes.as_mut_ptr();
        while !((*p).suffix).is_null() {
            if (*p).length == len
                && memcmp(
                    (*p).suffix as *const libc::c_void,
                    suf as *const libc::c_void,
                    len,
                ) == 0 as i32
            {
                if !ret_len.is_null() {
                    *ret_len = (strlen(name))
                        .wrapping_sub(len)
                        .wrapping_sub(1 as i32 as u64);
                }
                return p;
            }
            p = p.offset(1);
            p;
        }
    }
    return 0 as *const compression_suffix;
}
unsafe extern "C" fn find_compression_program(
    mut name: *const i8,
    mut defprog: *const i8,
) -> *const i8 {
    let mut p: *const compression_suffix = find_compression_suffix(
        name,
        0 as *mut size_t,
    );
    if !p.is_null() {
        return (*p).program;
    }
    return defprog;
}
#[no_mangle]
pub unsafe extern "C" fn set_compression_program_by_suffix(
    mut name: *const i8,
    mut defprog: *const i8,
) {
    let mut program: *const i8 = find_compression_program(name, defprog);
    if !program.is_null() {
        use_compress_program_option = program;
    }
}
#[no_mangle]
pub unsafe extern "C" fn strip_compression_suffix(mut name: *const i8) -> *mut i8 {
    let mut s: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0;
    let mut p: *const compression_suffix = find_compression_suffix(name, &mut len);
    if !p.is_null() {
        if len > 4 as i32 as u64
            && strncmp(
                name.offset(len as isize).offset(-(4 as i32 as isize)),
                b".tar\0" as *const u8 as *const i8,
                4 as i32 as u64,
            ) == 0 as i32
            && *((*p).suffix).offset(0 as i32 as isize) as i32 != 't' as i32
        {
            len = (len as u64).wrapping_sub(4 as i32 as u64) as size_t as size_t;
        }
        if len == 0 as i32 as u64 {
            return 0 as *mut i8;
        }
        s = xmalloc(len.wrapping_add(1 as i32 as u64)) as *mut i8;
        memcpy(s as *mut libc::c_void, name as *const libc::c_void, len);
        *s.offset(len as isize) = 0 as i32 as i8;
    }
    return s;
}
unsafe extern "C" fn run_static_initializers() {
    compression_suffixes = [
        {
            let mut init = compression_suffix {
                suffix: b"tar\0" as *const u8 as *const i8,
                length: 3 as i32 as size_t,
                program: 0 as *const i8,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"gz\0" as *const u8 as *const i8,
                length: (::core::mem::size_of::<[i8; 3]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                program: b"gzip\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"tgz\0" as *const u8 as *const i8,
                length: (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                program: b"gzip\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"taz\0" as *const u8 as *const i8,
                length: (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                program: b"gzip\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"Z\0" as *const u8 as *const i8,
                length: (::core::mem::size_of::<[i8; 2]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                program: b"compress\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"taZ\0" as *const u8 as *const i8,
                length: (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                program: b"compress\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"bz2\0" as *const u8 as *const i8,
                length: (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                program: b"bzip2\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"tbz\0" as *const u8 as *const i8,
                length: (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                program: b"bzip2\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"tbz2\0" as *const u8 as *const i8,
                length: (::core::mem::size_of::<[i8; 5]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                program: b"bzip2\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"tz2\0" as *const u8 as *const i8,
                length: (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                program: b"bzip2\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"lz\0" as *const u8 as *const i8,
                length: (::core::mem::size_of::<[i8; 3]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                program: b"lzip\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"lzma\0" as *const u8 as *const i8,
                length: (::core::mem::size_of::<[i8; 5]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                program: b"lzma\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"tlz\0" as *const u8 as *const i8,
                length: (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                program: b"lzma\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"lzo\0" as *const u8 as *const i8,
                length: (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                program: b"lzop\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"xz\0" as *const u8 as *const i8,
                length: (::core::mem::size_of::<[i8; 3]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                program: b"xz\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"txz\0" as *const u8 as *const i8,
                length: (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                program: b"xz\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"zst\0" as *const u8 as *const i8,
                length: (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                program: b"zstd\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: b"tzst\0" as *const u8 as *const i8,
                length: (::core::mem::size_of::<[i8; 5]>() as u64)
                    .wrapping_sub(1 as i32 as u64),
                program: b"zstd\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = compression_suffix {
                suffix: 0 as *const i8,
                length: 0,
                program: 0 as *const i8,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
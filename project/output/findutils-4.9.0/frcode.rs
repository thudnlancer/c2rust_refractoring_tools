#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn __errno_location() -> *mut i32;
    fn getopt_long(
        ___argc: i32,
        ___argv: *const *mut i8,
        __shortopts: *const i8,
        __longopts: *const option,
        __longind: *mut i32,
    ) -> i32;
    static mut optarg: *mut i8;
    static mut optind: i32;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    fn getdelim(
        __lineptr: *mut *mut i8,
        __n: *mut size_t,
        __delimiter: i32,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fputc(__c: i32, __stream: *mut FILE) -> i32;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> i32;
    fn exit(_: i32) -> !;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn rpl_free(ptr: *mut libc::c_void);
    fn close_stdout();
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    static mut program_name: *const i8;
    fn set_program_name(argv0: *const i8);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn explain_how_to_report_bugs(f: *mut FILE, program_name_0: *const i8) -> i32;
    fn display_findutils_version(official_name: *const i8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __ssize_t = i64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _gl_dummy: i32,
}
unsafe extern "C" fn put_short(mut c: i32, mut fp: *mut FILE) -> i32 {
    if c <= 32767 as i32 {} else {
        __assert_fail(
            b"c <= SHRT_MAX\0" as *const u8 as *const i8,
            b"frcode.c\0" as *const u8 as *const i8,
            104 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[i8; 27],
            >(b"int put_short(int, FILE *)\0"))
                .as_ptr(),
        );
    }
    'c_3306: {
        if c <= 32767 as i32 {} else {
            __assert_fail(
                b"c <= SHRT_MAX\0" as *const u8 as *const i8,
                b"frcode.c\0" as *const u8 as *const i8,
                104 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[i8; 27],
                >(b"int put_short(int, FILE *)\0"))
                    .as_ptr(),
            );
        }
    };
    if c >= -(32767 as i32) - 1 as i32 {} else {
        __assert_fail(
            b"c >= SHRT_MIN\0" as *const u8 as *const i8,
            b"frcode.c\0" as *const u8 as *const i8,
            105 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[i8; 27],
            >(b"int put_short(int, FILE *)\0"))
                .as_ptr(),
        );
    }
    'c_3257: {
        if c >= -(32767 as i32) - 1 as i32 {} else {
            __assert_fail(
                b"c >= SHRT_MIN\0" as *const u8 as *const i8,
                b"frcode.c\0" as *const u8 as *const i8,
                105 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[i8; 27],
                >(b"int put_short(int, FILE *)\0"))
                    .as_ptr(),
            );
        }
    };
    return (_IO_putc(c >> 8 as i32, fp) != -(1 as i32) && _IO_putc(c, fp) != -(1 as i32))
        as i32;
}
unsafe extern "C" fn prefix_length(mut s1: *mut i8, mut s2: *mut i8) -> i32 {
    let mut start: *mut i8 = 0 as *mut i8;
    let mut limit: i32 = 2147483647 as i32;
    start = s1;
    while *s1 as i32 == *s2 as i32 && *s1 as i32 != '\0' as i32 {
        limit -= 1;
        if 0 as i32 == limit {
            break;
        }
        s1 = s1.offset(1);
        s1;
        s2 = s2.offset(1);
        s2;
    }
    return s1.offset_from(start) as i64 as i32;
}
static mut longopts: [option; 4] = [
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"null\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: '0' as i32,
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
unsafe extern "C" fn usage(mut status: i32) -> ! {
    if status != 0 as i32 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Try '%s --help' for more information.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            program_name,
        );
        exit(status);
    }
    fprintf(
        stdout,
        dcgettext(
            0 as *const i8,
            b"Usage: %s [-0 | --null] [--version] [--help]\n\0" as *const u8
                as *const i8,
            5 as i32,
        ),
        program_name,
    );
    explain_how_to_report_bugs(stdout, program_name);
    exit(status);
}
unsafe extern "C" fn get_seclevel(mut s: *mut i8) -> i64 {
    let mut result: i64 = 0;
    let mut p: *mut i8 = 0 as *mut i8;
    *__errno_location() = 0 as i32;
    result = strtol(s, &mut p, 10 as i32);
    if 0 as i32 as i64 == result && p == optarg {
        if ::core::mem::size_of::<C2RustUnnamed_1>() as u64 != 0 {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"You need to specify a security level as a decimal integer.\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"You need to specify a security level as a decimal integer.\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
        return -(1 as i32) as i64;
    } else if (-(9223372036854775807 as i64) - 1 as i64 == result
        || 9223372036854775807 as i64 == result) && *__errno_location() != 0
    {
        if ::core::mem::size_of::<C2RustUnnamed_0>() as u64 != 0 {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Security level %s is outside the convertible range.\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                s,
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Security level %s is outside the convertible range.\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                s,
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
        return -(1 as i32) as i64;
    } else if *p != 0 {
        if ::core::mem::size_of::<C2RustUnnamed>() as u64 != 0 {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Security level %s has unexpected suffix %s.\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                s,
                p,
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Security level %s has unexpected suffix %s.\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                s,
                p,
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
        return -(1 as i32) as i64;
    } else {
        return result
    };
}
unsafe extern "C" fn outerr() {
    if ::core::mem::size_of::<C2RustUnnamed_2>() as u64 != 0 {
        error(
            1 as i32,
            *__errno_location(),
            dcgettext(
                0 as *const i8,
                b"write error\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        if 0 as i32 != 0 {} else {
            unreachable!();
        };
    } else {
        error(
            1 as i32,
            *__errno_location(),
            dcgettext(
                0 as *const i8,
                b"write error\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        if 0 as i32 != 0 {} else {
            unreachable!();
        };
    };
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut path: *mut i8 = 0 as *mut i8;
    let mut oldpath: *mut i8 = 0 as *mut i8;
    let mut pathsize: size_t = 0;
    let mut oldpathsize: size_t = 0;
    let mut count: i32 = 0;
    let mut oldcount: i32 = 0;
    let mut diffcount: i32 = 0;
    let mut line_len: i32 = 0;
    let mut delimiter: i32 = '\n' as i32;
    let mut optc: i32 = 0;
    let mut slocate_compat: i32 = 0 as i32;
    let mut slocate_seclevel: i64 = 0 as i64;
    if !(*argv.offset(0 as i32 as isize)).is_null() {
        set_program_name(*argv.offset(0 as i32 as isize));
    } else {
        set_program_name(b"frcode\0" as *const u8 as *const i8);
    }
    if atexit(Some(close_stdout as unsafe extern "C" fn() -> ())) != 0 {
        if ::core::mem::size_of::<C2RustUnnamed_5>() as u64 != 0 {
            error(
                1 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"The atexit library function failed\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"The atexit library function failed\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
    oldpathsize = 1026 as i32 as size_t;
    pathsize = oldpathsize;
    path = xmalloc(pathsize) as *mut i8;
    oldpath = xmalloc(oldpathsize) as *mut i8;
    *oldpath.offset(0 as i32 as isize) = 0 as i32 as i8;
    oldcount = 0 as i32;
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"hv0S:\0" as *const u8 as *const i8,
            longopts.as_ptr(),
            0 as *mut i32,
        );
        if !(optc != -(1 as i32)) {
            break;
        }
        match optc {
            48 => {
                delimiter = 0 as i32;
            }
            83 => {
                slocate_compat = 1 as i32;
                slocate_seclevel = get_seclevel(optarg);
                if slocate_seclevel < 0 as i32 as i64
                    || slocate_seclevel > 1 as i32 as i64
                {
                    if ::core::mem::size_of::<C2RustUnnamed_4>() as u64 != 0 {
                        error(
                            1 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"slocate security level %ld is unsupported.\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            slocate_seclevel,
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    } else {
                        error(
                            1 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"slocate security level %ld is unsupported.\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            slocate_seclevel,
                        );
                        if 0 as i32 != 0 {} else {
                            unreachable!();
                        };
                    };
                }
            }
            104 => {
                usage(0 as i32);
            }
            118 => {
                display_findutils_version(b"frcode\0" as *const u8 as *const i8);
                return 0 as i32;
            }
            _ => {
                usage(1 as i32);
            }
        }
    }
    if optind != argc {
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"no argument expected.\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        usage(1 as i32);
    }
    if slocate_compat != 0 {
        fputc(if slocate_seclevel != 0 { '1' as i32 } else { '0' as i32 }, stdout);
        fputc(0 as i32, stdout);
    } else if fwrite(
        b"\0LOCATE02\0" as *const u8 as *const i8 as *const libc::c_void,
        1 as i32 as size_t,
        ::core::mem::size_of::<[i8; 10]>() as u64,
        stdout,
    ) != ::core::mem::size_of::<[i8; 10]>() as u64
    {
        if ::core::mem::size_of::<C2RustUnnamed_3>() as u64 != 0 {
            error(
                1 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"Failed to write to standard output\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        } else {
            error(
                1 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"Failed to write to standard output\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
    loop {
        line_len = getdelim(&mut path, &mut pathsize, delimiter, stdin) as i32;
        if !(line_len > 0 as i32) {
            break;
        }
        if *path.offset((line_len - 1 as i32) as isize) as i32 != delimiter {
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"The input file should end with the delimiter\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        } else {
            *path.offset((line_len - 1 as i32) as isize) = '\0' as i32 as i8;
        }
        count = prefix_length(oldpath, path);
        diffcount = count - oldcount;
        if diffcount > 32767 as i32 || diffcount < -(32767 as i32) - 1 as i32 {
            count = 0 as i32;
            diffcount = -oldcount;
        }
        oldcount = count;
        if slocate_compat != 0 {
            slocate_compat = 0 as i32;
        } else if diffcount < -(127 as i32) || diffcount > 127 as i32 {
            if -(1 as i32) == _IO_putc(0x80 as i32, stdout) {
                outerr();
            }
            if put_short(diffcount, stdout) == 0 {
                outerr();
            }
        } else if -(1 as i32) == _IO_putc(diffcount, stdout) {
            outerr();
        }
        if -(1 as i32) == fputs(path.offset(count as isize), stdout)
            || -(1 as i32) == _IO_putc('\0' as i32, stdout)
        {
            outerr();
        }
        let mut tmppath: *mut i8 = oldpath;
        let mut tmppathsize: size_t = oldpathsize;
        oldpath = path;
        oldpathsize = pathsize;
        path = tmppath;
        pathsize = tmppathsize;
    }
    rpl_free(path as *mut libc::c_void);
    rpl_free(oldpath as *mut libc::c_void);
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
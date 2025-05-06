#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic)]
extern "C" {
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn vsprintf(_: *mut i8, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn perror(__s: *const i8);
    fn fileno(__stream: *mut FILE) -> i32;
    fn __errno_location() -> *mut i32;
    fn ftruncate(__fd: i32, __length: __off_t) -> i32;
    fn exit(_: i32) -> !;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn writebuf(_: i32, _: *const libc::c_void, _: size_t) -> ssize_t;
    static mut starting_directory: *mut i8;
    static mut makelevel: u32;
    static mut program: *const i8;
    static mut print_data_base_flag: i32;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn should_print_dir() -> i32;
    static mut output_sync: i32;
    fn get_tmpfd(_: *mut *mut i8) -> i32;
    fn die(_: i32) -> !;
    fn check_io_state() -> u32;
    fn fd_noinherit(_: i32);
    fn fd_set_append(_: i32);
    fn osync_clear();
    fn osync_acquire() -> u32;
    fn osync_release();
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = u64;
pub type __uintmax_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __ssize_t = i64;
pub type ssize_t = __ssize_t;
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
pub type va_list = __builtin_va_list;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const i8,
    pub lineno: u64,
    pub offset: u64,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct output {
    pub out: i32,
    pub err: i32,
    #[bitfield(name = "syncout", ty = "libc::c_uint", bits = "0..=0")]
    pub syncout: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fmtstring {
    pub buffer: *mut i8,
    pub size: size_t,
}
#[no_mangle]
pub static mut output_context: *mut output = 0 as *const output as *mut output;
#[no_mangle]
pub static mut stdio_traced: u32 = 0 as i32 as u32;
unsafe extern "C" fn _outputs(
    mut out: *mut output,
    mut is_err: i32,
    mut msg: *const i8,
) {
    let mut f: *mut FILE = 0 as *mut FILE;
    if !out.is_null() && (*out).syncout() as i32 != 0 {
        let mut fd: i32 = if is_err != 0 { (*out).err } else { (*out).out };
        if fd != -(1 as i32) {
            let mut len: size_t = strlen(msg);
            let mut r: i32 = 0;
            loop {
                r = lseek(fd, 0 as i32 as __off_t, 2 as i32) as i32;
                if !(r == -(1 as i32) && *__errno_location() == 4 as i32) {
                    break;
                }
            }
            writebuf(fd, msg as *const libc::c_void, len);
            return;
        }
    }
    f = if is_err != 0 { stderr } else { stdout };
    fputs(msg, f);
    fflush(f);
}
unsafe extern "C" fn log_working_directory(mut entering: i32) -> i32 {
    static mut buf: *mut i8 = 0 as *const i8 as *mut i8;
    static mut len: size_t = 0 as i32 as size_t;
    let mut need: size_t = 0;
    let mut fmt: *const i8 = 0 as *const i8;
    let mut p: *mut i8 = 0 as *mut i8;
    need = (strlen(program))
        .wrapping_add(
            (53 as i32 as u64)
                .wrapping_mul(::core::mem::size_of::<uintmax_t>() as u64)
                .wrapping_div(22 as i32 as u64)
                .wrapping_add(3 as i32 as u64),
        )
        .wrapping_add(2 as i32 as u64)
        .wrapping_add(1 as i32 as u64);
    if !starting_directory.is_null() {
        need = (need as u64).wrapping_add(strlen(starting_directory)) as size_t
            as size_t;
    }
    if makelevel == 0 as i32 as u32 {
        if starting_directory.is_null() {
            if entering != 0 {
                fmt = dcgettext(
                    0 as *const i8,
                    b"%s: Entering an unknown directory\n\0" as *const u8 as *const i8,
                    5 as i32,
                );
            } else {
                fmt = dcgettext(
                    0 as *const i8,
                    b"%s: Leaving an unknown directory\n\0" as *const u8 as *const i8,
                    5 as i32,
                );
            }
        } else if entering != 0 {
            fmt = dcgettext(
                0 as *const i8,
                b"%s: Entering directory '%s'\n\0" as *const u8 as *const i8,
                5 as i32,
            );
        } else {
            fmt = dcgettext(
                0 as *const i8,
                b"%s: Leaving directory '%s'\n\0" as *const u8 as *const i8,
                5 as i32,
            );
        }
    } else if starting_directory.is_null() {
        if entering != 0 {
            fmt = dcgettext(
                0 as *const i8,
                b"%s[%u]: Entering an unknown directory\n\0" as *const u8 as *const i8,
                5 as i32,
            );
        } else {
            fmt = dcgettext(
                0 as *const i8,
                b"%s[%u]: Leaving an unknown directory\n\0" as *const u8 as *const i8,
                5 as i32,
            );
        }
    } else if entering != 0 {
        fmt = dcgettext(
            0 as *const i8,
            b"%s[%u]: Entering directory '%s'\n\0" as *const u8 as *const i8,
            5 as i32,
        );
    } else {
        fmt = dcgettext(
            0 as *const i8,
            b"%s[%u]: Leaving directory '%s'\n\0" as *const u8 as *const i8,
            5 as i32,
        );
    }
    need = (need as u64).wrapping_add(strlen(fmt)) as size_t as size_t;
    if need > len {
        buf = xrealloc(buf as *mut libc::c_void, need) as *mut i8;
        len = need;
    }
    p = buf;
    if print_data_base_flag != 0 {
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = '#' as i32 as i8;
        let fresh1 = p;
        p = p.offset(1);
        *fresh1 = ' ' as i32 as i8;
    }
    if makelevel == 0 as i32 as u32 {
        if starting_directory.is_null() {
            sprintf(p, fmt, program);
        } else {
            sprintf(p, fmt, program, starting_directory);
        }
    } else if starting_directory.is_null() {
        sprintf(p, fmt, program, makelevel);
    } else {
        sprintf(p, fmt, program, makelevel, starting_directory);
    }
    _outputs(0 as *mut output, 0 as i32, buf);
    return 1 as i32;
}
unsafe extern "C" fn pump_from_tmp(mut from: i32, mut to: *mut FILE) {
    static mut buffer: [i8; 8192] = [0; 8192];
    if lseek(from, 0 as i32 as __off_t, 0 as i32) == -(1 as i32) as i64 {
        perror(b"lseek()\0" as *const u8 as *const i8);
    }
    loop {
        let mut len: i32 = 0;
        loop {
            len = read(
                from,
                buffer.as_mut_ptr() as *mut libc::c_void,
                ::core::mem::size_of::<[i8; 8192]>() as u64,
            ) as i32;
            if !(len == -(1 as i32) && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if len < 0 as i32 {
            perror(b"read()\0" as *const u8 as *const i8);
        }
        if len <= 0 as i32 {
            break;
        }
        if fwrite(
            buffer.as_mut_ptr() as *const libc::c_void,
            len as size_t,
            1 as i32 as size_t,
            to,
        ) < 1 as i32 as u64
        {
            perror(b"fwrite()\0" as *const u8 as *const i8);
            break;
        } else {
            fflush(to);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn output_tmpfd() -> i32 {
    let mut fd: i32 = get_tmpfd(0 as *mut *mut i8);
    fd_set_append(fd);
    return fd;
}
unsafe extern "C" fn setup_tmpfile(mut out: *mut output) {
    let mut current_block: u64;
    static mut in_setup: u32 = 0 as i32 as u32;
    let mut io_state: u32 = 0;
    if in_setup != 0 {
        return;
    }
    in_setup = 1 as i32 as u32;
    io_state = check_io_state();
    if !(io_state & (0x8 as i32 | 0x10 as i32) as u32 != 0 as i32 as u32) {
        perror_with_name(
            b"output-sync suppressed: \0" as *const u8 as *const i8,
            b"stderr\0" as *const u8 as *const i8,
        );
    } else {
        if io_state & 0x8 as i32 as u32 != 0 as i32 as u32 {
            let mut fd: i32 = output_tmpfd();
            if fd < 0 as i32 {
                current_block = 13427456408935031643;
            } else {
                fd_noinherit(fd);
                (*out).out = fd;
                current_block = 3276175668257526147;
            }
        } else {
            current_block = 3276175668257526147;
        }
        match current_block {
            13427456408935031643 => {}
            _ => {
                if io_state & 0x10 as i32 as u32 != 0 as i32 as u32 {
                    if (*out).out != -(1 as i32)
                        && io_state & 0x2 as i32 as u32 != 0 as i32 as u32
                    {
                        (*out).err = (*out).out;
                        current_block = 9606288038608642794;
                    } else {
                        let mut fd_0: i32 = output_tmpfd();
                        if fd_0 < 0 as i32 {
                            current_block = 13427456408935031643;
                        } else {
                            fd_noinherit(fd_0);
                            (*out).err = fd_0;
                            current_block = 9606288038608642794;
                        }
                    }
                } else {
                    current_block = 9606288038608642794;
                }
                match current_block {
                    13427456408935031643 => {}
                    _ => {
                        in_setup = 0 as i32 as u32;
                        return;
                    }
                }
            }
        }
    }
    error(
        0 as *mut floc,
        0 as i32 as size_t,
        dcgettext(
            0 as *const i8,
            b"cannot open output-sync lock file, suppressing output-sync.\0" as *const u8
                as *const i8,
            5 as i32,
        ),
    );
    output_close(out);
    output_sync = 0 as i32;
    osync_clear();
    in_setup = 0 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn output_dump(mut out: *mut output) {
    let mut outfd_not_empty: i32 = ((*out).out != -(1 as i32)
        && lseek((*out).out, 0 as i32 as __off_t, 2 as i32) > 0 as i32 as i64) as i32;
    let mut errfd_not_empty: i32 = ((*out).err != -(1 as i32)
        && lseek((*out).err, 0 as i32 as __off_t, 2 as i32) > 0 as i32 as i64) as i32;
    if outfd_not_empty != 0 || errfd_not_empty != 0 {
        let mut traced: i32 = 0 as i32;
        if osync_acquire() == 0 {
            error(
                0 as *mut floc,
                0 as i32 as size_t,
                dcgettext(
                    0 as *const i8,
                    b"warning: Cannot acquire output lock, disabling output sync.\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            osync_clear();
        }
        if output_sync != 3 as i32 && should_print_dir() != 0 {
            traced = log_working_directory(1 as i32);
        }
        if outfd_not_empty != 0 {
            pump_from_tmp((*out).out, stdout);
        }
        if errfd_not_empty != 0 && (*out).err != (*out).out {
            pump_from_tmp((*out).err, stderr);
        }
        if traced != 0 {
            log_working_directory(0 as i32);
        }
        osync_release();
        if (*out).out != -(1 as i32) {
            let mut e: i32 = 0;
            lseek((*out).out, 0 as i32 as __off_t, 0 as i32);
            loop {
                e = ftruncate((*out).out, 0 as i32 as __off_t);
                if !(e == -(1 as i32) && *__errno_location() == 4 as i32) {
                    break;
                }
            }
        }
        if (*out).err != -(1 as i32) && (*out).err != (*out).out {
            let mut e_0: i32 = 0;
            lseek((*out).err, 0 as i32 as __off_t, 0 as i32);
            loop {
                e_0 = ftruncate((*out).err, 0 as i32 as __off_t);
                if !(e_0 == -(1 as i32) && *__errno_location() == 4 as i32) {
                    break;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn output_init(mut out: *mut output) {
    if !out.is_null() {
        (*out).err = -(1 as i32);
        (*out).out = (*out).err;
        (*out).set_syncout((output_sync != 0) as i32 as u32);
        return;
    }
    fd_set_append(fileno(stdout));
    fd_set_append(fileno(stderr));
}
#[no_mangle]
pub unsafe extern "C" fn output_close(mut out: *mut output) {
    if out.is_null() {
        if stdio_traced != 0 {
            log_working_directory(0 as i32);
        }
        return;
    }
    output_dump(out);
    if (*out).out >= 0 as i32 {
        close((*out).out);
    }
    if (*out).err >= 0 as i32 && (*out).err != (*out).out {
        close((*out).err);
    }
    output_init(out);
}
#[no_mangle]
pub unsafe extern "C" fn output_start() {
    if !output_context.is_null() && (*output_context).syncout() as i32 != 0 {
        if !((*output_context).out >= 0 as i32 || (*output_context).err >= 0 as i32) {
            setup_tmpfile(output_context);
        }
    }
    if output_sync == 0 as i32 || output_sync == 3 as i32 {
        if stdio_traced == 0 && should_print_dir() != 0 {
            stdio_traced = log_working_directory(1 as i32) as u32;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn outputs(mut is_err: i32, mut msg: *const i8) {
    if msg.is_null() || *msg as i32 == '\0' as i32 {
        return;
    }
    output_start();
    _outputs(output_context, is_err, msg);
}
static mut fmtbuf: fmtstring = {
    let mut init = fmtstring {
        buffer: 0 as *const i8 as *mut i8,
        size: 0 as i32 as size_t,
    };
    init
};
unsafe extern "C" fn get_buffer(mut need: size_t) -> *mut i8 {
    if need > fmtbuf.size {
        fmtbuf.size = (fmtbuf.size as u64)
            .wrapping_add(need.wrapping_mul(2 as i32 as u64)) as size_t as size_t;
        fmtbuf.buffer = xrealloc(fmtbuf.buffer as *mut libc::c_void, fmtbuf.size)
            as *mut i8;
    }
    *(fmtbuf.buffer).offset(need.wrapping_sub(1 as i32 as u64) as isize) = '\0' as i32
        as i8;
    return fmtbuf.buffer;
}
#[no_mangle]
pub unsafe extern "C" fn message(
    mut prefix: i32,
    mut len: size_t,
    mut fmt: *const i8,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut start: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    len = (len as u64)
        .wrapping_add(
            (strlen(fmt))
                .wrapping_add(strlen(program))
                .wrapping_add(
                    (53 as i32 as u64)
                        .wrapping_mul(::core::mem::size_of::<uintmax_t>() as u64)
                        .wrapping_div(22 as i32 as u64)
                        .wrapping_add(3 as i32 as u64),
                )
                .wrapping_add(4 as i32 as u64)
                .wrapping_add(1 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
        ) as size_t as size_t;
    p = get_buffer(len);
    start = p;
    if prefix != 0 {
        if makelevel == 0 as i32 as u32 {
            sprintf(p, b"%s: \0" as *const u8 as *const i8, program);
        } else {
            sprintf(p, b"%s[%u]: \0" as *const u8 as *const i8, program, makelevel);
        }
        p = p.offset(strlen(p) as isize);
    }
    args_0 = args.clone();
    vsprintf(p, fmt, args_0.as_va_list());
    strcat(p, b"\n\0" as *const u8 as *const i8);
    outputs(0 as i32, start);
}
#[no_mangle]
pub unsafe extern "C" fn error(
    mut flocp: *const floc,
    mut len: size_t,
    mut fmt: *const i8,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut start: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    len = (len as u64)
        .wrapping_add(
            (strlen(fmt))
                .wrapping_add(strlen(program))
                .wrapping_add(
                    (if !flocp.is_null() && !((*flocp).filenm).is_null() {
                        strlen((*flocp).filenm)
                    } else {
                        0 as i32 as u64
                    }),
                )
                .wrapping_add(
                    (53 as i32 as u64)
                        .wrapping_mul(::core::mem::size_of::<uintmax_t>() as u64)
                        .wrapping_div(22 as i32 as u64)
                        .wrapping_add(3 as i32 as u64),
                )
                .wrapping_add(4 as i32 as u64)
                .wrapping_add(1 as i32 as u64)
                .wrapping_add(1 as i32 as u64),
        ) as size_t as size_t;
    p = get_buffer(len);
    start = p;
    if !flocp.is_null() && !((*flocp).filenm).is_null() {
        sprintf(
            p,
            b"%s:%lu: \0" as *const u8 as *const i8,
            (*flocp).filenm,
            ((*flocp).lineno).wrapping_add((*flocp).offset),
        );
    } else if makelevel == 0 as i32 as u32 {
        sprintf(p, b"%s: \0" as *const u8 as *const i8, program);
    } else {
        sprintf(p, b"%s[%u]: \0" as *const u8 as *const i8, program, makelevel);
    }
    p = p.offset(strlen(p) as isize);
    args_0 = args.clone();
    vsprintf(p, fmt, args_0.as_va_list());
    strcat(p, b"\n\0" as *const u8 as *const i8);
    outputs(1 as i32, start);
}
#[no_mangle]
pub unsafe extern "C" fn fatal(
    mut flocp: *const floc,
    mut len: size_t,
    mut fmt: *const i8,
    mut args: ...
) -> ! {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut stop: *const i8 = dcgettext(
        0 as *const i8,
        b".  Stop.\n\0" as *const u8 as *const i8,
        5 as i32,
    );
    let mut start: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    len = (len as u64)
        .wrapping_add(
            (strlen(fmt))
                .wrapping_add(strlen(program))
                .wrapping_add(
                    (if !flocp.is_null() && !((*flocp).filenm).is_null() {
                        strlen((*flocp).filenm)
                    } else {
                        0 as i32 as u64
                    }),
                )
                .wrapping_add(
                    (53 as i32 as u64)
                        .wrapping_mul(::core::mem::size_of::<uintmax_t>() as u64)
                        .wrapping_div(22 as i32 as u64)
                        .wrapping_add(3 as i32 as u64),
                )
                .wrapping_add(8 as i32 as u64)
                .wrapping_add(strlen(stop))
                .wrapping_add(1 as i32 as u64),
        ) as size_t as size_t;
    p = get_buffer(len);
    start = p;
    if !flocp.is_null() && !((*flocp).filenm).is_null() {
        sprintf(
            p,
            b"%s:%lu: *** \0" as *const u8 as *const i8,
            (*flocp).filenm,
            ((*flocp).lineno).wrapping_add((*flocp).offset),
        );
    } else if makelevel == 0 as i32 as u32 {
        sprintf(p, b"%s: *** \0" as *const u8 as *const i8, program);
    } else {
        sprintf(p, b"%s[%u]: *** \0" as *const u8 as *const i8, program, makelevel);
    }
    p = p.offset(strlen(p) as isize);
    args_0 = args.clone();
    vsprintf(p, fmt, args_0.as_va_list());
    strcat(p, stop);
    outputs(1 as i32, start);
    die(2 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn perror_with_name(mut str: *const i8, mut name: *const i8) {
    let mut err: *const i8 = strerror(*__errno_location());
    error(
        0 as *mut floc,
        (strlen(str)).wrapping_add(strlen(name)).wrapping_add(strlen(err)),
        dcgettext(0 as *const i8, b"%s%s: %s\0" as *const u8 as *const i8, 5 as i32),
        str,
        name,
        err,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pfatal_with_name(mut name: *const i8) -> ! {
    let mut err: *const i8 = strerror(*__errno_location());
    fatal(
        0 as *mut floc,
        (strlen(name)).wrapping_add(strlen(err)),
        dcgettext(0 as *const i8, b"%s: %s\0" as *const u8 as *const i8, 5 as i32),
        name,
        err,
    );
}
#[no_mangle]
pub unsafe extern "C" fn out_of_memory() -> ! {
    writebuf(fileno(stdout), program as *const libc::c_void, strlen(program));
    writebuf(
        fileno(stdout),
        b": *** virtual memory exhausted\n\0" as *const u8 as *const i8
            as *const libc::c_void,
        (::core::mem::size_of::<[i8; 32]>() as u64).wrapping_sub(1 as i32 as u64),
    );
    exit(2 as i32);
}
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
    fn __errno_location() -> *mut i32;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn vsprintf(_: *mut i8, _: *const i8, _: ::core::ffi::VaList) -> i32;
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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn _glp_xstrerr(_: i32) -> *mut i8;
    fn _glp_get_env_ptr() -> *mut ENV;
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn _glp_put_err_msg(msg: *const i8);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_zlib_gzread(file: gzFile, buf: voidp, len: u32) -> i32;
    fn _glp_zlib_gzwrite(file: gzFile, buf: voidpc, len: u32) -> i32;
    fn _glp_zlib_gzclose(file: gzFile) -> i32;
    fn _glp_zlib_gzerror(file: gzFile, errnum: *mut i32) -> *const i8;
    fn _glp_zlib_gzopen(_: *const i8, _: *const i8) -> gzFile;
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
pub type __off_t = i64;
pub type __off64_t = i64;
pub type va_list = __builtin_va_list;
pub type size_t = u64;
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
pub struct ENV {
    pub self_0: *mut ENV,
    pub term_buf: *mut i8,
    pub term_out: i32,
    pub term_hook: Option<unsafe extern "C" fn(*mut libc::c_void, *const i8) -> i32>,
    pub term_info: *mut libc::c_void,
    pub tee_file: *mut FILE,
    pub err_st: i32,
    pub err_file: *const i8,
    pub err_line: i32,
    pub err_hook: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub err_info: *mut libc::c_void,
    pub err_buf: *mut i8,
    pub mem_limit: size_t,
    pub mem_ptr: *mut MBD,
    pub mem_count: i32,
    pub mem_cpeak: i32,
    pub mem_total: size_t,
    pub mem_tpeak: size_t,
    pub gmp_pool: *mut libc::c_void,
    pub gmp_size: i32,
    pub gmp_work: *mut libc::c_ushort,
    pub h_odbc: *mut libc::c_void,
    pub h_mysql: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MBD {
    pub size: size_t,
    pub self_0: *mut MBD,
    pub prev: *mut MBD,
    pub next: *mut MBD,
}
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_file {
    pub base: *mut i8,
    pub size: i32,
    pub ptr: *mut i8,
    pub cnt: i32,
    pub flag: i32,
    pub file: *mut libc::c_void,
}
pub type gzFile = voidp;
pub type voidp = *mut libc::c_void;
pub type voidpc = *const libc::c_void;
#[no_mangle]
pub unsafe extern "C" fn _glp_open(
    mut name: *const i8,
    mut mode: *const i8,
) -> *mut glp_file {
    let mut f: *mut glp_file = 0 as *mut glp_file;
    let mut flag: i32 = 0;
    let mut file: *mut libc::c_void = 0 as *mut libc::c_void;
    if strcmp(mode, b"r\0" as *const u8 as *const i8) == 0 as i32
        || strcmp(mode, b"rb\0" as *const u8 as *const i8) == 0 as i32
    {
        flag = 0 as i32;
    } else if strcmp(mode, b"w\0" as *const u8 as *const i8) == 0 as i32
        || strcmp(mode, b"wb\0" as *const u8 as *const i8) == 0 as i32
    {
        flag = 0x8 as i32;
    } else if strcmp(mode, b"a\0" as *const u8 as *const i8) == 0 as i32
        || strcmp(mode, b"ab\0" as *const u8 as *const i8) == 0 as i32
    {
        flag = 0x8 as i32;
    } else {
        (glp_error_(b"env/stream.c\0" as *const u8 as *const i8, 101 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_open: invalid mode string\n\0" as *const u8 as *const i8);
    }
    if strcmp(name, b"/dev/null\0" as *const u8 as *const i8) == 0 as i32 {
        flag |= 0x1 as i32;
        file = 0 as *mut libc::c_void;
    } else if strcmp(name, b"/dev/stdin\0" as *const u8 as *const i8) == 0 as i32 {
        flag |= 0x2 as i32;
        file = stdin as *mut libc::c_void;
    } else if strcmp(name, b"/dev/stdout\0" as *const u8 as *const i8) == 0 as i32 {
        flag |= 0x2 as i32;
        file = stdout as *mut libc::c_void;
    } else if strcmp(name, b"/dev/stderr\0" as *const u8 as *const i8) == 0 as i32 {
        flag |= 0x2 as i32;
        file = stderr as *mut libc::c_void;
    } else {
        let mut ext: *mut i8 = strrchr(name, '.' as i32);
        if ext.is_null() || strcmp(ext, b".gz\0" as *const u8 as *const i8) != 0 as i32 {
            file = fopen(name, mode) as *mut libc::c_void;
            if file.is_null() {
                _glp_put_err_msg(_glp_xstrerr(*__errno_location()));
                return 0 as *mut glp_file;
            }
        } else {
            flag |= 0x4 as i32;
            if strcmp(mode, b"r\0" as *const u8 as *const i8) == 0 as i32 {
                mode = b"rb\0" as *const u8 as *const i8;
            } else if strcmp(mode, b"w\0" as *const u8 as *const i8) == 0 as i32 {
                mode = b"wb\0" as *const u8 as *const i8;
            } else if strcmp(mode, b"a\0" as *const u8 as *const i8) == 0 as i32 {
                mode = b"ab\0" as *const u8 as *const i8;
            }
            file = _glp_zlib_gzopen(name, mode);
            if file.is_null() {
                _glp_put_err_msg(_glp_xstrerr(*__errno_location()));
                return 0 as *mut glp_file;
            }
        }
    }
    f = glp_alloc(1 as i32, ::core::mem::size_of::<glp_file>() as u64 as i32)
        as *mut glp_file;
    (*f).base = glp_alloc(8192 as i32, ::core::mem::size_of::<i8>() as u64 as i32)
        as *mut i8;
    (*f).size = 8192 as i32;
    (*f).ptr = (*f).base;
    (*f).cnt = 0 as i32;
    (*f).flag = flag;
    (*f).file = file;
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_eof(mut f: *mut glp_file) -> i32 {
    return (*f).flag & 0x10 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ioerr(mut f: *mut glp_file) -> i32 {
    return (*f).flag & 0x20 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_read(
    mut f: *mut glp_file,
    mut buf: *mut libc::c_void,
    mut nnn: i32,
) -> i32 {
    let mut nrd: i32 = 0;
    let mut cnt: i32 = 0;
    if (*f).flag & 0x8 as i32 != 0 {
        (glp_error_(b"env/stream.c\0" as *const u8 as *const i8, 235 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_read: attempt to read from output stream\n\0" as *const u8 as *const i8,
        );
    }
    if nnn < 1 as i32 {
        (glp_error_(b"env/stream.c\0" as *const u8 as *const i8, 237 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_read: nnn = %d; invalid parameter\n\0" as *const u8 as *const i8,
            nnn,
        );
    }
    nrd = 0 as i32;
    while nrd < nnn {
        if (*f).cnt == 0 as i32 {
            if (*f).flag & 0x1 as i32 != 0 {
                cnt = 0 as i32;
            } else if (*f).flag & 0x4 as i32 == 0 {
                cnt = fread(
                    (*f).base as *mut libc::c_void,
                    1 as i32 as size_t,
                    (*f).size as size_t,
                    (*f).file as *mut FILE,
                ) as i32;
                if ferror((*f).file as *mut FILE) != 0 {
                    (*f).flag |= 0x20 as i32;
                    _glp_put_err_msg(_glp_xstrerr(*__errno_location()));
                    return -(1 as i32);
                }
            } else {
                let mut errnum: i32 = 0;
                let mut msg: *const i8 = 0 as *const i8;
                cnt = _glp_zlib_gzread((*f).file, (*f).base as voidp, (*f).size as u32);
                if cnt < 0 as i32 {
                    (*f).flag |= 0x20 as i32;
                    msg = _glp_zlib_gzerror((*f).file, &mut errnum);
                    if errnum == -(1 as i32) {
                        _glp_put_err_msg(_glp_xstrerr(*__errno_location()));
                    } else {
                        _glp_put_err_msg(msg);
                    }
                    return -(1 as i32);
                }
            }
            if cnt == 0 as i32 {
                if nrd == 0 as i32 {
                    (*f).flag |= 0x10 as i32;
                }
                break;
            } else {
                (*f).ptr = (*f).base;
                (*f).cnt = cnt;
            }
        }
        cnt = nnn - nrd;
        if cnt > (*f).cnt {
            cnt = (*f).cnt;
        }
        memcpy(
            (buf as *mut i8).offset(nrd as isize) as *mut libc::c_void,
            (*f).ptr as *const libc::c_void,
            cnt as u64,
        );
        (*f).ptr = ((*f).ptr).offset(cnt as isize);
        (*f).cnt -= cnt;
        nrd += cnt;
    }
    return nrd;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_getc(mut f: *mut glp_file) -> i32 {
    let mut buf: [u8; 1] = [0; 1];
    if (*f).flag & 0x8 as i32 != 0 {
        (glp_error_(b"env/stream.c\0" as *const u8 as *const i8, 315 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_getc: attempt to read from output stream\n\0" as *const u8 as *const i8,
        );
    }
    if _glp_read(f, buf.as_mut_ptr() as *mut libc::c_void, 1 as i32) != 1 as i32 {
        return -(1 as i32);
    }
    return buf[0 as i32 as usize] as i32;
}
unsafe extern "C" fn do_flush(mut f: *mut glp_file) -> i32 {
    ((*f).flag & 0x8 as i32 != 0
        || {
            glp_assert_(
                b"f->flag & IOWRT\0" as *const u8 as *const i8,
                b"env/stream.c\0" as *const u8 as *const i8,
                331 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*f).cnt > 0 as i32 {
        if !((*f).flag & 0x1 as i32 != 0) {
            if (*f).flag & 0x4 as i32 == 0 {
                if fwrite(
                    (*f).base as *const libc::c_void,
                    1 as i32 as size_t,
                    (*f).cnt as size_t,
                    (*f).file as *mut FILE,
                ) as i32 != (*f).cnt
                {
                    (*f).flag |= 0x20 as i32;
                    _glp_put_err_msg(_glp_xstrerr(*__errno_location()));
                    return -(1 as i32);
                }
            } else {
                let mut errnum: i32 = 0;
                let mut msg: *const i8 = 0 as *const i8;
                if _glp_zlib_gzwrite((*f).file, (*f).base as voidpc, (*f).cnt as u32)
                    != (*f).cnt
                {
                    (*f).flag |= 0x20 as i32;
                    msg = _glp_zlib_gzerror((*f).file, &mut errnum);
                    if errnum == -(1 as i32) {
                        _glp_put_err_msg(_glp_xstrerr(*__errno_location()));
                    } else {
                        _glp_put_err_msg(msg);
                    }
                    return -(1 as i32);
                }
            }
        }
    }
    (*f).ptr = (*f).base;
    (*f).cnt = 0 as i32;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_write(
    mut f: *mut glp_file,
    mut buf: *const libc::c_void,
    mut nnn: i32,
) -> i32 {
    let mut nwr: i32 = 0;
    let mut cnt: i32 = 0;
    if (*f).flag & 0x8 as i32 == 0 {
        (glp_error_(b"env/stream.c\0" as *const u8 as *const i8, 394 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_write: attempt to write to input stream\n\0" as *const u8 as *const i8,
        );
    }
    if nnn < 1 as i32 {
        (glp_error_(b"env/stream.c\0" as *const u8 as *const i8, 396 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_write: nnn = %d; invalid parameter\n\0" as *const u8 as *const i8,
            nnn,
        );
    }
    nwr = 0 as i32;
    while nwr < nnn {
        cnt = nnn - nwr;
        if cnt > (*f).size - (*f).cnt {
            cnt = (*f).size - (*f).cnt;
        }
        memcpy(
            (*f).ptr as *mut libc::c_void,
            (buf as *const i8).offset(nwr as isize) as *const libc::c_void,
            cnt as u64,
        );
        (*f).ptr = ((*f).ptr).offset(cnt as isize);
        (*f).cnt += cnt;
        if (*f).cnt == (*f).size {
            if do_flush(f) != 0 as i32 {
                return -(1 as i32);
            }
        }
        nwr += cnt;
    }
    return nwr;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_format(
    mut f: *mut glp_file,
    mut fmt: *const i8,
    mut args: ...
) -> i32 {
    let mut env: *mut ENV = _glp_get_env_ptr();
    let mut arg: ::core::ffi::VaListImpl;
    let mut nnn: i32 = 0;
    if (*f).flag & 0x8 as i32 == 0 {
        (glp_error_(b"env/stream.c\0" as *const u8 as *const i8, 438 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_format: attempt to write to input stream\n\0" as *const u8 as *const i8,
        );
    }
    arg = args.clone();
    nnn = vsprintf((*env).term_buf, fmt, arg.as_va_list());
    (0 as i32 <= nnn && nnn < 4096 as i32
        || {
            glp_assert_(
                b"0 <= nnn && nnn < TBUF_SIZE\0" as *const u8 as *const i8,
                b"env/stream.c\0" as *const u8 as *const i8,
                441 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return if nnn == 0 as i32 {
        0 as i32
    } else {
        _glp_write(f, (*env).term_buf as *const libc::c_void, nnn)
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_close(mut f: *mut glp_file) -> i32 {
    let mut ret: i32 = 0 as i32;
    if (*f).flag & 0x8 as i32 != 0 {
        if do_flush(f) != 0 as i32 {
            ret = -(1 as i32);
        }
    }
    if !((*f).flag & (0x1 as i32 | 0x2 as i32) != 0) {
        if (*f).flag & 0x4 as i32 == 0 {
            if fclose((*f).file as *mut FILE) != 0 as i32 {
                if ret == 0 as i32 {
                    _glp_put_err_msg(_glp_xstrerr(*__errno_location()));
                    ret = -(1 as i32);
                }
            }
        } else {
            let mut errnum: i32 = 0;
            errnum = _glp_zlib_gzclose((*f).file);
            if !(errnum == 0 as i32) {
                if errnum == -(1 as i32) {
                    if ret == 0 as i32 {
                        _glp_put_err_msg(_glp_xstrerr(*__errno_location()));
                        ret = -(1 as i32);
                    }
                } else if ret == 0 as i32 {
                    let mut env: *mut ENV = _glp_get_env_ptr();
                    sprintf(
                        (*env).term_buf,
                        b"gzclose returned %d\0" as *const u8 as *const i8,
                        errnum,
                    );
                    _glp_put_err_msg((*env).term_buf);
                    ret = -(1 as i32);
                }
            }
        }
    }
    glp_free((*f).base as *mut libc::c_void);
    glp_free(f as *mut libc::c_void);
    return ret;
}
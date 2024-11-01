#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic, label_break_value)]
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
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
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
static mut file: [*mut FILE; 16] = [0 as *const FILE as *mut FILE; 16];
static mut initialized: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn initialize() {
    let mut fd: libc::c_int = 0;
    if initialized == 0 {} else {
        __assert_fail(
            b"!initialized\0" as *const u8 as *const libc::c_char,
            b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
            18 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"void initialize(void)\0"))
                .as_ptr(),
        );
    }
    'c_1214: {
        if initialized == 0 {} else {
            __assert_fail(
                b"!initialized\0" as *const u8 as *const libc::c_char,
                b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
                18 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"void initialize(void)\0"))
                    .as_ptr(),
            );
        }
    };
    file[0 as libc::c_int as usize] = stdin;
    file[1 as libc::c_int as usize] = stdout;
    file[2 as libc::c_int as usize] = stderr;
    fd = 3 as libc::c_int;
    while fd < 16 as libc::c_int {
        file[fd as usize] = 0 as *mut FILE;
        fd += 1;
        fd;
    }
    initialized = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_open(
    mut path: *const libc::c_char,
    mut oflag: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut fd: libc::c_int = 0;
    if initialized == 0 {
        initialize();
    }
    if oflag == 0 as libc::c_int {
        fp = fopen(path, b"rb\0" as *const u8 as *const libc::c_char);
    } else if oflag == 0x1 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int {
        fp = fopen(path, b"wb\0" as *const u8 as *const libc::c_char);
    } else if oflag == 0x1 as libc::c_int | 0x10 as libc::c_int | 0x30 as libc::c_int {
        fp = fopen(path, b"ab\0" as *const u8 as *const libc::c_char);
    } else {
        if oflag != oflag {} else {
            __assert_fail(
                b"oflag != oflag\0" as *const u8 as *const libc::c_char,
                b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
                40 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"int _glp_zlib_open(const char *, int, ...)\0"))
                    .as_ptr(),
            );
        }
        'c_1057: {
            if oflag != oflag {} else {
                __assert_fail(
                    b"oflag != oflag\0" as *const u8 as *const libc::c_char,
                    b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
                    40 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 43],
                        &[libc::c_char; 43],
                    >(b"int _glp_zlib_open(const char *, int, ...)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    if fp.is_null() {
        return -(1 as libc::c_int);
    }
    fd = 0 as libc::c_int;
    while fd < 16 as libc::c_int {
        if (file[fd as usize]).is_null() {
            break;
        }
        fd += 1;
        fd;
    }
    if fd < 16 as libc::c_int {} else {
        __assert_fail(
            b"fd < FOPEN_MAX\0" as *const u8 as *const libc::c_char,
            b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"int _glp_zlib_open(const char *, int, ...)\0"))
                .as_ptr(),
        );
    }
    'c_972: {
        if fd < 16 as libc::c_int {} else {
            __assert_fail(
                b"fd < FOPEN_MAX\0" as *const u8 as *const libc::c_char,
                b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
                45 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"int _glp_zlib_open(const char *, int, ...)\0"))
                    .as_ptr(),
            );
        }
    };
    file[fd as usize] = fp;
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_read(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_void,
    mut nbyte: libc::c_ulong,
) -> libc::c_long {
    let mut count: libc::c_ulong = 0;
    if initialized == 0 {
        initialize();
    }
    if 0 as libc::c_int <= fd && fd < 16 as libc::c_int {} else {
        __assert_fail(
            b"0 <= fd && fd < FOPEN_MAX\0" as *const u8 as *const libc::c_char,
            b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"long _glp_zlib_read(int, void *, unsigned long)\0"))
                .as_ptr(),
        );
    }
    'c_1353: {
        if 0 as libc::c_int <= fd && fd < 16 as libc::c_int {} else {
            __assert_fail(
                b"0 <= fd && fd < FOPEN_MAX\0" as *const u8 as *const libc::c_char,
                b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
                53 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"long _glp_zlib_read(int, void *, unsigned long)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(file[fd as usize]).is_null() {} else {
        __assert_fail(
            b"file[fd] != NULL\0" as *const u8 as *const libc::c_char,
            b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"long _glp_zlib_read(int, void *, unsigned long)\0"))
                .as_ptr(),
        );
    }
    'c_1301: {
        if !(file[fd as usize]).is_null() {} else {
            __assert_fail(
                b"file[fd] != NULL\0" as *const u8 as *const libc::c_char,
                b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
                54 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"long _glp_zlib_read(int, void *, unsigned long)\0"))
                    .as_ptr(),
            );
        }
    };
    count = fread(buf, 1 as libc::c_int as size_t, nbyte, file[fd as usize]);
    if ferror(file[fd as usize]) != 0 {
        return -(1 as libc::c_int) as libc::c_long;
    }
    return count as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_write(
    mut fd: libc::c_int,
    mut buf: *const libc::c_void,
    mut nbyte: libc::c_ulong,
) -> libc::c_long {
    let mut count: libc::c_ulong = 0;
    if initialized == 0 {
        initialize();
    }
    if 0 as libc::c_int <= fd && fd < 16 as libc::c_int {} else {
        __assert_fail(
            b"0 <= fd && fd < FOPEN_MAX\0" as *const u8 as *const libc::c_char,
            b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"long _glp_zlib_write(int, const void *, unsigned long)\0"))
                .as_ptr(),
        );
    }
    'c_1518: {
        if 0 as libc::c_int <= fd && fd < 16 as libc::c_int {} else {
            __assert_fail(
                b"0 <= fd && fd < FOPEN_MAX\0" as *const u8 as *const libc::c_char,
                b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"long _glp_zlib_write(int, const void *, unsigned long)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(file[fd as usize]).is_null() {} else {
        __assert_fail(
            b"file[fd] != NULL\0" as *const u8 as *const libc::c_char,
            b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"long _glp_zlib_write(int, const void *, unsigned long)\0"))
                .as_ptr(),
        );
    }
    'c_1467: {
        if !(file[fd as usize]).is_null() {} else {
            __assert_fail(
                b"file[fd] != NULL\0" as *const u8 as *const libc::c_char,
                b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
                65 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"long _glp_zlib_write(int, const void *, unsigned long)\0"))
                    .as_ptr(),
            );
        }
    };
    count = fwrite(buf, 1 as libc::c_int as size_t, nbyte, file[fd as usize]);
    if count != nbyte {
        return -(1 as libc::c_int) as libc::c_long;
    }
    if fflush(file[fd as usize]) != 0 as libc::c_int {
        return -(1 as libc::c_int) as libc::c_long;
    }
    return count as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_lseek(
    mut fd: libc::c_int,
    mut offset: libc::c_long,
    mut whence: libc::c_int,
) -> libc::c_long {
    if initialized == 0 {
        initialize();
    }
    if 0 as libc::c_int <= fd && fd < 16 as libc::c_int {} else {
        __assert_fail(
            b"0 <= fd && fd < FOPEN_MAX\0" as *const u8 as *const libc::c_char,
            b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"long _glp_zlib_lseek(int, long, int)\0"))
                .as_ptr(),
        );
    }
    'c_1663: {
        if 0 as libc::c_int <= fd && fd < 16 as libc::c_int {} else {
            __assert_fail(
                b"0 <= fd && fd < FOPEN_MAX\0" as *const u8 as *const libc::c_char,
                b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
                76 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"long _glp_zlib_lseek(int, long, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(file[fd as usize]).is_null() {} else {
        __assert_fail(
            b"file[fd] != NULL\0" as *const u8 as *const libc::c_char,
            b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"long _glp_zlib_lseek(int, long, int)\0"))
                .as_ptr(),
        );
    }
    'c_1612: {
        if !(file[fd as usize]).is_null() {} else {
            __assert_fail(
                b"file[fd] != NULL\0" as *const u8 as *const libc::c_char,
                b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
                77 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"long _glp_zlib_lseek(int, long, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if fseek(file[fd as usize], offset, whence) != 0 as libc::c_int {
        return -(1 as libc::c_int) as libc::c_long;
    }
    return ftell(file[fd as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_close(mut fd: libc::c_int) -> libc::c_int {
    if initialized == 0 {
        initialize();
    }
    if 0 as libc::c_int <= fd && fd < 16 as libc::c_int {} else {
        __assert_fail(
            b"0 <= fd && fd < FOPEN_MAX\0" as *const u8 as *const libc::c_char,
            b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"int _glp_zlib_close(int)\0"))
                .as_ptr(),
        );
    }
    'c_1796: {
        if 0 as libc::c_int <= fd && fd < 16 as libc::c_int {} else {
            __assert_fail(
                b"0 <= fd && fd < FOPEN_MAX\0" as *const u8 as *const libc::c_char,
                b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"int _glp_zlib_close(int)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(file[fd as usize]).is_null() {} else {
        __assert_fail(
            b"file[fd] != NULL\0" as *const u8 as *const libc::c_char,
            b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"int _glp_zlib_close(int)\0"))
                .as_ptr(),
        );
    }
    'c_1745: {
        if !(file[fd as usize]).is_null() {} else {
            __assert_fail(
                b"file[fd] != NULL\0" as *const u8 as *const libc::c_char,
                b"zlib/zio.c\0" as *const u8 as *const libc::c_char,
                86 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"int _glp_zlib_close(int)\0"))
                    .as_ptr(),
            );
        }
    };
    fclose(file[fd as usize]);
    file[fd as usize] = 0 as *mut FILE;
    return 0 as libc::c_int;
}

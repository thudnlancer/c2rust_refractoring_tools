#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, label_break_value)]
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
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
    fn fseek(__stream: *mut FILE, __off: i64, __whence: i32) -> i32;
    fn ftell(__stream: *mut FILE) -> i64;
    fn ferror(__stream: *mut FILE) -> i32;
}
pub type size_t = u64;
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
static mut file: [*mut FILE; 16] = [0 as *const FILE as *mut FILE; 16];
static mut initialized: i32 = 0 as i32;
unsafe extern "C" fn initialize() {
    let mut fd: i32 = 0;
    if initialized == 0 {} else {
        __assert_fail(
            b"!initialized\0" as *const u8 as *const i8,
            b"zlib/zio.c\0" as *const u8 as *const i8,
            18 as i32 as u32,
            (*::core::mem::transmute::<&[u8; 22], &[i8; 22]>(b"void initialize(void)\0"))
                .as_ptr(),
        );
    }
    'c_1214: {
        if initialized == 0 {} else {
            __assert_fail(
                b"!initialized\0" as *const u8 as *const i8,
                b"zlib/zio.c\0" as *const u8 as *const i8,
                18 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 22],
                    &[i8; 22],
                >(b"void initialize(void)\0"))
                    .as_ptr(),
            );
        }
    };
    file[0 as i32 as usize] = stdin;
    file[1 as i32 as usize] = stdout;
    file[2 as i32 as usize] = stderr;
    fd = 3 as i32;
    while fd < 16 as i32 {
        file[fd as usize] = 0 as *mut FILE;
        fd += 1;
        fd;
    }
    initialized = 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_open(
    mut path: *const i8,
    mut oflag: i32,
    mut args: ...
) -> i32 {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut fd: i32 = 0;
    if initialized == 0 {
        initialize();
    }
    if oflag == 0 as i32 {
        fp = fopen(path, b"rb\0" as *const u8 as *const i8);
    } else if oflag == 0x1 as i32 | 0x10 as i32 | 0x20 as i32 {
        fp = fopen(path, b"wb\0" as *const u8 as *const i8);
    } else if oflag == 0x1 as i32 | 0x10 as i32 | 0x30 as i32 {
        fp = fopen(path, b"ab\0" as *const u8 as *const i8);
    } else {
        if oflag != oflag {} else {
            __assert_fail(
                b"oflag != oflag\0" as *const u8 as *const i8,
                b"zlib/zio.c\0" as *const u8 as *const i8,
                40 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 43],
                    &[i8; 43],
                >(b"int _glp_zlib_open(const char *, int, ...)\0"))
                    .as_ptr(),
            );
        }
        'c_1057: {
            if oflag != oflag {} else {
                __assert_fail(
                    b"oflag != oflag\0" as *const u8 as *const i8,
                    b"zlib/zio.c\0" as *const u8 as *const i8,
                    40 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 43],
                        &[i8; 43],
                    >(b"int _glp_zlib_open(const char *, int, ...)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    if fp.is_null() {
        return -(1 as i32);
    }
    fd = 0 as i32;
    while fd < 16 as i32 {
        if (file[fd as usize]).is_null() {
            break;
        }
        fd += 1;
        fd;
    }
    if fd < 16 as i32 {} else {
        __assert_fail(
            b"fd < FOPEN_MAX\0" as *const u8 as *const i8,
            b"zlib/zio.c\0" as *const u8 as *const i8,
            45 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[i8; 43],
            >(b"int _glp_zlib_open(const char *, int, ...)\0"))
                .as_ptr(),
        );
    }
    'c_972: {
        if fd < 16 as i32 {} else {
            __assert_fail(
                b"fd < FOPEN_MAX\0" as *const u8 as *const i8,
                b"zlib/zio.c\0" as *const u8 as *const i8,
                45 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 43],
                    &[i8; 43],
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
    mut fd: i32,
    mut buf: *mut libc::c_void,
    mut nbyte: u64,
) -> i64 {
    let mut count: u64 = 0;
    if initialized == 0 {
        initialize();
    }
    if 0 as i32 <= fd && fd < 16 as i32 {} else {
        __assert_fail(
            b"0 <= fd && fd < FOPEN_MAX\0" as *const u8 as *const i8,
            b"zlib/zio.c\0" as *const u8 as *const i8,
            53 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[i8; 48],
            >(b"long _glp_zlib_read(int, void *, unsigned long)\0"))
                .as_ptr(),
        );
    }
    'c_1353: {
        if 0 as i32 <= fd && fd < 16 as i32 {} else {
            __assert_fail(
                b"0 <= fd && fd < FOPEN_MAX\0" as *const u8 as *const i8,
                b"zlib/zio.c\0" as *const u8 as *const i8,
                53 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[i8; 48],
                >(b"long _glp_zlib_read(int, void *, unsigned long)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(file[fd as usize]).is_null() {} else {
        __assert_fail(
            b"file[fd] != NULL\0" as *const u8 as *const i8,
            b"zlib/zio.c\0" as *const u8 as *const i8,
            54 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[i8; 48],
            >(b"long _glp_zlib_read(int, void *, unsigned long)\0"))
                .as_ptr(),
        );
    }
    'c_1301: {
        if !(file[fd as usize]).is_null() {} else {
            __assert_fail(
                b"file[fd] != NULL\0" as *const u8 as *const i8,
                b"zlib/zio.c\0" as *const u8 as *const i8,
                54 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[i8; 48],
                >(b"long _glp_zlib_read(int, void *, unsigned long)\0"))
                    .as_ptr(),
            );
        }
    };
    count = fread(buf, 1 as i32 as size_t, nbyte, file[fd as usize]);
    if ferror(file[fd as usize]) != 0 {
        return -(1 as i32) as i64;
    }
    return count as i64;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_write(
    mut fd: i32,
    mut buf: *const libc::c_void,
    mut nbyte: u64,
) -> i64 {
    let mut count: u64 = 0;
    if initialized == 0 {
        initialize();
    }
    if 0 as i32 <= fd && fd < 16 as i32 {} else {
        __assert_fail(
            b"0 <= fd && fd < FOPEN_MAX\0" as *const u8 as *const i8,
            b"zlib/zio.c\0" as *const u8 as *const i8,
            64 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[i8; 55],
            >(b"long _glp_zlib_write(int, const void *, unsigned long)\0"))
                .as_ptr(),
        );
    }
    'c_1518: {
        if 0 as i32 <= fd && fd < 16 as i32 {} else {
            __assert_fail(
                b"0 <= fd && fd < FOPEN_MAX\0" as *const u8 as *const i8,
                b"zlib/zio.c\0" as *const u8 as *const i8,
                64 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[i8; 55],
                >(b"long _glp_zlib_write(int, const void *, unsigned long)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(file[fd as usize]).is_null() {} else {
        __assert_fail(
            b"file[fd] != NULL\0" as *const u8 as *const i8,
            b"zlib/zio.c\0" as *const u8 as *const i8,
            65 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[i8; 55],
            >(b"long _glp_zlib_write(int, const void *, unsigned long)\0"))
                .as_ptr(),
        );
    }
    'c_1467: {
        if !(file[fd as usize]).is_null() {} else {
            __assert_fail(
                b"file[fd] != NULL\0" as *const u8 as *const i8,
                b"zlib/zio.c\0" as *const u8 as *const i8,
                65 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[i8; 55],
                >(b"long _glp_zlib_write(int, const void *, unsigned long)\0"))
                    .as_ptr(),
            );
        }
    };
    count = fwrite(buf, 1 as i32 as size_t, nbyte, file[fd as usize]);
    if count != nbyte {
        return -(1 as i32) as i64;
    }
    if fflush(file[fd as usize]) != 0 as i32 {
        return -(1 as i32) as i64;
    }
    return count as i64;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_lseek(
    mut fd: i32,
    mut offset: i64,
    mut whence: i32,
) -> i64 {
    if initialized == 0 {
        initialize();
    }
    if 0 as i32 <= fd && fd < 16 as i32 {} else {
        __assert_fail(
            b"0 <= fd && fd < FOPEN_MAX\0" as *const u8 as *const i8,
            b"zlib/zio.c\0" as *const u8 as *const i8,
            76 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[i8; 37],
            >(b"long _glp_zlib_lseek(int, long, int)\0"))
                .as_ptr(),
        );
    }
    'c_1663: {
        if 0 as i32 <= fd && fd < 16 as i32 {} else {
            __assert_fail(
                b"0 <= fd && fd < FOPEN_MAX\0" as *const u8 as *const i8,
                b"zlib/zio.c\0" as *const u8 as *const i8,
                76 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 37],
                    &[i8; 37],
                >(b"long _glp_zlib_lseek(int, long, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(file[fd as usize]).is_null() {} else {
        __assert_fail(
            b"file[fd] != NULL\0" as *const u8 as *const i8,
            b"zlib/zio.c\0" as *const u8 as *const i8,
            77 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[i8; 37],
            >(b"long _glp_zlib_lseek(int, long, int)\0"))
                .as_ptr(),
        );
    }
    'c_1612: {
        if !(file[fd as usize]).is_null() {} else {
            __assert_fail(
                b"file[fd] != NULL\0" as *const u8 as *const i8,
                b"zlib/zio.c\0" as *const u8 as *const i8,
                77 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 37],
                    &[i8; 37],
                >(b"long _glp_zlib_lseek(int, long, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if fseek(file[fd as usize], offset, whence) != 0 as i32 {
        return -(1 as i32) as i64;
    }
    return ftell(file[fd as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_close(mut fd: i32) -> i32 {
    if initialized == 0 {
        initialize();
    }
    if 0 as i32 <= fd && fd < 16 as i32 {} else {
        __assert_fail(
            b"0 <= fd && fd < FOPEN_MAX\0" as *const u8 as *const i8,
            b"zlib/zio.c\0" as *const u8 as *const i8,
            85 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[i8; 25],
            >(b"int _glp_zlib_close(int)\0"))
                .as_ptr(),
        );
    }
    'c_1796: {
        if 0 as i32 <= fd && fd < 16 as i32 {} else {
            __assert_fail(
                b"0 <= fd && fd < FOPEN_MAX\0" as *const u8 as *const i8,
                b"zlib/zio.c\0" as *const u8 as *const i8,
                85 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[i8; 25],
                >(b"int _glp_zlib_close(int)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(file[fd as usize]).is_null() {} else {
        __assert_fail(
            b"file[fd] != NULL\0" as *const u8 as *const i8,
            b"zlib/zio.c\0" as *const u8 as *const i8,
            86 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[i8; 25],
            >(b"int _glp_zlib_close(int)\0"))
                .as_ptr(),
        );
    }
    'c_1745: {
        if !(file[fd as usize]).is_null() {} else {
            __assert_fail(
                b"file[fd] != NULL\0" as *const u8 as *const i8,
                b"zlib/zio.c\0" as *const u8 as *const i8,
                86 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[i8; 25],
                >(b"int _glp_zlib_close(int)\0"))
                    .as_ptr(),
            );
        }
    };
    fclose(file[fd as usize]);
    file[fd as usize] = 0 as *mut FILE;
    return 0 as i32;
}
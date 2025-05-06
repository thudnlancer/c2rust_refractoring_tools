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
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const i8) -> *mut i8;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strtok(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn __errno_location() -> *mut i32;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn argcv_get(
        command: *const i8,
        delim: *const i8,
        cmnt: *const i8,
        argc: *mut i32,
        argv: *mut *mut *mut i8,
    ) -> i32;
}
pub type size_t = u64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
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
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
unsafe extern "C" fn expand_argcv(
    mut argc_ptr: *mut i32,
    mut argv_ptr: *mut *mut *mut i8,
    mut argc: i32,
    mut argv: *mut *mut i8,
) {
    let mut i: i32 = 0;
    *argv_ptr = xrealloc(
        *argv_ptr as *mut libc::c_void,
        ((*argc_ptr + argc + 1 as i32) as u64)
            .wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
    ) as *mut *mut i8;
    i = 0 as i32;
    while i <= argc {
        let ref mut fresh0 = *(*argv_ptr).offset((*argc_ptr + i) as isize);
        *fresh0 = *argv.offset(i as isize);
        i += 1;
        i;
    }
    *argc_ptr += argc;
}
#[no_mangle]
pub unsafe extern "C" fn parse_rc(
    mut argc_ptr: *mut i32,
    mut argv_ptr: *mut *mut *mut i8,
    mut name: *mut i8,
) {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut rcfile: *mut FILE = 0 as *mut FILE;
    let mut size: i32 = 0;
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    if stat(name, &mut st) != 0 {
        return;
    }
    buf = malloc((st.st_size + 1 as i32 as i64) as u64) as *mut i8;
    if buf.is_null() {
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"not enough memory to process rc file\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return;
    }
    rcfile = fopen(name, b"r\0" as *const u8 as *const i8);
    if rcfile.is_null() {
        error(
            0 as i32,
            *__errno_location(),
            dcgettext(
                0 as *const i8,
                b"cannot open `%s'\0" as *const u8 as *const i8,
                5 as i32,
            ),
            name,
        );
        return;
    }
    size = fread(
        buf as *mut libc::c_void,
        1 as i32 as size_t,
        st.st_size as size_t,
        rcfile,
    ) as i32;
    *buf.offset(size as isize) = 0 as i32 as i8;
    fclose(rcfile);
    p = strtok(buf, b"\n\0" as *const u8 as *const i8);
    while !p.is_null() {
        let mut argc: i32 = 0;
        let mut argv: *mut *mut i8 = 0 as *mut *mut i8;
        argcv_get(
            p,
            b"\0" as *const u8 as *const i8,
            b"#\0" as *const u8 as *const i8,
            &mut argc,
            &mut argv,
        );
        expand_argcv(argc_ptr, argv_ptr, argc, argv);
        free(argv as *mut libc::c_void);
        p = strtok(0 as *mut i8, b"\n\0" as *const u8 as *const i8);
    }
    free(buf as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn sourcerc(
    mut argc_ptr: *mut i32,
    mut argv_ptr: *mut *mut *mut i8,
) {
    let mut env: *mut i8 = 0 as *mut i8;
    let mut xargc: i32 = 1 as i32;
    let mut xargv: *mut *mut i8 = 0 as *mut *mut i8;
    xargv = xmalloc(
        (2 as i32 as u64).wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
    ) as *mut *mut i8;
    let ref mut fresh1 = *xargv.offset(0 as i32 as isize);
    *fresh1 = **argv_ptr;
    let ref mut fresh2 = *xargv.offset(1 as i32 as isize);
    *fresh2 = 0 as *mut i8;
    env = getenv(b"CFLOW_OPTIONS\0" as *const u8 as *const i8);
    if !env.is_null() {
        let mut argc: i32 = 0;
        let mut argv: *mut *mut i8 = 0 as *mut *mut i8;
        argcv_get(
            env,
            b"\0" as *const u8 as *const i8,
            b"#\0" as *const u8 as *const i8,
            &mut argc,
            &mut argv,
        );
        expand_argcv(&mut xargc, &mut xargv, argc, argv);
        free(argv as *mut libc::c_void);
    }
    env = getenv(b"CFLOWRC\0" as *const u8 as *const i8);
    if !env.is_null() {
        parse_rc(&mut xargc, &mut xargv, env);
    } else {
        let mut home: *mut i8 = getenv(b"HOME\0" as *const u8 as *const i8);
        if !home.is_null() {
            let mut len: i32 = strlen(home) as i32;
            let mut buf: *mut i8 = malloc(
                (len as u64)
                    .wrapping_add(::core::mem::size_of::<[i8; 9]>() as u64)
                    .wrapping_add(
                        (*home.offset((len - 1 as i32) as isize) as i32 != '/' as i32)
                            as i32 as u64,
                    ),
            ) as *mut i8;
            if buf.is_null() {
                return;
            }
            strcpy(buf, home);
            if *home.offset((len - 1 as i32) as isize) as i32 != '/' as i32 {
                let fresh3 = len;
                len = len + 1;
                *buf.offset(fresh3 as isize) = '/' as i32 as i8;
            }
            strcpy(buf.offset(len as isize), b".cflowrc\0" as *const u8 as *const i8);
            parse_rc(&mut xargc, &mut xargv, buf);
            free(buf as *mut libc::c_void);
        }
    }
    if xargc > 1 as i32 {
        expand_argcv(
            &mut xargc,
            &mut xargv,
            *argc_ptr - 1 as i32,
            (*argv_ptr).offset(1 as i32 as isize),
        );
        *argc_ptr = xargc;
        *argv_ptr = xargv;
    }
}
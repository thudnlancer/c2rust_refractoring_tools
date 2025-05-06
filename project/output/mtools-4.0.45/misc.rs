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
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn exit(_: i32) -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    static mut stderr: *mut _IO_FILE;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn geteuid() -> __uid_t;
    fn unlink(__name: *const i8) -> i32;
    fn getlogin() -> *mut i8;
    fn time(__timer: *mut time_t) -> time_t;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn __errno_location() -> *mut i32;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getpwnam(__name: *const i8) -> *mut passwd;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strerror(_: i32) -> *mut i8;
}
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
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type time_t = __time_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type C2RustUnnamed = u32;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut i8,
    pub pw_passwd: *mut i8,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut i8,
    pub pw_dir: *mut i8,
    pub pw_shell: *mut i8,
}
pub type mt_off_t = off_t;
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[no_mangle]
pub unsafe extern "C" fn printOom() {
    fprintf(stderr, b"Out of memory error\0" as *const u8 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn get_homedir() -> *mut i8 {
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut uid: uid_t = 0;
    let mut homedir: *mut i8 = 0 as *mut i8;
    let mut username: *mut i8 = 0 as *mut i8;
    homedir = getenv(b"HOME\0" as *const u8 as *const i8);
    if !homedir.is_null() {
        return homedir;
    }
    pw = 0 as *mut passwd;
    username = getenv(b"LOGNAME\0" as *const u8 as *const i8);
    if username.is_null() {
        username = getlogin();
    }
    if !username.is_null() {
        pw = getpwnam(username);
    }
    if pw.is_null() {
        uid = geteuid();
        pw = getpwuid(uid);
    }
    if !pw.is_null() {
        return (*pw).pw_dir;
    }
    return 0 as *mut i8;
}
unsafe extern "C" fn get_mcwd_file_name(mut file: *mut i8) {
    let mut mcwd_path: *mut i8 = 0 as *mut i8;
    let mut homedir: *const i8 = 0 as *const i8;
    mcwd_path = getenv(b"MCWD\0" as *const u8 as *const i8);
    if mcwd_path.is_null() || *mcwd_path as i32 == '\0' as i32 {
        homedir = get_homedir();
        if homedir.is_null() {
            homedir = b"/tmp\0" as *const u8 as *const i8;
        }
        strncpy(file, homedir, (4096 as i32 - 6 as i32) as u64);
        *file.offset((4096 as i32 - 6 as i32) as isize) = '\0' as i32 as i8;
        strcat(file, b"/.mcwd\0" as *const u8 as *const i8);
    } else {
        strncpy(file, mcwd_path, 4096 as i32 as u64);
        *file.offset(4096 as i32 as isize) = '\0' as i32 as i8;
    };
}
#[no_mangle]
pub unsafe extern "C" fn unlink_mcwd() {
    let mut file: [i8; 4097] = [0; 4097];
    get_mcwd_file_name(file.as_mut_ptr());
    unlink(file.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn open_mcwd(mut mode: *const i8) -> *mut FILE {
    let mut sbuf: stat = stat {
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
    let mut file: [i8; 4097] = [0; 4097];
    let mut now: time_t = 0;
    get_mcwd_file_name(file.as_mut_ptr());
    if *mode as i32 == 'r' as i32 {
        if stat(file.as_mut_ptr(), &mut sbuf) < 0 as i32 {
            return 0 as *mut FILE;
        }
        getTimeNow(&mut now);
        if now - sbuf.st_mtim.tv_sec > (6 as i32 * 60 as i32 * 60 as i32) as i64 {
            fprintf(
                stderr,
                b"Warning: \"%s\" is out of date, removing it\n\0" as *const u8
                    as *const i8,
                file.as_mut_ptr(),
            );
            unlink(file.as_mut_ptr());
            return 0 as *mut FILE;
        }
    }
    return fopen(file.as_mut_ptr(), mode);
}
#[no_mangle]
pub unsafe extern "C" fn safe_malloc(mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = malloc(size);
    if p.is_null() {
        printOom();
        exit(1 as i32);
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn print_sector(
    mut message: *const i8,
    mut data: *mut u8,
    mut size: i32,
) {
    let mut col: i32 = 0;
    let mut row: i32 = 0;
    printf(b"%s:\n\0" as *const u8 as *const i8, message);
    row = 0 as i32;
    while (row * 16 as i32) < size {
        printf(b"%03x  \0" as *const u8 as *const i8, row * 16 as i32);
        col = 0 as i32;
        while col < 16 as i32 {
            printf(
                b"%02x \0" as *const u8 as *const i8,
                *data.offset((row * 16 as i32 + col) as isize) as i32,
            );
            col += 1;
            col;
        }
        col = 0 as i32;
        while col < 16 as i32 {
            if *(*__ctype_b_loc())
                .offset(*data.offset((row * 16 as i32 + col) as isize) as i32 as isize)
                as i32 & _ISprint as i32 as libc::c_ushort as i32 != 0
            {
                printf(
                    b"%c\0" as *const u8 as *const i8,
                    *data.offset((row * 16 as i32 + col) as isize) as i32,
                );
            } else {
                printf(b".\0" as *const u8 as *const i8);
            }
            col += 1;
            col;
        }
        printf(b"\n\0" as *const u8 as *const i8);
        row += 1;
        row;
    }
}
#[no_mangle]
pub unsafe extern "C" fn getTimeNow(mut now: *mut time_t) -> time_t {
    static mut haveTime: i32 = 0 as i32;
    static mut sharedNow: time_t = 0;
    if haveTime == 0 {
        let mut source_date_epoch: *const i8 = getenv(
            b"SOURCE_DATE_EPOCH\0" as *const u8 as *const i8,
        );
        if !source_date_epoch.is_null() {
            let mut endptr: *mut i8 = 0 as *mut i8;
            let mut epoch: time_t = strtol(source_date_epoch, &mut endptr, 10 as i32);
            *__errno_location() = 0 as i32;
            if endptr == source_date_epoch as *mut i8 {
                fprintf(
                    stderr,
                    b"SOURCE_DATE_EPOCH \"%s\" invalid\n\0" as *const u8 as *const i8,
                    source_date_epoch,
                );
            } else if *__errno_location() != 0 as i32 {
                fprintf(
                    stderr,
                    b"SOURCE_DATE_EPOCH: strtoll: %s: %s\n\0" as *const u8 as *const i8,
                    strerror(*__errno_location()),
                    source_date_epoch,
                );
            } else if *endptr as i32 != '\0' as i32 {
                fprintf(
                    stderr,
                    b"SOURCE_DATE_EPOCH has trailing garbage \"%s\"\n\0" as *const u8
                        as *const i8,
                    endptr,
                );
            } else {
                sharedNow = epoch;
                haveTime = 1 as i32;
            }
        }
    }
    if haveTime == 0 {
        time(&mut sharedNow);
        haveTime = 1 as i32;
    }
    if !now.is_null() {
        *now = sharedNow;
    }
    return sharedNow;
}
#[no_mangle]
pub unsafe extern "C" fn str_to_off_with_end(
    mut str: *const i8,
    mut endp: *mut *mut i8,
) -> mt_off_t {
    let mut s: i8 = 0;
    let mut siz: mt_off_t = 0;
    *endp = 0 as *mut i8;
    siz = strtol(str, endp, 0 as i32);
    s = **endp;
    if s as i32 == 's' as i32 || s as i32 == 'S' as i32 {
        siz <<= 9 as i32;
    } else if s as i32 == 'k' as i32 || s as i32 == 'K' as i32 {
        siz <<= 10 as i32;
    } else if s as i32 == 'm' as i32 || s as i32 == 'M' as i32 {
        siz <<= 20 as i32;
    } else if s as i32 == 'g' as i32 || s as i32 == 'G' as i32 {
        siz <<= 30 as i32;
    } else if s as i32 == 't' as i32 || s as i32 == 'T' as i32 {
        siz <<= 40 as i32;
    } else {
        return siz
    }
    *endp = (*endp).offset(1);
    *endp;
    return siz;
}
#[no_mangle]
pub unsafe extern "C" fn str_to_offset(mut str: *mut i8) -> mt_off_t {
    let mut end: *mut i8 = 0 as *mut i8;
    let mut ofs: mt_off_t = str_to_off_with_end(str, &mut end);
    if ofs <= 0 as i32 as i64 {
        return 0 as i32 as mt_off_t;
    }
    if *end != 0 {
        return 0 as i32 as mt_off_t;
    }
    return ofs;
}
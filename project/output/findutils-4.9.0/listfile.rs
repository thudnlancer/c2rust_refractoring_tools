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
    fn __errno_location() -> *mut i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    fn rpl_free(ptr: *mut libc::c_void);
    fn strlen(_: *const i8) -> u64;
    fn strftime(
        __s: *mut i8,
        __maxsize: size_t,
        __format: *const i8,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn areadlinkat(fd: i32, filename: *const i8) -> *mut i8;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn strmode(mode: mode_t, str: *mut i8);
    fn human_readable(
        _: uintmax_t,
        _: *mut i8,
        _: i32,
        _: uintmax_t,
        _: uintmax_t,
    ) -> *mut i8;
    fn gnu_mbswidth(string: *const i8, flags: i32) -> i32;
    fn getuser(uid: uid_t) -> *mut i8;
    fn getgroup(gid: gid_t) -> *mut i8;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
}
pub type __uintmax_t = u64;
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
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
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
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const i8,
}
pub type uintmax_t = __uintmax_t;
pub type C2RustUnnamed = u32;
pub const human_B: C2RustUnnamed = 256;
pub const human_SI: C2RustUnnamed = 128;
pub const human_space_before_unit: C2RustUnnamed = 64;
pub const human_base_1024: C2RustUnnamed = 32;
pub const human_autoscale: C2RustUnnamed = 16;
pub const human_suppress_point_zero: C2RustUnnamed = 8;
pub const human_group_digits: C2RustUnnamed = 4;
pub const human_floor: C2RustUnnamed = 2;
pub const human_round_to_nearest: C2RustUnnamed = 1;
pub const human_ceiling: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: i32,
}
#[inline]
unsafe extern "C" fn gnu_dev_major(mut __dev: __dev_t) -> u32 {
    let mut __major: u32 = 0;
    __major = ((__dev & 0xfff00 as u32 as __dev_t) >> 8 as i32) as u32;
    __major = (__major as u64 | (__dev & 0xfffff00000000000 as u64) >> 32 as i32) as u32;
    return __major;
}
#[inline]
unsafe extern "C" fn gnu_dev_minor(mut __dev: __dev_t) -> u32 {
    let mut __minor: u32 = 0;
    __minor = ((__dev & 0xff as u32 as __dev_t) >> 0 as i32) as u32;
    __minor = (__minor as u64 | (__dev & 0xffffff00000 as u64) >> 12 as i32) as u32;
    return __minor;
}
static mut inode_number_width: i32 = 9 as i32;
static mut block_size_width: i32 = 6 as i32;
static mut nlink_width: i32 = 3 as i32;
static mut owner_width: i32 = 8 as i32;
static mut group_width: i32 = 8 as i32;
static mut major_device_number_width: i32 = 3 as i32;
static mut minor_device_number_width: i32 = 3 as i32;
static mut file_size_width: i32 = 8 as i32;
unsafe extern "C" fn print_num(
    mut stream: *mut FILE,
    mut num: u64,
    mut width: *mut i32,
) -> bool {
    let chars_out: i32 = fprintf(
        stream,
        b"%*lu\0" as *const u8 as *const i8,
        *width,
        num,
    );
    if chars_out >= 0 as i32 {
        if *width < chars_out {
            *width = chars_out;
        }
        return 1 as i32 != 0;
    }
    return 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn list_file(
    mut name: *const i8,
    mut dir_fd: i32,
    mut relname: *const i8,
    mut statp: *const stat,
    mut current_time: time_t,
    mut output_block_size: i32,
    mut literal_control_chars: i32,
    mut stream: *mut FILE,
) {
    let mut modebuf: [i8; 12] = [0; 12];
    let mut when_local: *const tm = 0 as *const tm;
    let mut user_name: *const i8 = 0 as *const i8;
    let mut group_name: *const i8 = 0 as *const i8;
    let mut hbuf: [i8; 652] = [0; 652];
    let mut output_good: bool = 1 as i32 != 0;
    let mut chars_out: i32 = 0;
    let mut failed_at: i32 = 0 as i32;
    strmode((*statp).st_mode, modebuf.as_mut_ptr());
    chars_out = fprintf(
        stream,
        b"%*s\0" as *const u8 as *const i8,
        inode_number_width,
        human_readable(
            (*statp).st_ino,
            hbuf.as_mut_ptr(),
            human_ceiling as i32,
            1 as u32 as uintmax_t,
            1 as u32 as uintmax_t,
        ),
    );
    if chars_out < 0 as i32 {
        output_good = 0 as i32 != 0;
        failed_at = 100 as i32;
    } else if chars_out > inode_number_width {
        inode_number_width = chars_out;
    }
    if output_good {
        if -(1 as i32) == _IO_putc(' ' as i32, stream) {
            output_good = 0 as i32 != 0;
            failed_at = 150 as i32;
        }
        chars_out = fprintf(
            stream,
            b"%*s\0" as *const u8 as *const i8,
            block_size_width,
            human_readable(
                (*statp).st_blocks as uintmax_t,
                hbuf.as_mut_ptr(),
                human_ceiling as i32,
                512 as i32 as uintmax_t,
                output_block_size as uintmax_t,
            ),
        );
        if chars_out < 0 as i32 {
            output_good = 0 as i32 != 0;
            failed_at = 200 as i32;
        } else if chars_out > block_size_width {
            block_size_width = chars_out;
        }
    }
    if output_good {
        if -(1 as i32) == _IO_putc(' ' as i32, stream) {
            output_good = 0 as i32 != 0;
            failed_at = 250 as i32;
        }
    }
    if output_good {
        if fputs(modebuf.as_mut_ptr(), stream) < 0 as i32 {
            output_good = 0 as i32 != 0;
            failed_at = 275 as i32;
        }
    }
    if output_good {
        chars_out = fprintf(
            stream,
            b"%*lu\0" as *const u8 as *const i8,
            nlink_width,
            (*statp).st_nlink,
        );
        if chars_out < 0 as i32 {
            output_good = 0 as i32 != 0;
            failed_at = 300 as i32;
        } else if chars_out > nlink_width {
            nlink_width = chars_out;
        }
    }
    if output_good {
        if -(1 as i32) == _IO_putc(' ' as i32, stream) {
            output_good = 0 as i32 != 0;
            failed_at = 250 as i32;
        }
        user_name = getuser((*statp).st_uid);
        if !user_name.is_null() {
            let mut len: i32 = gnu_mbswidth(user_name, 0 as i32);
            if len > owner_width {
                owner_width = len;
            }
            output_good = fprintf(
                stream,
                b"%-*s \0" as *const u8 as *const i8,
                owner_width,
                user_name,
            ) >= 0 as i32;
            if !output_good {
                failed_at = 400 as i32;
            }
        } else {
            chars_out = fprintf(
                stream,
                b"%-8lu \0" as *const u8 as *const i8,
                (*statp).st_uid as u64,
            );
            if chars_out > owner_width {
                owner_width = chars_out;
            }
            output_good = chars_out > 0 as i32;
            if !output_good {
                failed_at = 450 as i32;
            }
        }
    }
    if output_good {
        group_name = getgroup((*statp).st_gid);
        if !group_name.is_null() {
            let mut len_0: i32 = gnu_mbswidth(group_name, 0 as i32);
            if len_0 > group_width {
                group_width = len_0;
            }
            output_good = fprintf(
                stream,
                b"%-*s \0" as *const u8 as *const i8,
                group_width,
                group_name,
            ) >= 0 as i32;
            if !output_good {
                failed_at = 500 as i32;
            }
        } else {
            chars_out = fprintf(
                stream,
                b"%-*lu\0" as *const u8 as *const i8,
                group_width,
                (*statp).st_gid as u64,
            );
            if chars_out > group_width {
                group_width = chars_out;
            }
            output_good = chars_out >= 0 as i32;
            if output_good {
                if -(1 as i32) == _IO_putc(' ' as i32, stream) {
                    output_good = 0 as i32 != 0;
                    failed_at = 525 as i32;
                }
            } else if !output_good {
                failed_at = 550 as i32;
            }
        }
    }
    if output_good {
        if (*statp).st_mode & 0o170000 as i32 as u32 == 0o20000 as i32 as u32
            || (*statp).st_mode & 0o170000 as i32 as u32 == 0o60000 as i32 as u32
        {
            if !print_num(
                stream,
                gnu_dev_major((*statp).st_rdev) as u64,
                &mut major_device_number_width,
            ) {
                output_good = 0 as i32 != 0;
                failed_at = 600 as i32;
            }
            if output_good {
                if fprintf(stream, b", \0" as *const u8 as *const i8) < 0 as i32 {
                    output_good = 0 as i32 != 0;
                    failed_at = 625 as i32;
                }
            }
            if output_good {
                if !print_num(
                    stream,
                    gnu_dev_minor((*statp).st_rdev) as u64,
                    &mut minor_device_number_width,
                ) {
                    output_good = 0 as i32 != 0;
                    failed_at = 650 as i32;
                }
            }
        } else {
            let blocksize: i32 = if output_block_size < 0 as i32 {
                output_block_size
            } else {
                1 as i32
            };
            chars_out = fprintf(
                stream,
                b"%*s\0" as *const u8 as *const i8,
                file_size_width,
                human_readable(
                    (*statp).st_size as uintmax_t,
                    hbuf.as_mut_ptr(),
                    human_ceiling as i32,
                    1 as i32 as uintmax_t,
                    blocksize as uintmax_t,
                ),
            );
            if chars_out < 0 as i32 {
                output_good = 0 as i32 != 0;
                failed_at = 800 as i32;
            } else if chars_out > file_size_width {
                file_size_width = chars_out;
            }
        }
    }
    if output_good {
        if -(1 as i32) == _IO_putc(' ' as i32, stream) {
            output_good = 0 as i32 != 0;
            failed_at = 850 as i32;
        }
    }
    if output_good {
        when_local = localtime(&(*statp).st_mtim.tv_sec);
        if !when_local.is_null() {
            let mut init_bigbuf: [i8; 256] = [0; 256];
            let mut buf: *mut i8 = init_bigbuf.as_mut_ptr();
            let mut bufsize: size_t = ::core::mem::size_of::<[i8; 256]>() as u64;
            let mut fmt: *const i8 = if current_time
                - (6 as i32 * 30 as i32 * 24 as i32 * 60 as i32 * 60 as i32) as i64
                <= (*statp).st_mtim.tv_sec
                && (*statp).st_mtim.tv_sec
                    <= current_time + (60 as i32 * 60 as i32) as i64
            {
                b"%b %e %H:%M\0" as *const u8 as *const i8
            } else {
                b"%b %e  %Y\0" as *const u8 as *const i8
            };
            while strftime(buf, bufsize, fmt, when_local) == 0 {
                bufsize = (bufsize as u64).wrapping_mul(2 as i32 as u64) as size_t
                    as size_t;
                let mut fresh0 = ::std::vec::from_elem(0, bufsize as usize);
                buf = fresh0.as_mut_ptr() as *mut i8;
            }
            if fprintf(stream, b"%s \0" as *const u8 as *const i8, buf) < 0 as i32 {
                output_good = 0 as i32 != 0;
                failed_at = 900 as i32;
            }
        } else {
            let mut width: i32 = 12 as i32;
            if (*statp).st_mtim.tv_sec < 0 as i32 as i64 {
                let mut num: *const i8 = human_readable(
                    ((*statp).st_mtim.tv_sec as uintmax_t).wrapping_neg(),
                    hbuf.as_mut_ptr(),
                    human_ceiling as i32,
                    1 as i32 as uintmax_t,
                    1 as i32 as uintmax_t,
                );
                let mut sign_width: i32 = (width as u64).wrapping_sub(strlen(num))
                    as i32;
                if fprintf(
                    stream,
                    b"%*s%s \0" as *const u8 as *const i8,
                    (if sign_width < 0 as i32 { 0 as i32 } else { sign_width }),
                    b"-\0" as *const u8 as *const i8,
                    num,
                ) < 0 as i32
                {
                    output_good = 0 as i32 != 0;
                    failed_at = 1000 as i32;
                }
            } else if fprintf(
                stream,
                b"%*s \0" as *const u8 as *const i8,
                width,
                human_readable(
                    (*statp).st_mtim.tv_sec as uintmax_t,
                    hbuf.as_mut_ptr(),
                    human_ceiling as i32,
                    1 as i32 as uintmax_t,
                    1 as i32 as uintmax_t,
                ),
            ) < 0 as i32
            {
                output_good = 0 as i32 != 0;
                failed_at = 1100 as i32;
            }
        }
    }
    if output_good {
        output_good = print_name(name, stream, literal_control_chars);
        if !output_good {
            failed_at = 1200 as i32;
        }
    }
    if output_good {
        if (*statp).st_mode & 0o170000 as i32 as u32 == 0o120000 as i32 as u32 {
            let mut linkname: *mut i8 = areadlinkat(dir_fd, relname);
            if !linkname.is_null() {
                if fputs(b" -> \0" as *const u8 as *const i8, stream) < 0 as i32 {
                    output_good = 0 as i32 != 0;
                    failed_at = 1300 as i32;
                }
                if output_good {
                    output_good = print_name(linkname, stream, literal_control_chars);
                    if !output_good {
                        failed_at = 1350 as i32;
                    }
                }
            } else {
                error(
                    0 as i32,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const i8,
                    name,
                );
            }
            rpl_free(linkname as *mut libc::c_void);
        }
        if output_good {
            if -(1 as i32) == _IO_putc('\n' as i32, stream) {
                output_good = 0 as i32 != 0;
                if !output_good {
                    failed_at = 1400 as i32;
                }
            }
        }
    }
    if !output_good {
        if ::core::mem::size_of::<C2RustUnnamed_0>() as u64 != 0 {
            error(
                1 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"Failed to write output (at stage %d)\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                failed_at,
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
                    b"Failed to write output (at stage %d)\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                failed_at,
            );
            if 0 as i32 != 0 {} else {
                unreachable!();
            };
        };
    }
}
unsafe extern "C" fn print_name_without_quoting(
    mut p: *const i8,
    mut stream: *mut FILE,
) -> bool {
    return fprintf(stream, b"%s\0" as *const u8 as *const i8, p) >= 0 as i32;
}
unsafe extern "C" fn print_name_with_quoting(
    mut p: *const i8,
    mut stream: *mut FILE,
) -> bool {
    let mut c: u8 = 0;
    loop {
        let fresh1 = p;
        p = p.offset(1);
        c = *fresh1 as u8;
        if !(c as i32 != '\0' as i32) {
            break;
        }
        let mut fprintf_result: i32 = -(1 as i32);
        match c as i32 {
            92 => {
                fprintf_result = fprintf(stream, b"\\\\\0" as *const u8 as *const i8);
            }
            10 => {
                fprintf_result = fprintf(stream, b"\\n\0" as *const u8 as *const i8);
            }
            8 => {
                fprintf_result = fprintf(stream, b"\\b\0" as *const u8 as *const i8);
            }
            13 => {
                fprintf_result = fprintf(stream, b"\\r\0" as *const u8 as *const i8);
            }
            9 => {
                fprintf_result = fprintf(stream, b"\\t\0" as *const u8 as *const i8);
            }
            12 => {
                fprintf_result = fprintf(stream, b"\\f\0" as *const u8 as *const i8);
            }
            32 => {
                fprintf_result = fprintf(stream, b"\\ \0" as *const u8 as *const i8);
            }
            34 => {
                fprintf_result = fprintf(stream, b"\\\"\0" as *const u8 as *const i8);
            }
            _ => {
                if c as i32 > 0o40 as i32 && (c as i32) < 0o177 as i32 {
                    if -(1 as i32) == _IO_putc(c as i32, stream) {
                        return 0 as i32 != 0;
                    }
                    fprintf_result = 1 as i32;
                } else {
                    fprintf_result = fprintf(
                        stream,
                        b"\\%03o\0" as *const u8 as *const i8,
                        c as u32,
                    );
                }
            }
        }
        if fprintf_result < 0 as i32 {
            return 0 as i32 != 0;
        }
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn print_name(
    mut p: *const i8,
    mut stream: *mut FILE,
    mut literal_control_chars: i32,
) -> bool {
    if literal_control_chars != 0 {
        return print_name_without_quoting(p, stream)
    } else {
        return print_name_with_quoting(p, stream)
    };
}
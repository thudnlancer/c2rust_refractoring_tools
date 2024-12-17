#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type __dirstream;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn rpl_free(ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn pathconf(__path: *const libc::c_char, __name: libc::c_int) -> libc::c_long;
    fn fpathconf(__fd: libc::c_int, __name: libc::c_int) -> libc::c_long;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn base_len(filename: *const libc::c_char) -> size_t;
    fn opendirat(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut libc::c_int,
    ) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn rewinddir(__dirp: *mut DIR);
    fn renameatu(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_uint,
    ) -> libc::c_int;
}
pub type __ino_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum backup_type {
    numbered_backups = 3,
    numbered_existing_backups = 2,
    simple_backups = 1,
    no_backups = 0,
}  // end of enum

pub type idx_t = ptrdiff_t;
pub type ptrdiff_t = libc::c_long;
pub type DIR = __dirstream;
pub const BACKUP_NOMEM: numbered_backup_result = 3;
pub const _PC_NAME_MAX: C2RustUnnamed_0 = 3;
pub const BACKUP_IS_LONGER: numbered_backup_result = 1;
pub const BACKUP_IS_NEW: numbered_backup_result = 2;
pub const BACKUP_IS_SAME_LENGTH: numbered_backup_result = 0;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum numbered_backup_result {
    BACKUP_NOMEM = 3,
    BACKUP_IS_LONGER = 1,
    BACKUP_IS_NEW = 2,
    BACKUP_IS_SAME_LENGTH = 0,
}  // end of enum

pub type numbered_backup_result = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub const GUESS: C2RustUnnamed = 9;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    GUESS = 9,
}  // end of enum

pub type C2RustUnnamed = libc::c_uint;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    _PC_NAME_MAX = 3,
    _PC_2_SYMLINKS = 20,
    _PC_SYMLINK_MAX = 19,
    _PC_ALLOC_SIZE_MIN = 18,
    _PC_REC_XFER_ALIGN = 17,
    _PC_REC_MIN_XFER_SIZE = 16,
    _PC_REC_MAX_XFER_SIZE = 15,
    _PC_REC_INCR_XFER_SIZE = 14,
    _PC_FILESIZEBITS = 13,
    _PC_SOCK_MAXBUF = 12,
    _PC_PRIO_IO = 11,
    _PC_ASYNC_IO = 10,
    _PC_SYNC_IO = 9,
    _PC_VDISABLE = 8,
    _PC_NO_TRUNC = 7,
    _PC_CHOWN_RESTRICTED = 6,
    _PC_PIPE_BUF = 5,
    _PC_PATH_MAX = 4,
    _PC_MAX_INPUT = 2,
    _PC_MAX_CANON = 1,
    _PC_LINK_MAX = 0,
}  // end of enum

#[no_mangle]
pub static mut simple_backup_suffix: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn set_simple_backup_suffix(mut s: *const libc::c_char) {
    if s.is_null() {
        s = getenv(b"SIMPLE_BACKUP_SUFFIX\0" as *const u8 as *const libc::c_char);
    }
    simple_backup_suffix = if !s.is_null() && *s as libc::c_int != 0
        && s == last_component(s)
    {
        s
    } else {
        b"~\0" as *const u8 as *const libc::c_char
    };
}
unsafe extern "C" fn check_extension(
    mut file: *mut libc::c_char,
    mut filelen: size_t,
    mut e: libc::c_char,
    mut dir_fd: libc::c_int,
    mut base_max: *mut size_t,
) {
    let mut base: *mut libc::c_char = last_component(file);
    let mut baselen: size_t = base_len(base);
    let mut baselen_max: size_t = (if 1 as libc::c_int != 0 {
        255 as libc::c_int
    } else {
        14 as libc::c_int
    }) as size_t;
    if 0 as libc::c_int != 0 || (14 as libc::c_int as libc::c_ulong) < baselen {
        if *base_max == 0 as libc::c_int as libc::c_ulong {
            let mut name_max: libc::c_long = 0;
            if dir_fd < 0 as libc::c_int {
                let mut tmp: [libc::c_char; 2] = [0; 2];
                memcpy(
                    tmp.as_mut_ptr() as *mut libc::c_void,
                    base as *const libc::c_void,
                    ::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
                );
                strcpy(base, b".\0" as *const u8 as *const libc::c_char);
                *__errno_location() = 0 as libc::c_int;
                name_max = pathconf(file, _PC_NAME_MAX as libc::c_int);
                name_max -= (*__errno_location() == 0) as libc::c_int as libc::c_long;
                memcpy(
                    base as *mut libc::c_void,
                    tmp.as_mut_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
                );
            } else {
                *__errno_location() = 0 as libc::c_int;
                name_max = fpathconf(dir_fd, _PC_NAME_MAX as libc::c_int);
                name_max -= (*__errno_location() == 0) as libc::c_int as libc::c_long;
            }
            *base_max = if 0 as libc::c_int as libc::c_long <= name_max
                && name_max as libc::c_ulong <= 18446744073709551615 as libc::c_ulong
            {
                name_max as libc::c_ulong
            } else if name_max < -(1 as libc::c_int) as libc::c_long {
                14 as libc::c_int as libc::c_ulong
            } else {
                18446744073709551615 as libc::c_ulong
            };
        }
        baselen_max = *base_max;
    }
    if 0 as libc::c_int != 0 && baselen_max <= 12 as libc::c_int as libc::c_ulong {
        let mut dot: *mut libc::c_char = strchr(base, '.' as i32);
        if dot.is_null() {
            baselen_max = 8 as libc::c_int as size_t;
        } else {
            let mut second_dot: *const libc::c_char = strchr(
                dot.offset(1 as libc::c_int as isize),
                '.' as i32,
            );
            baselen_max = (if !second_dot.is_null() {
                second_dot.offset_from(base) as libc::c_long
            } else {
                dot.offset(1 as libc::c_int as isize).offset_from(base) as libc::c_long
                    + 3 as libc::c_int as libc::c_long
            }) as size_t;
        }
    }
    if baselen_max < baselen {
        baselen = file.offset(filelen as isize).offset_from(base) as libc::c_long
            as size_t;
        if baselen_max <= baselen {
            baselen = baselen_max.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        *base.offset(baselen as isize) = e;
        *base
            .offset(
                baselen.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
    }
}
unsafe extern "C" fn numbered_backup(
    mut dir_fd: libc::c_int,
    mut buffer: *mut *mut libc::c_char,
    mut buffer_size: size_t,
    mut filelen: size_t,
    mut base_offset: idx_t,
    mut dirpp: *mut *mut DIR,
    mut pnew_fd: *mut libc::c_int,
) -> numbered_backup_result {
    let mut result: numbered_backup_result = BACKUP_IS_NEW;
    let mut dirp: *mut DIR = *dirpp;
    let mut dp: *mut dirent = 0 as *mut dirent;
    let mut buf: *mut libc::c_char = *buffer;
    let mut versionlenmax: size_t = 1 as libc::c_int as size_t;
    let mut base: *mut libc::c_char = buf.offset(base_offset as isize);
    let mut baselen: size_t = base_len(base);
    if !dirp.is_null() {
        rewinddir(dirp);
    } else {
        let mut tmp: [libc::c_char; 2] = [0; 2];
        memcpy(
            tmp.as_mut_ptr() as *mut libc::c_void,
            base as *const libc::c_void,
            ::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
        );
        strcpy(base, b".\0" as *const u8 as *const libc::c_char);
        dirp = opendirat(dir_fd, buf, 0 as libc::c_int, pnew_fd);
        if dirp.is_null() && *__errno_location() == 12 as libc::c_int {
            result = BACKUP_NOMEM;
        }
        memcpy(
            base as *mut libc::c_void,
            tmp.as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
        );
        strcpy(
            base.offset(baselen as isize),
            b".~1~\0" as *const u8 as *const libc::c_char,
        );
        if dirp.is_null() {
            return result;
        }
        *dirpp = dirp;
    }
    loop {
        dp = readdir(dirp);
        if dp.is_null() {
            break;
        }
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut all_9s: bool = false;
        let mut versionlen: size_t = 0;
        if strlen(((*dp).d_name).as_mut_ptr())
            < baselen.wrapping_add(4 as libc::c_int as libc::c_ulong)
        {
            continue;
        }
        if memcmp(
            buf.offset(base_offset as isize) as *const libc::c_void,
            ((*dp).d_name).as_mut_ptr() as *const libc::c_void,
            baselen.wrapping_add(2 as libc::c_int as libc::c_ulong),
        ) != 0 as libc::c_int
        {
            continue;
        }
        p = ((*dp).d_name)
            .as_mut_ptr()
            .offset(baselen as isize)
            .offset(2 as libc::c_int as isize);
        if !('1' as i32 <= *p as libc::c_int && *p as libc::c_int <= '9' as i32) {
            continue;
        }
        all_9s = *p as libc::c_int == '9' as i32;
        versionlen = 1 as libc::c_int as size_t;
        while (*p.offset(versionlen as isize) as libc::c_uint)
            .wrapping_sub('0' as i32 as libc::c_uint) <= 9 as libc::c_int as libc::c_uint
        {
            all_9s = (all_9s as libc::c_int
                & (*p.offset(versionlen as isize) as libc::c_int == '9' as i32)
                    as libc::c_int) as bool;
            versionlen = versionlen.wrapping_add(1);
            versionlen;
        }
        if !(*p.offset(versionlen as isize) as libc::c_int == '~' as i32
            && *p
                .offset(
                    versionlen.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) == 0
            && (versionlenmax < versionlen
                || versionlenmax == versionlen
                    && memcmp(
                        buf.offset(filelen as isize).offset(2 as libc::c_int as isize)
                            as *const libc::c_void,
                        p as *const libc::c_void,
                        versionlen,
                    ) <= 0 as libc::c_int))
        {
            continue;
        }
        versionlenmax = (all_9s as libc::c_ulong).wrapping_add(versionlen);
        result = (if all_9s as libc::c_int != 0 {
            BACKUP_IS_LONGER as libc::c_int
        } else {
            BACKUP_IS_SAME_LENGTH as libc::c_int
        }) as numbered_backup_result;
        let mut new_buffer_size: size_t = filelen
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(versionlenmax)
            .wrapping_add(2 as libc::c_int as libc::c_ulong);
        if buffer_size < new_buffer_size {
            if !((if (9223372036854775807 as libc::c_long as libc::c_ulong)
                < 18446744073709551615 as libc::c_ulong
            {
                9223372036854775807 as libc::c_long as libc::c_ulong
            } else {
                (18446744073709551615 as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            })
                .wrapping_div(2 as libc::c_int as libc::c_ulong) < new_buffer_size)
            {
                new_buffer_size = (new_buffer_size as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            }
            let mut new_buf: *mut libc::c_char = realloc(
                buf as *mut libc::c_void,
                new_buffer_size,
            ) as *mut libc::c_char;
            if new_buf.is_null() {
                *buffer = buf;
                return BACKUP_NOMEM;
            }
            buf = new_buf;
            buffer_size = new_buffer_size;
        }
        q = buf.offset(filelen as isize);
        let fresh0 = q;
        q = q.offset(1);
        *fresh0 = '.' as i32 as libc::c_char;
        let fresh1 = q;
        q = q.offset(1);
        *fresh1 = '~' as i32 as libc::c_char;
        *q = '0' as i32 as libc::c_char;
        q = q.offset(all_9s as libc::c_int as isize);
        memcpy(
            q as *mut libc::c_void,
            p as *const libc::c_void,
            versionlen.wrapping_add(2 as libc::c_int as libc::c_ulong),
        );
        q = q.offset(versionlen as isize);
        loop {
            q = q.offset(-1);
            if !(*q as libc::c_int == '9' as i32) {
                break;
            }
            *q = '0' as i32 as libc::c_char;
        }
        *q += 1;
        *q;
    }
    *buffer = buf;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn backupfile_internal(
    mut dir_fd: libc::c_int,
    mut file: *const libc::c_char,
    mut backup_type: backup_type,
    mut rename: bool,
) -> *mut libc::c_char {
    let mut base_offset: idx_t = (last_component(file)).offset_from(file)
        as libc::c_long;
    let mut filelen: size_t = (base_offset as libc::c_ulong)
        .wrapping_add(strlen(file.offset(base_offset as isize)));
    if simple_backup_suffix.is_null() {
        set_simple_backup_suffix(0 as *const libc::c_char);
    }
    let mut simple_backup_suffix_size: size_t = (strlen(simple_backup_suffix))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut backup_suffix_size_guess: size_t = simple_backup_suffix_size;
    if backup_suffix_size_guess < GUESS as libc::c_int as libc::c_ulong {
        backup_suffix_size_guess = GUESS as libc::c_int as size_t;
    }
    let mut ssize: ssize_t = filelen
        .wrapping_add(backup_suffix_size_guess)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as ssize_t;
    let mut s: *mut libc::c_char = malloc(ssize as libc::c_ulong) as *mut libc::c_char;
    if s.is_null() {
        return s;
    }
    let mut dirp: *mut DIR = 0 as *mut DIR;
    let mut sdir: libc::c_int = -(1 as libc::c_int);
    let mut base_max: size_t = 0 as libc::c_int as size_t;
    loop {
        memcpy(
            s as *mut libc::c_void,
            file as *const libc::c_void,
            filelen.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if backup_type as libc::c_uint == simple_backups as libc::c_int as libc::c_uint {
            memcpy(
                s.offset(filelen as isize) as *mut libc::c_void,
                simple_backup_suffix as *const libc::c_void,
                simple_backup_suffix_size,
            );
        } else {
            let mut current_block_18: u64;
            match numbered_backup(
                dir_fd,
                &mut s,
                ssize as size_t,
                filelen,
                base_offset,
                &mut dirp,
                &mut sdir,
            ) as libc::c_uint
            {
                2 => {
                    if backup_type as libc::c_uint
                        == numbered_existing_backups as libc::c_int as libc::c_uint
                    {
                        backup_type = simple_backups;
                        memcpy(
                            s.offset(filelen as isize) as *mut libc::c_void,
                            simple_backup_suffix as *const libc::c_void,
                            simple_backup_suffix_size,
                        );
                    }
                    current_block_18 = 8585585342168113498;
                }
                1 => {
                    current_block_18 = 8585585342168113498;
                }
                3 => {
                    if !dirp.is_null() {
                        closedir(dirp);
                    }
                    rpl_free(s as *mut libc::c_void);
                    *__errno_location() = 12 as libc::c_int;
                    return 0 as *mut libc::c_char;
                }
                0 | _ => {
                    current_block_18 = 15089075282327824602;
                }
            }
            match current_block_18 {
                8585585342168113498 => {
                    check_extension(
                        s,
                        filelen,
                        '~' as i32 as libc::c_char,
                        sdir,
                        &mut base_max,
                    );
                }
                _ => {}
            }
        }
        if !rename {
            break;
        }
        if sdir < 0 as libc::c_int {
            sdir = -(100 as libc::c_int);
            base_offset = 0 as libc::c_int as idx_t;
        }
        let mut flags: libc::c_uint = (if backup_type as libc::c_uint
            == simple_backups as libc::c_int as libc::c_uint
        {
            0 as libc::c_int
        } else {
            (1 as libc::c_int) << 0 as libc::c_int
        }) as libc::c_uint;
        if renameatu(
            -(100 as libc::c_int),
            file,
            sdir,
            s.offset(base_offset as isize),
            flags,
        ) == 0 as libc::c_int
        {
            break;
        }
        let mut e: libc::c_int = *__errno_location();
        if e != 17 as libc::c_int {
            if !dirp.is_null() {
                closedir(dirp);
            }
            rpl_free(s as *mut libc::c_void);
            *__errno_location() = e;
            return 0 as *mut libc::c_char;
        }
    }
    if !dirp.is_null() {
        closedir(dirp);
    }
    return s;
}

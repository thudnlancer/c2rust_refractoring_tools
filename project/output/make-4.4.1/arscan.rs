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
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    fn close(__fd: i32) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn __errno_location() -> *mut i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn fatal(flocp: *const floc, length: size_t, fmt: *const i8, _: ...) -> !;
    fn make_toui(_: *const i8, _: *mut *const i8) -> u32;
    fn writebuf(_: i32, _: *const libc::c_void, _: size_t) -> ssize_t;
    fn readbuf(_: i32, _: *mut libc::c_void, _: size_t) -> ssize_t;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
}
pub type size_t = u64;
pub type __intmax_t = i64;
pub type __uintmax_t = u64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
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
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const i8,
    pub lineno: u64,
    pub offset: u64,
}
pub type ar_member_func_t = Option<
    unsafe extern "C" fn(
        i32,
        *const i8,
        i32,
        i64,
        i64,
        i64,
        intmax_t,
        i32,
        i32,
        u32,
        *const libc::c_void,
    ) -> intmax_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ar_hdr {
    pub ar_name: [i8; 16],
    pub ar_date: [i8; 12],
    pub ar_uid: [i8; 6],
    pub ar_gid: [i8; 6],
    pub ar_mode: [i8; 8],
    pub ar_size: [i8; 10],
    pub ar_fmag: [i8; 2],
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
unsafe extern "C" fn parse_int(
    mut ptr: *const i8,
    len: size_t,
    base: i32,
    mut max: uintmax_t,
    mut type_0: *const i8,
    mut archive: *const i8,
    mut name: *const i8,
) -> uintmax_t {
    let ep: *const i8 = ptr.offset(len as isize);
    let maxchar: i32 = '0' as i32 + base - 1 as i32;
    let mut val: uintmax_t = 0 as i32 as uintmax_t;
    while ptr < ep && *ptr as i32 == ' ' as i32 {
        ptr = ptr.offset(1);
        ptr;
    }
    while ptr < ep && *ptr as i32 != ' ' as i32 {
        let mut nv: uintmax_t = 0;
        if (*ptr as i32) < '0' as i32 || *ptr as i32 > maxchar {
            fatal(
                0 as *mut floc,
                (strlen(type_0))
                    .wrapping_add(strlen(archive))
                    .wrapping_add(strlen(name)),
                dcgettext(
                    0 as *const i8,
                    b"Invalid %s for archive %s member %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                type_0,
                archive,
                name,
            );
        }
        nv = val
            .wrapping_mul(base as u64)
            .wrapping_add((*ptr as i32 - '0' as i32) as u64);
        if nv < val || nv > max {
            fatal(
                0 as *mut floc,
                (strlen(type_0))
                    .wrapping_add(strlen(archive))
                    .wrapping_add(strlen(name)),
                dcgettext(
                    0 as *const i8,
                    b"Invalid %s for archive %s member %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                type_0,
                archive,
                name,
            );
        }
        val = nv;
        ptr = ptr.offset(1);
        ptr;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn ar_scan(
    mut archive: *const i8,
    mut function: ar_member_func_t,
    mut arg: *const libc::c_void,
) -> intmax_t {
    let mut current_block: u64;
    let mut namemap: *mut i8 = 0 as *mut i8;
    let mut namemap_size: u32 = 0 as i32 as u32;
    let mut desc: i32 = open(archive, 0 as i32, 0 as i32);
    if desc < 0 as i32 {
        return -(1 as i32) as intmax_t;
    }
    let mut buf: [i8; 8] = [0; 8];
    let mut nread: i32 = 0;
    nread = readbuf(desc, buf.as_mut_ptr() as *mut libc::c_void, 8 as i32 as size_t)
        as i32;
    if !(nread != 8 as i32
        || memcmp(
            buf.as_mut_ptr() as *const libc::c_void,
            b"!<arch>\n\0" as *const u8 as *const i8 as *const libc::c_void,
            8 as i32 as u64,
        ) != 0)
    {
        let mut member_offset: i64 = 8 as i32 as i64;
        loop {
            let mut nread_0: ssize_t = 0;
            let mut member_header: ar_hdr = ar_hdr {
                ar_name: [0; 16],
                ar_date: [0; 12],
                ar_uid: [0; 6],
                ar_gid: [0; 6],
                ar_mode: [0; 8],
                ar_size: [0; 10],
                ar_fmag: [0; 2],
            };
            let mut namebuf: [i8; 17] = [0; 17];
            let mut name: *mut i8 = 0 as *mut i8;
            let mut is_namemap: i32 = 0;
            let mut long_name: i32 = 0 as i32;
            let mut eltsize: i64 = 0;
            let mut eltmode: u32 = 0;
            let mut eltdate: intmax_t = 0;
            let mut eltuid: i32 = 0;
            let mut eltgid: i32 = 0;
            let mut fnval: intmax_t = 0;
            let mut o: off_t = 0;
            memset(
                &mut member_header as *mut ar_hdr as *mut libc::c_void,
                '\0' as i32,
                ::core::mem::size_of::<ar_hdr>() as u64,
            );
            loop {
                o = lseek(desc, member_offset, 0 as i32);
                if !(o == -(1 as i32) as i64 && *__errno_location() == 4 as i32) {
                    break;
                }
            }
            if o < 0 as i32 as i64 {
                current_block = 8030616964507537510;
                break;
            }
            nread_0 = readbuf(
                desc,
                &mut member_header as *mut ar_hdr as *mut libc::c_void,
                ::core::mem::size_of::<ar_hdr>() as u64,
            );
            if nread_0 == 0 as i32 as i64 {
                current_block = 16203797167131938757;
                break;
            }
            if nread_0 as u64 != ::core::mem::size_of::<ar_hdr>() as u64
                || memcmp(
                    (member_header.ar_fmag).as_mut_ptr() as *const libc::c_void,
                    b"`\n\0" as *const u8 as *const i8 as *const libc::c_void,
                    2 as i32 as u64,
                ) != 0 && 1 as i32 != 0
            {
                current_block = 8030616964507537510;
                break;
            }
            name = namebuf.as_mut_ptr();
            memcpy(
                name as *mut libc::c_void,
                (member_header.ar_name).as_mut_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[i8; 16]>() as u64,
            );
            let mut p: *mut i8 = name
                .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize);
            loop {
                *p = '\0' as i32 as i8;
                if !(p > name
                    && {
                        p = p.offset(-1);
                        *p as i32 == ' ' as i32
                    })
                {
                    break;
                }
            }
            is_namemap = (strcmp(name, b"//\0" as *const u8 as *const i8) == 0
                || strcmp(name, b"ARFILENAMES/\0" as *const u8 as *const i8) == 0)
                as i32;
            if *p as i32 == '/' as i32 {
                *p = '\0' as i32 as i8;
            }
            if is_namemap == 0
                && (*name.offset(0 as i32 as isize) as i32 == ' ' as i32
                    || *name.offset(0 as i32 as isize) as i32 == '/' as i32)
                && !namemap.is_null()
            {
                let mut err: *const i8 = 0 as *const i8;
                let mut name_off: u32 = make_toui(
                    name.offset(1 as i32 as isize),
                    &mut err,
                );
                let mut name_len: size_t = 0;
                if !err.is_null() || name_off >= namemap_size {
                    current_block = 8030616964507537510;
                    break;
                }
                name = namemap.offset(name_off as isize);
                name_len = strlen(name);
                if name_len < 1 as i32 as u64 {
                    current_block = 8030616964507537510;
                    break;
                }
                long_name = 1 as i32;
            } else if *name.offset(0 as i32 as isize) as i32 == '#' as i32
                && *name.offset(1 as i32 as isize) as i32 == '1' as i32
                && *name.offset(2 as i32 as isize) as i32 == '/' as i32
            {
                let mut err_0: *const i8 = 0 as *const i8;
                let mut name_len_0: u32 = make_toui(
                    name.offset(3 as i32 as isize),
                    &mut err_0,
                );
                if !err_0.is_null() || name_len_0 == 0 as i32 as u32
                    || name_len_0
                        >= (if (4096 as i32) < 2147483647 as i32 {
                            4096 as i32
                        } else {
                            2147483647 as i32
                        }) as u32
                {
                    current_block = 8030616964507537510;
                    break;
                }
                let mut fresh0 = ::std::vec::from_elem(
                    0,
                    name_len_0.wrapping_add(1 as i32 as u32) as u64 as usize,
                );
                name = fresh0.as_mut_ptr() as *mut i8;
                nread_0 = readbuf(desc, name as *mut libc::c_void, name_len_0 as size_t);
                if nread_0 < 0 as i32 as i64 || nread_0 as u32 != name_len_0 {
                    current_block = 8030616964507537510;
                    break;
                }
                *name.offset(name_len_0 as isize) = '\0' as i32 as i8;
                long_name = 1 as i32;
            }
            eltmode = parse_int(
                (member_header.ar_mode).as_mut_ptr(),
                ::core::mem::size_of::<[i8; 8]>() as u64,
                8 as i32,
                (if (0 as i32 as u32) < -(1 as i32) as u32 {
                    -(1 as i32) as u32
                } else {
                    ((1 as i32 as u32)
                        << (::core::mem::size_of::<u32>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64))
                        .wrapping_sub(1 as i32 as u32)
                        .wrapping_mul(2 as i32 as u32)
                        .wrapping_add(1 as i32 as u32)
                }) as uintmax_t,
                b"mode\0" as *const u8 as *const i8,
                archive,
                name,
            ) as u32;
            eltsize = parse_int(
                (member_header.ar_size).as_mut_ptr(),
                ::core::mem::size_of::<[i8; 10]>() as u64,
                10 as i32,
                (if (0 as i32 as i64) < -(1 as i32) as i64 {
                    -(1 as i32) as i64
                } else {
                    (((1 as i32 as i64)
                        << (::core::mem::size_of::<i64>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                        * 2 as i32 as i64 + 1 as i32 as i64
                }) as uintmax_t,
                b"size\0" as *const u8 as *const i8,
                archive,
                name,
            ) as i64;
            eltdate = parse_int(
                (member_header.ar_date).as_mut_ptr(),
                ::core::mem::size_of::<[i8; 12]>() as u64,
                10 as i32,
                (if (0 as i32 as intmax_t) < -(1 as i32) as intmax_t {
                    -(1 as i32) as intmax_t
                } else {
                    (((1 as i32 as intmax_t)
                        << (::core::mem::size_of::<intmax_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                        * 2 as i32 as i64 + 1 as i32 as i64
                }) as uintmax_t,
                b"date\0" as *const u8 as *const i8,
                archive,
                name,
            ) as intmax_t;
            eltuid = parse_int(
                (member_header.ar_uid).as_mut_ptr(),
                ::core::mem::size_of::<[i8; 6]>() as u64,
                10 as i32,
                (if (0 as i32) < -(1 as i32) {
                    -(1 as i32)
                } else {
                    (((1 as i32)
                        << (::core::mem::size_of::<i32>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                        + 1 as i32
                }) as uintmax_t,
                b"uid\0" as *const u8 as *const i8,
                archive,
                name,
            ) as i32;
            eltgid = parse_int(
                (member_header.ar_gid).as_mut_ptr(),
                ::core::mem::size_of::<[i8; 6]>() as u64,
                10 as i32,
                (if (0 as i32) < -(1 as i32) {
                    -(1 as i32)
                } else {
                    (((1 as i32)
                        << (::core::mem::size_of::<i32>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                        + 1 as i32
                }) as uintmax_t,
                b"gid\0" as *const u8 as *const i8,
                archive,
                name,
            ) as i32;
            fnval = (Some(function.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                desc,
                name,
                (long_name == 0) as i32,
                member_offset,
                (member_offset as u64)
                    .wrapping_add(::core::mem::size_of::<ar_hdr>() as u64) as i64,
                eltsize,
                eltdate,
                eltuid,
                eltgid,
                eltmode,
                arg,
            );
            if fnval != 0 {
                close(desc);
                return fnval;
            }
            if is_namemap != 0 {
                let mut clear: *mut i8 = 0 as *mut i8;
                let mut limit: *mut i8 = 0 as *mut i8;
                if eltsize > 2147483647 as i32 as i64 {
                    current_block = 8030616964507537510;
                    break;
                }
                let mut fresh1 = ::std::vec::from_elem(
                    0,
                    (eltsize + 1 as i32 as i64) as u64 as usize,
                );
                namemap = fresh1.as_mut_ptr() as *mut i8;
                nread_0 = readbuf(desc, namemap as *mut libc::c_void, eltsize as size_t);
                if nread_0 != eltsize {
                    current_block = 8030616964507537510;
                    break;
                }
                namemap_size = eltsize as u32;
                limit = namemap.offset(eltsize as isize);
                clear = namemap;
                while clear < limit {
                    if *clear as i32 == '\n' as i32 {
                        *clear = '\0' as i32 as i8;
                        if *clear.offset(-(1 as i32) as isize) as i32 == '/' as i32 {
                            *clear.offset(-(1 as i32) as isize) = '\0' as i32 as i8;
                        }
                    }
                    clear = clear.offset(1);
                    clear;
                }
                *limit = '\0' as i32 as i8;
                is_namemap = 0 as i32;
            }
            member_offset = (member_offset as u64)
                .wrapping_add(
                    (::core::mem::size_of::<ar_hdr>() as u64)
                        .wrapping_add(eltsize as u64),
                ) as i64 as i64;
            if member_offset % 2 as i32 as i64 != 0 as i32 as i64 {
                member_offset += 1;
                member_offset;
            }
        }
        match current_block {
            8030616964507537510 => {}
            _ => {
                close(desc);
                return 0 as i32 as intmax_t;
            }
        }
    }
    close(desc);
    return -(2 as i32) as intmax_t;
}
#[no_mangle]
pub unsafe extern "C" fn ar_name_equal(
    mut name: *const i8,
    mut mem: *const i8,
    mut truncated: i32,
) -> i32 {
    let mut p: *const i8 = 0 as *const i8;
    p = strrchr(name, '/' as i32);
    if !p.is_null() {
        name = p.offset(1 as i32 as isize);
    }
    if truncated != 0 {
        let mut hdr: ar_hdr = ar_hdr {
            ar_name: [0; 16],
            ar_date: [0; 12],
            ar_uid: [0; 6],
            ar_gid: [0; 6],
            ar_mode: [0; 8],
            ar_size: [0; 10],
            ar_fmag: [0; 2],
        };
        return (strncmp(
            name,
            mem,
            (::core::mem::size_of::<[i8; 16]>() as u64).wrapping_sub(1 as i32 as u64),
        ) == 0 as i32) as i32;
    }
    return (strcmp(name, mem) == 0) as i32;
}
unsafe extern "C" fn ar_member_pos(
    mut desc: i32,
    mut mem: *const i8,
    mut truncated: i32,
    mut hdrpos: i64,
    mut datapos: i64,
    mut size: i64,
    mut date: intmax_t,
    mut uid: i32,
    mut gid: i32,
    mut mode: u32,
    mut name: *const libc::c_void,
) -> intmax_t {
    if ar_name_equal(name as *const i8, mem, truncated) == 0 {
        return 0 as i32 as intmax_t;
    }
    return hdrpos;
}
#[no_mangle]
pub unsafe extern "C" fn ar_member_touch(
    mut arname: *const i8,
    mut memname: *const i8,
) -> i32 {
    let mut pos: intmax_t = ar_scan(
        arname,
        Some(
            ar_member_pos
                as unsafe extern "C" fn(
                    i32,
                    *const i8,
                    i32,
                    i64,
                    i64,
                    i64,
                    intmax_t,
                    i32,
                    i32,
                    u32,
                    *const libc::c_void,
                ) -> intmax_t,
        ),
        memname as *const libc::c_void,
    );
    let mut opos: off_t = 0;
    let mut fd: i32 = 0;
    let mut ar_hdr: ar_hdr = ar_hdr {
        ar_name: [0; 16],
        ar_date: [0; 12],
        ar_uid: [0; 6],
        ar_gid: [0; 6],
        ar_mode: [0; 8],
        ar_size: [0; 10],
        ar_fmag: [0; 2],
    };
    let mut o: off_t = 0;
    let mut r: i32 = 0;
    let mut datelen: i32 = 0;
    let mut statbuf: stat = stat {
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
    if pos < 0 as i32 as i64 {
        return pos as i32;
    }
    if pos == 0 {
        return 1 as i32;
    }
    opos = pos;
    loop {
        fd = open(arname, 0o2 as i32, 0o666 as i32);
        if !(fd == -(1 as i32) && *__errno_location() == 4 as i32) {
            break;
        }
    }
    if fd < 0 as i32 {
        return -(3 as i32);
    }
    loop {
        o = lseek(fd, opos, 0 as i32);
        if !(o == -(1 as i32) as i64 && *__errno_location() == 4 as i32) {
            break;
        }
    }
    if !(o < 0 as i32 as i64) {
        r = readbuf(
            fd,
            &mut ar_hdr as *mut ar_hdr as *mut libc::c_void,
            ::core::mem::size_of::<ar_hdr>() as u64,
        ) as i32;
        if !(r as u64 != ::core::mem::size_of::<ar_hdr>() as u64) {
            loop {
                r = fstat(fd, &mut statbuf);
                if !(r == -(1 as i32) && *__errno_location() == 4 as i32) {
                    break;
                }
            }
            if !(r < 0 as i32) {
                datelen = snprintf(
                    (ar_hdr.ar_date).as_mut_ptr(),
                    ::core::mem::size_of::<[i8; 12]>() as u64,
                    b"%ld\0" as *const u8 as *const i8,
                    statbuf.st_mtim.tv_sec,
                );
                if 0 as i32 <= datelen
                    && datelen < ::core::mem::size_of::<[i8; 12]>() as u64 as i32
                {
                    memset(
                        (ar_hdr.ar_date).as_mut_ptr().offset(datelen as isize)
                            as *mut libc::c_void,
                        ' ' as i32,
                        (::core::mem::size_of::<[i8; 12]>() as u64)
                            .wrapping_sub(datelen as u64),
                    );
                    loop {
                        o = lseek(fd, opos, 0 as i32);
                        if !(o == -(1 as i32) as i64 && *__errno_location() == 4 as i32)
                        {
                            break;
                        }
                    }
                    if !(o < 0 as i32 as i64) {
                        r = writebuf(
                            fd,
                            &mut ar_hdr as *mut ar_hdr as *const libc::c_void,
                            ::core::mem::size_of::<ar_hdr>() as u64,
                        ) as i32;
                        if !(r as u64 != ::core::mem::size_of::<ar_hdr>() as u64) {
                            close(fd);
                            return 0 as i32;
                        }
                    }
                }
            }
        }
    }
    r = *__errno_location();
    close(fd);
    *__errno_location() = r;
    return -(3 as i32);
}
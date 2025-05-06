#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type doscp_t;
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn perror(__s: *const i8);
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __lxstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strpbrk(_: *const i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn mt_basename(filename: *const i8) -> *const i8;
    static mut got_signal: i32;
    fn get_default_drive() -> i8;
    fn copy_stream(Stream: *mut Stream_t) -> *mut Stream_t;
    fn free_stream(Stream: *mut *mut Stream_t) -> i32;
    fn open_root_dir(drivename: i8, flags: i32, isRop: *mut i32) -> *mut Stream_t;
    static mut progname: *const i8;
    fn OpenDir(filename: *const i8) -> *mut Stream_t;
    fn open_mcwd(mode: *const i8) -> *mut FILE;
    fn unlink_mcwd();
    fn fat_error(Dir: *mut Stream_t) -> i32;
    fn vfat_lookup(
        entry: *mut direntry_t,
        filename: *const i8,
        length: size_t,
        flags: i32,
        shortname: *mut i8,
        shortname_len: size_t,
        longname: *mut i8,
        longname_len: size_t,
    ) -> i32;
    fn vfat_lookup_zt(
        entry: *mut direntry_t,
        filename: *const i8,
        flags: i32,
        shortname: *mut i8,
        shortname_len: size_t,
        longname: *mut i8,
        longname_len: size_t,
    ) -> i32;
    fn initializeDirentry(entry: *mut direntry_t, Dir: *mut Stream_t);
    fn getParent(entry: *mut direntry_t) -> *mut direntry_t;
    fn fprintPwd(f: *mut FILE, entry: *mut direntry_t, escape: i32);
    fn unix_dir_loop(Stream: *mut Stream_t, mp: *mut MainParam_t) -> i32;
    fn SimpleFileOpen(
        dev: *mut device,
        orig_dev: *mut device,
        name: *const i8,
        mode: i32,
        errmsg: *mut i8,
        mode2: i32,
        locked: i32,
        maxSize: *mut mt_off_t,
    ) -> *mut Stream_t;
    fn OpenFileByDirentry(entry: *mut direntry_t) -> *mut Stream_t;
    fn getDirentry(Stream: *mut Stream_t) -> *mut direntry_t;
    fn wchar_to_native(
        wchar: *const wchar_t,
        native: *mut i8,
        len: size_t,
        out_len: size_t,
    ) -> size_t;
    fn isSpecialW(name: *const wchar_t) -> i32;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = i32;
pub type __uint32_t = u32;
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
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type wchar_t = i32;
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
pub type mt_off_t = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream_t {
    pub Class: *mut Class_t,
    pub refs: i32,
    pub Next: *mut Stream_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Class_t {
    pub read: Option<unsafe extern "C" fn(*mut Stream_t, *mut i8, size_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut Stream_t, *mut i8, size_t) -> ssize_t>,
    pub pread: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut i8, mt_off_t, size_t) -> ssize_t,
    >,
    pub pwrite: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut i8, mt_off_t, size_t) -> ssize_t,
    >,
    pub flush: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
    pub freeFunc: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
    pub set_geom: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut device_t, *mut device_t) -> i32,
    >,
    pub get_data: Option<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut time_t,
            *mut mt_off_t,
            *mut i32,
            *mut uint32_t,
        ) -> i32,
    >,
    pub pre_allocate: Option<unsafe extern "C" fn(*mut Stream_t, mt_off_t) -> i32>,
    pub get_dosConvert: Option<unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t>,
    pub discard: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
}
pub type device_t = device;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device {
    pub name: *const i8,
    pub drive: i8,
    pub fat_bits: i32,
    pub mode: i32,
    pub tracks: u32,
    pub heads: uint16_t,
    pub sectors: uint16_t,
    pub hidden: u32,
    pub offset: mt_off_t,
    pub partition: u32,
    pub misc_flags: u32,
    pub ssize: uint8_t,
    pub use_2m: u32,
    pub precmd: *mut i8,
    pub file_nr: i32,
    pub blocksize: u32,
    pub codepage: u32,
    pub data_map: *const i8,
    pub tot_sectors: uint32_t,
    pub sector_size: uint16_t,
    pub postcmd: *mut i8,
    pub cfg_filename: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct directory {
    pub name: [i8; 8],
    pub ext: [i8; 3],
    pub attr: u8,
    pub Case: u8,
    pub ctime_ms: u8,
    pub ctime: [u8; 2],
    pub cdate: [u8; 2],
    pub adate: [u8; 2],
    pub startHi: [u8; 2],
    pub time: [u8; 2],
    pub date: [u8; 2],
    pub start: [u8; 2],
    pub size: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct direntry_t {
    pub Dir: *mut Stream_t,
    pub entry: i32,
    pub dir: directory,
    pub name: [wchar_t; 256],
    pub beginSlot: u32,
    pub endSlot: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bounded_string {
    pub data: *mut i8,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MainParam_t {
    pub loop_0: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut MainParam_t, *const i8) -> i32,
    >,
    pub dirCallback: Option<
        unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
    >,
    pub callback: Option<unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32>,
    pub unixcallback: Option<unsafe extern "C" fn(*mut MainParam_t) -> i32>,
    pub arg: *mut libc::c_void,
    pub openflags: i32,
    pub lookupflags: i32,
    pub fast_quit: i32,
    pub shortname: bounded_string,
    pub longname: bounded_string,
    pub File: *mut Stream_t,
    pub direntry: *mut direntry_t,
    pub unixSourceName: *mut i8,
    pub targetDir: *mut Stream_t,
    pub targetName: *const i8,
    pub originalArg: *mut i8,
    pub basenameHasWildcard: i32,
    pub mcwd: [i8; 132],
    pub targetBuffer: [i8; 1021],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lookupState_t {
    pub container: *mut Stream_t,
    pub nbContainers: i32,
    pub Dir: *mut Stream_t,
    pub nbDirs: i32,
    pub filename: *const i8,
}
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn lstat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __lxstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn ch_toupper(mut ch: i8) -> i8 {
    return ({
        let mut __res: i32 = 0;
        if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
            if 0 != 0 {
                let mut __c: i32 = ch as u8 as i32;
                __res = if __c < -(128 as i32) || __c > 255 as i32 {
                    __c
                } else {
                    *(*__ctype_toupper_loc()).offset(__c as isize)
                };
            } else {
                __res = toupper(ch as u8 as i32);
            }
        } else {
            __res = *(*__ctype_toupper_loc()).offset(ch as u8 as i32 as isize);
        }
        __res
    }) as i8;
}
#[inline]
unsafe extern "C" fn ptrdiff(mut end: *const i8, mut begin: *const i8) -> size_t {
    return end.offset_from(begin) as i64 as size_t;
}
unsafe extern "C" fn fix_mcwd(mut ans: *mut i8) -> *const i8 {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut buf: [i8; 128] = [0; 128];
    fp = open_mcwd(b"r\0" as *const u8 as *const i8);
    if fp.is_null() || (fgets(buf.as_mut_ptr(), 128 as i32, fp)).is_null() {
        if !fp.is_null() {
            fclose(fp);
        }
        *ans.offset(0 as i32 as isize) = get_default_drive();
        strcpy(ans.offset(1 as i32 as isize), b":/\0" as *const u8 as *const i8);
        return ans;
    }
    buf[(strlen(buf.as_mut_ptr())).wrapping_sub(1 as i32 as u64) as usize] = '\0' as i32
        as i8;
    fclose(fp);
    s = buf.as_mut_ptr();
    if buf[0 as i32 as usize] as i32 != 0 && buf[1 as i32 as usize] as i32 == ':' as i32
    {
        memcpy(
            ans as *mut libc::c_void,
            buf.as_mut_ptr() as *const libc::c_void,
            2 as i32 as u64,
        );
        *ans.offset(2 as i32 as isize) = '\0' as i32 as i8;
        s = &mut *buf.as_mut_ptr().offset(2 as i32 as isize) as *mut i8;
    } else {
        *ans.offset(0 as i32 as isize) = get_default_drive();
        strcpy(ans.offset(1 as i32 as isize), b":\0" as *const u8 as *const i8);
    }
    if *s as i32 != '/' as i32 && *s as i32 != '\\' as i32 {
        strcat(ans, b"/\0" as *const u8 as *const i8);
        strcat(ans, s);
    } else {
        strcat(ans, s);
    }
    if strlen(ans) == 3 as i32 as u64 {
        return ans;
    }
    s = s.offset(-1);
    if *s as i32 == '/' as i32 {
        *s = '\0' as i32 as i8;
    }
    return ans;
}
unsafe extern "C" fn mt_unix_loop(
    mut Dir: *mut Stream_t,
    mut mp: *mut MainParam_t,
    mut filename: *const i8,
) -> i32 {
    return unix_dir_loop(Dir, mp);
}
#[no_mangle]
pub unsafe extern "C" fn unix_loop(
    mut Stream: *mut Stream_t,
    mut mp: *mut MainParam_t,
    mut arg: *mut i8,
    mut follow_dir_link: i32,
) -> i32 {
    let mut ret: i32 = 0;
    let mut isdir: i32 = 0 as i32;
    let mut unixNameLength: size_t = 0;
    (*mp).File = 0 as *mut Stream_t;
    (*mp).direntry = 0 as *mut direntry_t;
    unixNameLength = strlen(arg);
    if unixNameLength > 1 as i32 as u64
        && *arg.offset(unixNameLength.wrapping_sub(1 as i32 as u64) as isize) as i32
            == '/' as i32
    {
        let mut name: *mut i8 = strdup(arg);
        *name.offset(unixNameLength.wrapping_sub(1 as i32 as u64) as isize) = '\0' as i32
            as i8;
        (*mp).unixSourceName = name;
    } else {
        (*mp).unixSourceName = arg;
    }
    (*mp).loop_0 = Some(
        mt_unix_loop
            as unsafe extern "C" fn(*mut Stream_t, *mut MainParam_t, *const i8) -> i32,
    );
    if (*mp).lookupflags & 1 as i32 != 0 {
        (*mp).File = SimpleFileOpen(
            0 as *mut device,
            0 as *mut device,
            arg,
            0 as i32,
            0 as *mut i8,
            0 as i32,
            0 as i32,
            0 as *mut mt_off_t,
        );
        if ((*mp).File).is_null() {
            perror(arg);
            return 16 as i32;
        }
        ((*(*(*mp).File).Class).get_data)
            .expect(
                "non-null function pointer",
            )(
            (*mp).File,
            0 as *mut time_t,
            0 as *mut mt_off_t,
            &mut isdir,
            0 as *mut uint32_t,
        );
        if isdir != 0 {
            let mut buf: stat = stat {
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
            free_stream(&mut (*mp).File);
            if follow_dir_link == 0 && lstat(arg, &mut buf) == 0 as i32
                && buf.st_mode & 0o170000 as i32 as u32 == 0o120000 as i32 as u32
            {
                fprintf(
                    stderr,
                    b"skipping directory symlink %s\n\0" as *const u8 as *const i8,
                    arg,
                );
                return 0 as i32;
            }
            if (*mp).lookupflags & 0x10 as i32 == 0 {
                return 0 as i32;
            }
            (*mp).File = OpenDir(arg);
        }
    }
    if isdir != 0 {
        ret = ((*mp).dirCallback)
            .expect("non-null function pointer")(0 as *mut direntry_t, mp);
    } else {
        ret = ((*mp).unixcallback).expect("non-null function pointer")(mp);
    }
    free_stream(&mut (*mp).File);
    return ret;
}
unsafe extern "C" fn checkForDot(mut lookupflags: i32, mut name: *const wchar_t) -> i32 {
    return (lookupflags & 0x100 as i32 != 0 && isSpecialW(name) != 0) as i32;
}
unsafe extern "C" fn isUniqueTarget(mut name: *const i8) -> i32 {
    return (!name.is_null() && strcmp(name, b"-\0" as *const u8 as *const i8) != 0)
        as i32;
}
unsafe extern "C" fn handle_leaf(
    mut direntry: *mut direntry_t,
    mut mp: *mut MainParam_t,
    mut lookupState: *mut lookupState_t,
    mut DeferredFileP: *mut *mut Stream_t,
) -> i32 {
    let mut MyFile: *mut Stream_t = 0 as *mut Stream_t;
    let mut ret: i32 = 0;
    if got_signal != 0 {
        return 16 as i32;
    }
    if !lookupState.is_null() {
        match (*lookupState).nbDirs {
            0 => {
                (*lookupState).Dir = OpenFileByDirentry(direntry);
                (*lookupState).nbDirs += 1;
                (*lookupState).nbDirs;
                free_stream(&mut (*lookupState).container);
                return 0 as i32;
            }
            1 => {
                free_stream(&mut (*lookupState).Dir);
                fprintf(stderr, b"Ambiguous\n\0" as *const u8 as *const i8);
                return 32 as i32 | 16 as i32;
            }
            _ => return 32 as i32 | 16 as i32,
        }
    }
    (*mp).direntry = direntry;
    if (*direntry).dir.attr as i32 & 0x10 as i32 != 0 {
        if (*mp).lookupflags & (1 as i32 | 0x400 as i32) != 0 {
            (*mp).File = OpenFileByDirentry(direntry);
            MyFile = (*mp).File;
        }
        ret = ((*mp).dirCallback).expect("non-null function pointer")(direntry, mp);
    } else {
        if (*mp).lookupflags & 1 as i32 != 0 {
            if !DeferredFileP.is_null() && !(*DeferredFileP).is_null() {
                free_stream(DeferredFileP);
                fprintf(
                    stderr,
                    b"Attempt to copy multiple files to non-directory\n\0" as *const u8
                        as *const i8,
                );
                return 32 as i32 | 16 as i32;
            }
            (*mp).File = OpenFileByDirentry(direntry);
            MyFile = (*mp).File;
            if !DeferredFileP.is_null() {
                *DeferredFileP = MyFile;
                return 0 as i32;
            }
        }
        ret = ((*mp).callback).expect("non-null function pointer")(direntry, mp);
    }
    free_stream(&mut MyFile);
    return ret;
}
unsafe extern "C" fn mt_dos_loop(
    mut Dir: *mut Stream_t,
    mut mp: *mut MainParam_t,
    mut filename: *const i8,
) -> i32 {
    let mut MyFile: *mut Stream_t = 0 as *mut Stream_t;
    let mut entry: direntry_t = direntry_t {
        Dir: 0 as *mut Stream_t,
        entry: 0,
        dir: directory {
            name: [0; 8],
            ext: [0; 3],
            attr: 0,
            Case: 0,
            ctime_ms: 0,
            ctime: [0; 2],
            cdate: [0; 2],
            adate: [0; 2],
            startHi: [0; 2],
            time: [0; 2],
            date: [0; 2],
            start: [0; 2],
            size: [0; 4],
        },
        name: [0; 256],
        beginSlot: 0,
        endSlot: 0,
    };
    let mut ret: i32 = 0;
    let mut r: i32 = 0;
    ret = 0 as i32;
    r = 0 as i32;
    initializeDirentry(&mut entry, Dir);
    while got_signal == 0
        && {
            r = vfat_lookup_zt(
                &mut entry,
                filename,
                (*mp).lookupflags,
                (*mp).shortname.data,
                (*mp).shortname.len,
                (*mp).longname.data,
                (*mp).longname.len,
            );
            r == 0 as i32
        }
    {
        (*mp).File = 0 as *mut Stream_t;
        if checkForDot((*mp).lookupflags, (entry.name).as_mut_ptr()) == 0 {
            MyFile = 0 as *mut Stream_t;
            if (*mp).lookupflags & 1 as i32 != 0
                || entry.dir.attr as i32 & 0x10 as i32 != 0
                    && (*mp).lookupflags & 0x400 as i32 != 0
            {
                (*mp).File = OpenFileByDirentry(&mut entry);
                MyFile = (*mp).File;
            }
            if got_signal != 0 {
                break;
            }
            (*mp).direntry = &mut entry;
            if entry.dir.attr as i32 & 0x10 as i32 != 0 {
                ret
                    |= ((*mp).dirCallback)
                        .expect("non-null function pointer")(&mut entry, mp);
            } else {
                ret
                    |= ((*mp).callback)
                        .expect("non-null function pointer")(&mut entry, mp);
            }
            free_stream(&mut MyFile);
        }
        if fat_error(Dir) != 0 {
            ret |= 16 as i32;
        }
        if (*mp).fast_quit != 0 && ret & 16 as i32 != 0 {
            break;
        }
    }
    if r == -(2 as i32) {
        return 16 as i32;
    }
    if got_signal != 0 {
        ret |= 16 as i32;
    }
    return ret;
}
unsafe extern "C" fn recurs_dos_loop(
    mut mp: *mut MainParam_t,
    mut filename0: *const i8,
    mut filename1: *const i8,
    mut lookupState: *mut lookupState_t,
    mut DeferredFileP: *mut *mut Stream_t,
) -> i32 {
    let mut ptr: *const i8 = 0 as *const i8;
    let mut entry: direntry_t = direntry_t {
        Dir: 0 as *mut Stream_t,
        entry: 0,
        dir: directory {
            name: [0; 8],
            ext: [0; 3],
            attr: 0,
            Case: 0,
            ctime_ms: 0,
            ctime: [0; 2],
            cdate: [0; 2],
            adate: [0; 2],
            startHi: [0; 2],
            time: [0; 2],
            date: [0; 2],
            start: [0; 2],
            size: [0; 4],
        },
        name: [0; 256],
        beginSlot: 0,
        endSlot: 0,
    };
    let mut length: size_t = 0;
    let mut lookupflags: i32 = 0;
    let mut ret: i32 = 0;
    let mut have_one: i32 = 0;
    let mut doing_mcwd: i32 = 0;
    let mut r: i32 = 0;
    loop {
        if strncmp(filename0, b"./\0" as *const u8 as *const i8, 2 as i32 as u64) == 0 {
            filename0 = filename0.offset(2 as i32 as isize);
        } else if strcmp(filename0, b".\0" as *const u8 as *const i8) == 0
            && !filename1.is_null()
        {
            filename0 = filename0.offset(1);
            filename0;
        } else if *filename0.offset(0 as i32 as isize) as i32 == '/' as i32 {
            filename0 = filename0.offset(1);
            filename0;
        } else {
            if !(*filename0.offset(0 as i32 as isize) == 0) {
                break;
            }
            if filename1.is_null() {
                break;
            }
            filename0 = filename1;
            filename1 = 0 as *const i8;
        }
    }
    if strncmp(filename0, b"../\0" as *const u8 as *const i8, 3 as i32 as u64) == 0
        || strcmp(filename0, b"..\0" as *const u8 as *const i8) == 0
            && !filename1.is_null()
    {
        (*mp).File = (*getDirentry((*mp).File)).Dir;
        return recurs_dos_loop(
            mp,
            filename0.offset(2 as i32 as isize),
            filename1,
            lookupState,
            DeferredFileP,
        );
    }
    doing_mcwd = !filename1.is_null() as i32;
    ptr = strchr(filename0, '/' as i32);
    if ptr.is_null() {
        length = strlen(filename0);
        ptr = filename1;
        filename1 = 0 as *const i8;
    } else {
        length = ptrdiff(ptr, filename0);
        ptr = ptr.offset(1);
        ptr;
    }
    if ptr.is_null() {
        if (*mp).lookupflags & 0x1000 as i32 != 0 {
            (*mp).targetName = filename0;
            ret = handle_leaf(
                getDirentry((*mp).File),
                mp,
                lookupState,
                0 as *mut *mut Stream_t,
            );
            (*mp).targetName = 0 as *const i8;
            return ret;
        }
        if strcmp(filename0, b".\0" as *const u8 as *const i8) == 0
            || *filename0.offset(0 as i32 as isize) == 0
        {
            return handle_leaf(
                getDirentry((*mp).File),
                mp,
                lookupState,
                0 as *mut *mut Stream_t,
            );
        }
        if strcmp(filename0, b"..\0" as *const u8 as *const i8) == 0 {
            return handle_leaf(
                getParent(getDirentry((*mp).File)),
                mp,
                lookupState,
                0 as *mut *mut Stream_t,
            );
        }
        lookupflags = (*mp).lookupflags;
        if !lookupState.is_null() {
            (*lookupState).filename = filename0;
            if (*lookupState).nbContainers + (*lookupState).nbDirs > 0 as i32 {
                free_stream(&mut (*lookupState).container);
            } else {
                (*lookupState).container = copy_stream((*mp).File);
            }
            (*lookupState).nbContainers += 1;
            (*lookupState).nbContainers;
        }
    } else {
        lookupflags = 0x10 as i32 | 1 as i32 | 0x100 as i32;
    }
    ret = 0 as i32;
    r = 0 as i32;
    have_one = 0 as i32;
    initializeDirentry(&mut entry, (*mp).File);
    while ret & 32 as i32 == 0 && got_signal == 0
        && {
            r = vfat_lookup(
                &mut entry,
                filename0,
                length,
                lookupflags | 0x80 as i32,
                (*mp).shortname.data,
                (*mp).shortname.len,
                (*mp).longname.data,
                (*mp).longname.len,
            );
            r == 0 as i32
        }
    {
        if checkForDot(lookupflags, (entry.name).as_mut_ptr()) != 0 {
            continue;
        }
        have_one = 1 as i32;
        if !ptr.is_null() {
            let mut SubDir: *mut Stream_t = 0 as *mut Stream_t;
            (*mp).File = OpenFileByDirentry(&mut entry);
            SubDir = (*mp).File;
            ret |= recurs_dos_loop(mp, ptr, filename1, lookupState, DeferredFileP);
            free_stream(&mut SubDir);
        } else {
            ret |= handle_leaf(&mut entry, mp, lookupState, DeferredFileP);
        }
        if doing_mcwd != 0 {
            break;
        }
    }
    if r == -(2 as i32) {
        return 16 as i32;
    }
    if got_signal != 0 {
        return ret | 16 as i32;
    }
    if doing_mcwd != 0 && have_one == 0 {
        return 8 as i32;
    }
    return ret;
}
unsafe extern "C" fn common_dos_loop(
    mut mp: *mut MainParam_t,
    mut pathname: *const i8,
    mut lookupState: *mut lookupState_t,
    mut open_mode: i32,
) -> i32 {
    let mut RootDir: *mut Stream_t = 0 as *mut Stream_t;
    let mut cwd: *const i8 = 0 as *const i8;
    let mut drive: i8 = 0;
    let mut DeferredFile: *mut Stream_t = 0 as *mut Stream_t;
    let mut DeferredFileP: *mut *mut Stream_t = 0 as *mut *mut Stream_t;
    let mut ret: i32 = 0;
    (*mp).loop_0 = Some(
        mt_dos_loop
            as unsafe extern "C" fn(*mut Stream_t, *mut MainParam_t, *const i8) -> i32,
    );
    drive = '\0' as i32 as i8;
    cwd = b"\0" as *const u8 as *const i8;
    if *pathname as i32 != 0 && *pathname.offset(1 as i32 as isize) as i32 == ':' as i32
    {
        drive = ch_toupper(*pathname);
        pathname = pathname.offset(2 as i32 as isize);
        if (*mp).mcwd[0 as i32 as usize] as i32 == drive as i32 {
            cwd = ((*mp).mcwd).as_mut_ptr().offset(2 as i32 as isize);
        }
    } else if (*mp).mcwd[0 as i32 as usize] != 0 {
        drive = (*mp).mcwd[0 as i32 as usize];
        cwd = ((*mp).mcwd).as_mut_ptr().offset(2 as i32 as isize);
    } else {
        drive = get_default_drive();
    }
    if *pathname as i32 == '/' as i32 {
        cwd = b"\0" as *const u8 as *const i8;
    }
    (*mp).File = open_root_dir(drive, open_mode, 0 as *mut i32);
    RootDir = (*mp).File;
    if ((*mp).File).is_null() {
        return 16 as i32;
    }
    if !((*mp).originalArg).is_null()
        && !(strpbrk((*mp).originalArg, b"*[?\0" as *const u8 as *const i8)).is_null()
        && (*mp).lookupflags & 0x2000 as i32 != 0
        && isUniqueTarget((*mp).targetName) != 0
    {
        DeferredFileP = &mut DeferredFile;
    }
    ret = recurs_dos_loop(mp, cwd, pathname, lookupState, DeferredFileP);
    if ret & 8 as i32 != 0 {
        *((*mp).mcwd).as_mut_ptr() = '\0' as i32 as i8;
        unlink_mcwd();
        ret = recurs_dos_loop(
            mp,
            b"\0" as *const u8 as *const i8,
            pathname,
            lookupState,
            DeferredFileP,
        );
    }
    if !DeferredFile.is_null() {
        (*mp).File = DeferredFile;
        ret = ((*mp).callback)
            .expect("non-null function pointer")(0 as *mut direntry_t, mp);
        free_stream(&mut DeferredFile);
    }
    free_stream(&mut RootDir);
    return ret;
}
unsafe extern "C" fn dos_loop(mut mp: *mut MainParam_t, mut arg: *const i8) -> i32 {
    return common_dos_loop(mp, arg, 0 as *mut lookupState_t, (*mp).openflags);
}
#[no_mangle]
pub unsafe extern "C" fn dos_target_lookup(
    mut mp: *mut MainParam_t,
    mut arg: *const i8,
) -> i32 {
    let mut lookupState: lookupState_t = lookupState_t {
        container: 0 as *mut Stream_t,
        nbContainers: 0,
        Dir: 0 as *mut Stream_t,
        nbDirs: 0,
        filename: 0 as *const i8,
    };
    let mut ret: i32 = 0;
    let mut lookupflags: i32 = 0;
    lookupState.nbDirs = 0 as i32;
    lookupState.Dir = 0 as *mut Stream_t;
    lookupState.nbContainers = 0 as i32;
    lookupState.container = 0 as *mut Stream_t;
    lookupflags = (*mp).lookupflags;
    (*mp).lookupflags = 1 as i32 | 0x10 as i32;
    ret = common_dos_loop(mp, arg, &mut lookupState, 0o2 as i32);
    (*mp).lookupflags = lookupflags;
    if ret & 16 as i32 != 0 {
        return ret;
    }
    if lookupState.nbDirs != 0 {
        (*mp).targetName = 0 as *const i8;
        (*mp).targetDir = lookupState.Dir;
        free_stream(&mut lookupState.container);
        return ret;
    }
    match lookupState.nbContainers {
        0 => {
            fprintf(
                stderr,
                b"%s: no match for target\n\0" as *const u8 as *const i8,
                arg,
            );
            return 2 as i32;
        }
        1 => {
            (*mp).targetName = strdup(lookupState.filename);
            (*mp).targetDir = lookupState.container;
            return ret;
        }
        _ => {
            fprintf(stderr, b"Ambiguous %s\n\0" as *const u8 as *const i8, arg);
            return 16 as i32;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn main_loop(
    mut mp: *mut MainParam_t,
    mut argv: *mut *mut i8,
    mut argc: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut ret: i32 = 0;
    let mut Bret: i32 = 0;
    Bret = 0 as i32;
    if argc != 1 as i32 && !((*mp).targetName).is_null() {
        fprintf(
            stderr,
            b"Several file names given, but last argument (%s) not a directory\n\0"
                as *const u8 as *const i8,
            (*mp).targetName,
        );
        free_stream(&mut (*mp).targetDir);
        return 1 as i32;
    }
    i = 0 as i32;
    while i < argc {
        if got_signal != 0 {
            break;
        }
        (*mp).originalArg = *argv.offset(i as isize);
        (*mp).basenameHasWildcard = (strpbrk(
            mt_basename((*mp).originalArg),
            b"*[?\0" as *const u8 as *const i8,
        ) != 0 as *mut i8) as i32;
        if ((*mp).unixcallback).is_some()
            && (*(*argv.offset(i as isize)).offset(0 as i32 as isize) == 0
                || *(*argv.offset(i as isize)).offset(1 as i32 as isize) as i32
                    != ':' as i32)
        {
            ret = unix_loop(0 as *mut Stream_t, mp, *argv.offset(i as isize), 1 as i32);
        } else {
            ret = dos_loop(mp, *argv.offset(i as isize));
        }
        if ret & (4 as i32 | 16 as i32) == 0 {
            fprintf(
                stderr,
                b"%s: File \"%s\" not found\n\0" as *const u8 as *const i8,
                progname,
                *argv.offset(i as isize),
            );
            ret |= 16 as i32;
        }
        Bret |= ret;
        if (*mp).fast_quit != 0 && Bret & (2 as i32 | 16 as i32) != 0 {
            break;
        }
        i += 1;
        i;
    }
    free_stream(&mut (*mp).targetDir);
    if Bret & 16 as i32 != 0 {
        return 1 as i32;
    }
    if Bret & 4 as i32 != 0 && Bret & 2 as i32 != 0 {
        return 2 as i32;
    }
    if Bret & 2 as i32 != 0 {
        return 1 as i32;
    }
    return 0 as i32;
}
unsafe extern "C" fn dispatchToFile(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> i32 {
    if !entry.is_null() {
        return ((*mp).callback).expect("non-null function pointer")(entry, mp)
    } else {
        return ((*mp).unixcallback).expect("non-null function pointer")(mp)
    };
}
#[no_mangle]
pub unsafe extern "C" fn init_mp(mut mp: *mut MainParam_t) {
    fix_mcwd(((*mp).mcwd).as_mut_ptr());
    (*mp).openflags = 0 as i32;
    (*mp).lookupflags = 0 as i32;
    (*mp).targetName = 0 as *const i8;
    (*mp).targetDir = 0 as *mut Stream_t;
    (*mp).dirCallback = Some(
        dispatchToFile as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
    );
    (*mp).unixcallback = None;
    (*mp).longname.data = 0 as *mut i8;
    (*mp).shortname.data = (*mp).longname.data;
    (*mp).longname.len = 0 as i32 as size_t;
    (*mp).shortname.len = (*mp).longname.len;
    (*mp).File = 0 as *mut Stream_t;
    (*mp).fast_quit = 0 as i32;
    (*mp).originalArg = 0 as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn mpGetBasename(mut mp: *mut MainParam_t) -> *const i8 {
    if !((*mp).direntry).is_null() {
        wchar_to_native(
            ((*(*mp).direntry).name).as_mut_ptr(),
            ((*mp).targetBuffer).as_mut_ptr(),
            (255 as i32 + 1 as i32) as size_t,
            ::core::mem::size_of::<[i8; 1021]>() as u64,
        );
        return ((*mp).targetBuffer).as_mut_ptr();
    } else {
        return mt_basename((*mp).unixSourceName)
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpPrintFilename(mut fp: *mut FILE, mut mp: *mut MainParam_t) {
    if !((*mp).direntry).is_null() {
        fprintPwd(fp, (*mp).direntry, 0 as i32);
    } else {
        fprintf(fp, b"%s\0" as *const u8 as *const i8, (*mp).originalArg);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpPickTargetName(mut mp: *mut MainParam_t) -> *const i8 {
    if !((*mp).targetName).is_null() {
        return (*mp).targetName
    } else {
        return mpGetBasename(mp)
    };
}
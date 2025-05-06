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
    pub type __dirstream;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    static mut got_signal: i32;
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn unix_loop(
        Stream: *mut Stream_t,
        mp: *mut MainParam_t,
        arg: *mut i8,
        follow_dir_link: i32,
    ) -> i32;
    fn isSpecial(name: *const i8) -> i32;
    fn closedir(__dirp: *mut DIR) -> i32;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn opendir(__name: *const i8) -> *mut DIR;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
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
pub type mt_off_t = off_t;
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
pub struct Dir_t {
    pub head: Stream_t,
    pub statbuf: stat,
    pub pathname: *mut i8,
    pub dir: *mut DIR,
}
pub type DIR = __dirstream;
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
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: u8,
    pub d_name: [i8; 256],
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
unsafe extern "C" fn get_dir_data(
    mut Stream: *mut Stream_t,
    mut date: *mut time_t,
    mut size: *mut mt_off_t,
    mut type_0: *mut i32,
    mut address: *mut u32,
) -> i32 {
    let mut This: *mut Dir_t = Stream as *mut Dir_t;
    if !date.is_null() {
        *date = (*This).statbuf.st_mtim.tv_sec;
    }
    if !size.is_null() {
        *size = (*This).statbuf.st_size;
    }
    if !type_0.is_null() {
        *type_0 = 1 as i32;
    }
    if !address.is_null() {
        *address = 0 as i32 as u32;
    }
    return 0 as i32;
}
unsafe extern "C" fn dir_free(mut Stream: *mut Stream_t) -> i32 {
    let mut This: *mut Dir_t = Stream as *mut Dir_t;
    free((*This).pathname as *mut libc::c_void);
    closedir((*This).dir);
    return 0 as i32;
}
static mut DirClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: None,
            write: None,
            pread: None,
            pwrite: None,
            flush: None,
            freeFunc: Some(dir_free as unsafe extern "C" fn(*mut Stream_t) -> i32),
            set_geom: None,
            get_data: Some(
                get_dir_data
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut time_t,
                        *mut mt_off_t,
                        *mut i32,
                        *mut u32,
                    ) -> i32,
            ),
            pre_allocate: None,
            get_dosConvert: None,
            discard: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn unix_dir_loop(
    mut Stream: *mut Stream_t,
    mut mp: *mut MainParam_t,
) -> i32 {
    let mut This: *mut Dir_t = Stream as *mut Dir_t;
    let mut entry: *mut dirent = 0 as *mut dirent;
    let mut newName: *mut i8 = 0 as *mut i8;
    let mut ret: i32 = 0 as i32;
    loop {
        entry = readdir((*This).dir);
        if entry.is_null() {
            break;
        }
        if got_signal != 0 {
            break;
        }
        if isSpecial(((*entry).d_name).as_mut_ptr()) != 0 {
            continue;
        }
        newName = malloc(
            (strlen((*This).pathname))
                .wrapping_add(1 as i32 as u64)
                .wrapping_add(strlen(((*entry).d_name).as_mut_ptr()))
                .wrapping_add(1 as i32 as u64),
        ) as *mut i8;
        if newName.is_null() {
            ret = 16 as i32;
            break;
        } else {
            strcpy(newName, (*This).pathname);
            strcat(newName, b"/\0" as *const u8 as *const i8);
            strcat(newName, ((*entry).d_name).as_mut_ptr());
            ret |= unix_loop(Stream, mp, newName, 0 as i32);
            free(newName as *mut libc::c_void);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn OpenDir(mut filename: *const i8) -> *mut Stream_t {
    let mut This: *mut Dir_t = 0 as *mut Dir_t;
    This = calloc(1 as i32 as u64, ::core::mem::size_of::<Dir_t>() as u64) as *mut Dir_t;
    init_head(&mut (*This).head, &mut DirClass, 0 as *mut Stream_t);
    (*This).pathname = malloc((strlen(filename)).wrapping_add(1 as i32 as u64))
        as *mut i8;
    if ((*This).pathname).is_null() {
        free(This as *mut i8 as *mut libc::c_void);
        return 0 as *mut Stream_t;
    }
    strcpy((*This).pathname, filename);
    if stat(filename, &mut (*This).statbuf) < 0 as i32 {
        free((*This).pathname as *mut libc::c_void);
        free(This as *mut i8 as *mut libc::c_void);
        return 0 as *mut Stream_t;
    }
    (*This).dir = opendir(filename);
    if ((*This).dir).is_null() {
        free((*This).pathname as *mut libc::c_void);
        free(This as *mut i8 as *mut libc::c_void);
        return 0 as *mut Stream_t;
    }
    return &mut (*This).head;
}
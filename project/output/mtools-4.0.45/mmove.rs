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
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type dos_name_t;
    pub type doscp_t;
    fn exit(_: i32) -> !;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn getopt(___argc: i32, ___argv: *const *mut i8, __shortopts: *const i8) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
    fn strpbrk(_: *const i8, _: *const i8) -> *mut i8;
    fn mt_basename(filename: *const i8) -> *const i8;
    fn copy_stream(Stream: *mut Stream_t) -> *mut Stream_t;
    fn free_stream(Stream: *mut *mut Stream_t) -> i32;
    fn set_cmd_line_image(img: *mut i8);
    fn helpFlag(_: i32, _: *mut *mut i8) -> i32;
    static mut mversion: *const i8;
    static mut mdate: *const i8;
    static mut progname: *const i8;
    fn isSubdirOf(inside: *mut Stream_t, outside: *mut Stream_t) -> i32;
    fn init_mp(MainParam: *mut MainParam_t);
    fn isRootEntry(entry: *mut direntry_t) -> i32;
    fn fprintPwd(f: *mut FILE, entry: *mut direntry_t, escape: i32);
    fn mpPickTargetName(mp: *mut MainParam_t) -> *const i8;
    fn dosnameToDirentry(n: *const dos_name_t, dir: *mut directory);
    fn initializeDirentry(entry: *mut direntry_t, Dir: *mut Stream_t);
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
    fn dir_write(entry: *mut direntry_t);
    fn wipeEntry(entry: *mut direntry_t);
    fn main_loop(MainParam: *mut MainParam_t, argv: *mut *mut i8, argc: i32) -> i32;
    fn dos_target_lookup(mp: *mut MainParam_t, arg: *const i8) -> i32;
    fn mwrite_one(
        Dir: *mut Stream_t,
        argname: *const i8,
        shortname: *const i8,
        cb: Option<write_data_callback>,
        arg: *mut libc::c_void,
        ch: *mut ClashHandling_t,
    ) -> i32;
    fn handle_clash_options(ch: *mut ClashHandling_t, c: i32) -> i32;
    fn init_clash_handling(ch: *mut ClashHandling_t);
    fn getDirentry(Stream: *mut Stream_t) -> *mut direntry_t;
    fn fat32RootCluster(Dir: *mut Stream_t) -> uint32_t;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __ssize_t = i64;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = u64;
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
pub struct Arg_t {
    pub fromname: *const i8,
    pub verbose: i32,
    pub mp: MainParam_t,
    pub entry: *mut direntry_t,
    pub ch: ClashHandling_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClashHandling_t {
    pub action: [clash_action; 2],
    pub namematch_default: [clash_action; 2],
    pub nowarn: i32,
    pub got_slots: i32,
    pub mod_time: i32,
    pub myname: *mut i8,
    pub dosname: *mut u8,
    pub single: i32,
    pub use_longname: i32,
    pub ignore_entry: i32,
    pub source: i32,
    pub source_entry: i32,
    pub name_converter: Option<
        unsafe extern "C" fn(
            *mut doscp_t,
            *const i8,
            i32,
            *mut i32,
            *mut dos_name_t,
        ) -> (),
    >,
    pub is_label: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum clash_action {
    NAMEMATCH_GREW = 9,
    NAMEMATCH_SUCCESS = 8,
    NAMEMATCH_ERROR = 7,
    NAMEMATCH_OVERWRITE = 6,
    NAMEMATCH_PRENAME = 5,
    NAMEMATCH_RENAME = 4,
    NAMEMATCH_SKIP = 3,
    NAMEMATCH_QUIT = 2,
    NAMEMATCH_AUTORENAME = 1,
    NAMEMATCH_NONE = 0,
}
impl clash_action {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            clash_action::NAMEMATCH_GREW => 9,
            clash_action::NAMEMATCH_SUCCESS => 8,
            clash_action::NAMEMATCH_ERROR => 7,
            clash_action::NAMEMATCH_OVERWRITE => 6,
            clash_action::NAMEMATCH_PRENAME => 5,
            clash_action::NAMEMATCH_RENAME => 4,
            clash_action::NAMEMATCH_SKIP => 3,
            clash_action::NAMEMATCH_QUIT => 2,
            clash_action::NAMEMATCH_AUTORENAME => 1,
            clash_action::NAMEMATCH_NONE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> clash_action {
        match value {
            9 => clash_action::NAMEMATCH_GREW,
            8 => clash_action::NAMEMATCH_SUCCESS,
            7 => clash_action::NAMEMATCH_ERROR,
            6 => clash_action::NAMEMATCH_OVERWRITE,
            5 => clash_action::NAMEMATCH_PRENAME,
            4 => clash_action::NAMEMATCH_RENAME,
            3 => clash_action::NAMEMATCH_SKIP,
            2 => clash_action::NAMEMATCH_QUIT,
            1 => clash_action::NAMEMATCH_AUTORENAME,
            0 => clash_action::NAMEMATCH_NONE,
            _ => panic!("Invalid value for clash_action: {}", value),
        }
    }
}
impl AddAssign<u32> for clash_action {
    fn add_assign(&mut self, rhs: u32) {
        *self = clash_action::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for clash_action {
    fn sub_assign(&mut self, rhs: u32) {
        *self = clash_action::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for clash_action {
    fn mul_assign(&mut self, rhs: u32) {
        *self = clash_action::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for clash_action {
    fn div_assign(&mut self, rhs: u32) {
        *self = clash_action::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for clash_action {
    fn rem_assign(&mut self, rhs: u32) {
        *self = clash_action::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for clash_action {
    type Output = clash_action;
    fn add(self, rhs: u32) -> clash_action {
        clash_action::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for clash_action {
    type Output = clash_action;
    fn sub(self, rhs: u32) -> clash_action {
        clash_action::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for clash_action {
    type Output = clash_action;
    fn mul(self, rhs: u32) -> clash_action {
        clash_action::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for clash_action {
    type Output = clash_action;
    fn div(self, rhs: u32) -> clash_action {
        clash_action::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for clash_action {
    type Output = clash_action;
    fn rem(self, rhs: u32) -> clash_action {
        clash_action::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type write_data_callback = unsafe extern "C" fn(
    *mut dos_name_t,
    *mut i8,
    *mut libc::c_void,
    *mut direntry_t,
) -> i32;
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
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
unsafe extern "C" fn renameit(
    mut dosname: *mut dos_name_t,
    mut longname: *mut i8,
    mut arg0: *mut libc::c_void,
    mut targetEntry: *mut direntry_t,
) -> i32 {
    let mut arg: *mut Arg_t = arg0 as *mut Arg_t;
    let mut fat: uint32_t = 0;
    (*targetEntry).dir = (*(*arg).entry).dir;
    dosnameToDirentry(dosname, &mut (*targetEntry).dir);
    if (*targetEntry).dir.attr as i32 & 0x10 as i32 != 0 {
        let mut movedEntry: *mut direntry_t = 0 as *mut direntry_t;
        movedEntry = getDirentry((*arg).mp.File);
        if (*movedEntry).Dir != (*targetEntry).Dir {
            let mut subEntry: direntry_t = direntry_t {
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
            let mut oldDir: *mut Stream_t = 0 as *mut Stream_t;
            initializeDirentry(&mut subEntry, (*arg).mp.File);
            match vfat_lookup(
                &mut subEntry,
                b"..\0" as *const u8 as *const i8,
                2 as i32 as size_t,
                0x10 as i32,
                0 as *mut i8,
                0 as i32 as size_t,
                0 as *mut i8,
                0 as i32 as size_t,
            ) {
                -1 => {
                    fprintf(
                        stderr,
                        b" Directory has no parent entry\n\0" as *const u8 as *const i8,
                    );
                }
                -2 => return 16 as i32,
                0 => {
                    ((*(*(*targetEntry).Dir).Class).get_data)
                        .expect(
                            "non-null function pointer",
                        )(
                        (*targetEntry).Dir,
                        0 as *mut time_t,
                        0 as *mut mt_off_t,
                        0 as *mut i32,
                        &mut fat,
                    );
                    if fat == fat32RootCluster((*targetEntry).Dir) {
                        fat = 0 as i32 as uint32_t;
                    }
                    subEntry.dir.start[1 as i32 as usize] = (fat >> 8 as i32
                        & 0xff as i32 as u32) as u8;
                    subEntry.dir.start[0 as i32 as usize] = (fat & 0xff as i32 as u32)
                        as u8;
                    dir_write(&mut subEntry);
                    if (*arg).verbose != 0 {
                        fprintf(
                            stderr,
                            b"Easy, isn't it? I wonder why DOS can't do this.\n\0"
                                as *const u8 as *const i8,
                        );
                    }
                }
                _ => {}
            }
            wipeEntry(movedEntry);
            oldDir = (*movedEntry).Dir;
            *movedEntry = *targetEntry;
            copy_stream((*targetEntry).Dir);
            free_stream(&mut oldDir);
            return 0 as i32;
        }
    }
    wipeEntry((*arg).mp.direntry);
    return 0 as i32;
}
unsafe extern "C" fn rename_file(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> i32 {
    let mut result: i32 = 0;
    let mut targetDir: *mut Stream_t = 0 as *mut Stream_t;
    let mut shortname: *mut i8 = 0 as *mut i8;
    let mut longname: *const i8 = 0 as *const i8;
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    (*arg).entry = entry;
    targetDir = (*mp).targetDir;
    if targetDir == (*entry).Dir {
        (*arg).ch.ignore_entry = -(1 as i32);
        (*arg).ch.source = (*entry).entry;
        (*arg).ch.source_entry = (*entry).entry;
    } else {
        (*arg).ch.ignore_entry = -(1 as i32);
        (*arg).ch.source = -(2 as i32);
    }
    longname = mpPickTargetName(mp);
    shortname = 0 as *mut i8;
    result = mwrite_one(
        targetDir,
        longname,
        shortname,
        Some(
            renameit
                as unsafe extern "C" fn(
                    *mut dos_name_t,
                    *mut i8,
                    *mut libc::c_void,
                    *mut direntry_t,
                ) -> i32,
        ),
        arg as *mut libc::c_void,
        &mut (*arg).ch,
    );
    if result == 1 as i32 { return 4 as i32 } else { return 16 as i32 };
}
unsafe extern "C" fn rename_directory(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> i32 {
    let mut ret: i32 = 0;
    if isSubdirOf((*mp).targetDir, (*mp).File) != 0 {
        fprintf(stderr, b"Cannot move directory \0" as *const u8 as *const i8);
        fprintPwd(stderr, entry, 0 as i32);
        fprintf(
            stderr,
            b" into one of its own subdirectories (\0" as *const u8 as *const i8,
        );
        fprintPwd(stderr, getDirentry((*mp).targetDir), 0 as i32);
        fprintf(stderr, b")\n\0" as *const u8 as *const i8);
        return 16 as i32;
    }
    if isRootEntry(entry) != 0 {
        fprintf(stderr, b"Cannot move a root directory: \0" as *const u8 as *const i8);
        fprintPwd(stderr, entry, 0 as i32);
        return 16 as i32;
    }
    ret = rename_file(entry, mp);
    if ret & 16 as i32 != 0 {
        return ret;
    }
    return ret;
}
unsafe extern "C" fn rename_oldsyntax(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> i32 {
    let mut result: i32 = 0;
    let mut targetDir: *mut Stream_t = 0 as *mut Stream_t;
    let mut shortname: *const i8 = 0 as *const i8;
    let mut longname: *const i8 = 0 as *const i8;
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    (*arg).entry = entry;
    targetDir = (*entry).Dir;
    (*arg).ch.ignore_entry = -(1 as i32);
    (*arg).ch.source = (*entry).entry;
    (*arg).ch.source_entry = (*entry).entry;
    longname = (*mp).targetName;
    shortname = 0 as *const i8;
    result = mwrite_one(
        targetDir,
        longname,
        shortname,
        Some(
            renameit
                as unsafe extern "C" fn(
                    *mut dos_name_t,
                    *mut i8,
                    *mut libc::c_void,
                    *mut direntry_t,
                ) -> i32,
        ),
        arg as *mut libc::c_void,
        &mut (*arg).ch,
    );
    if result == 1 as i32 { return 4 as i32 } else { return 16 as i32 };
}
unsafe extern "C" fn usage(mut ret: i32) -> ! {
    fprintf(
        stderr,
        b"Mtools version %s, dated %s\n\0" as *const u8 as *const i8,
        mversion,
        mdate,
    );
    fprintf(
        stderr,
        b"Usage: %s [-vV] [-D clash_option] file targetfile\n\0" as *const u8
            as *const i8,
        progname,
    );
    fprintf(
        stderr,
        b"       %s [-vV] [-D clash_option] file [files...] target_directory\n\0"
            as *const u8 as *const i8,
        progname,
    );
    exit(ret);
}
#[no_mangle]
pub unsafe extern "C" fn mmove(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut oldsyntax: i32,
) {
    let mut arg: Arg_t = Arg_t {
        fromname: 0 as *const i8,
        verbose: 0,
        mp: MainParam_t {
            loop_0: None,
            dirCallback: None,
            callback: None,
            unixcallback: None,
            arg: 0 as *mut libc::c_void,
            openflags: 0,
            lookupflags: 0,
            fast_quit: 0,
            shortname: bounded_string {
                data: 0 as *mut i8,
                len: 0,
            },
            longname: bounded_string {
                data: 0 as *mut i8,
                len: 0,
            },
            File: 0 as *mut Stream_t,
            direntry: 0 as *mut direntry_t,
            unixSourceName: 0 as *mut i8,
            targetDir: 0 as *mut Stream_t,
            targetName: 0 as *const i8,
            originalArg: 0 as *mut i8,
            basenameHasWildcard: 0,
            mcwd: [0; 132],
            targetBuffer: [0; 1021],
        },
        entry: 0 as *mut direntry_t,
        ch: ClashHandling_t {
            action: [clash_action::NAMEMATCH_NONE; 2],
            namematch_default: [clash_action::NAMEMATCH_NONE; 2],
            nowarn: 0,
            got_slots: 0,
            mod_time: 0,
            myname: 0 as *mut i8,
            dosname: 0 as *mut u8,
            single: 0,
            use_longname: 0,
            ignore_entry: 0,
            source: 0,
            source_entry: 0,
            name_converter: None,
            is_label: 0,
        },
    };
    let mut c: i32 = 0;
    let mut shortname: [i8; 49] = [0; 49];
    let mut longname: [i8; 1021] = [0; 1021];
    let mut def_drive: i8 = 0;
    let mut i: i32 = 0;
    init_clash_handling(&mut arg.ch);
    arg.verbose = 0 as i32;
    if helpFlag(argc, argv) != 0 {
        usage(0 as i32);
    }
    loop {
        c = getopt(argc, argv, b"i:vD:oh\0" as *const u8 as *const i8);
        if !(c != -(1 as i32)) {
            break;
        }
        match c {
            105 => {
                set_cmd_line_image(optarg);
            }
            118 => {
                arg.verbose = 1 as i32;
            }
            111 => {
                handle_clash_options(&mut arg.ch, c);
            }
            68 => {
                if handle_clash_options(&mut arg.ch, *optarg as i32) != 0 {
                    usage(1 as i32);
                }
            }
            104 => {
                usage(0 as i32);
            }
            63 => {
                usage(1 as i32);
            }
            _ => {}
        }
    }
    if argc - optind < 2 as i32 {
        usage(1 as i32);
    }
    init_mp(&mut arg.mp);
    arg.mp.arg = &mut arg as *mut Arg_t as *mut libc::c_void;
    arg.mp.openflags = 0o2 as i32;
    def_drive = '\0' as i32 as i8;
    i = optind;
    while i < argc {
        if *(*argv.offset(i as isize)).offset(0 as i32 as isize) as i32 != 0
            && *(*argv.offset(i as isize)).offset(1 as i32 as isize) as i32 == ':' as i32
        {
            if def_drive == 0 {
                def_drive = ch_toupper(
                    *(*argv.offset(i as isize)).offset(0 as i32 as isize),
                );
            } else if def_drive as i32
                != ch_toupper(*(*argv.offset(i as isize)).offset(0 as i32 as isize))
                    as i32
            {
                fprintf(
                    stderr,
                    b"Cannot move files across different drives\n\0" as *const u8
                        as *const i8,
                );
                exit(1 as i32);
            }
        }
        i += 1;
        i;
    }
    if def_drive != 0 {
        *(arg.mp.mcwd).as_mut_ptr() = def_drive;
    }
    if oldsyntax != 0
        && (argc - optind != 2 as i32
            || !(strpbrk(
                b":/\0" as *const u8 as *const i8,
                *argv.offset((argc - 1 as i32) as isize),
            ))
                .is_null())
    {
        oldsyntax = 0 as i32;
    }
    arg.mp.lookupflags = 0x20 as i32 | 0x10 as i32 | 0x400 as i32 | 0x100 as i32;
    if oldsyntax == 0 {
        dos_target_lookup(&mut arg.mp, *argv.offset((argc - 1 as i32) as isize));
        arg.mp.callback = Some(
            rename_file as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
        );
        arg.mp.dirCallback = Some(
            rename_directory
                as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
        );
    } else {
        arg.fromname = *argv.offset(optind as isize);
        if *(arg.fromname).offset(0 as i32 as isize) as i32 != 0
            && *(arg.fromname).offset(1 as i32 as isize) as i32 == ':' as i32
        {
            arg.fromname = (arg.fromname).offset(2 as i32 as isize);
        }
        arg.fromname = mt_basename(arg.fromname);
        arg.mp.targetName = strdup(*argv.offset((argc - 1 as i32) as isize));
        arg.mp.callback = Some(
            rename_oldsyntax
                as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
        );
    }
    arg.mp.longname.data = longname.as_mut_ptr();
    arg.mp.longname.len = ::core::mem::size_of::<[i8; 1021]>() as u64;
    longname[0 as i32 as usize] = '\0' as i32 as i8;
    arg.mp.shortname.data = shortname.as_mut_ptr();
    arg.mp.shortname.len = ::core::mem::size_of::<[i8; 49]>() as u64;
    shortname[0 as i32 as usize] = '\0' as i32 as i8;
    exit(main_loop(&mut arg.mp, argv.offset(optind as isize), argc - optind - 1 as i32));
}
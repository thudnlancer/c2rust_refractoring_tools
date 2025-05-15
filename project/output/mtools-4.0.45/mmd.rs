use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type dos_name_t;
    pub type doscp_t;
    fn exit(_: i32) -> !;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn getopt(___argc: i32, ___argv: *const *mut i8, __shortopts: *const i8) -> i32;
    fn set_cmd_line_image(img: *mut i8);
    fn flush_stream(Stream: *mut Stream_t) -> i32;
    fn free_stream(Stream: *mut *mut Stream_t) -> i32;
    fn mk_entry(
        filename: *const dos_name_t,
        attr: u8,
        fat: u32,
        size: uint32_t,
        date: time_t,
        ndir: *mut directory,
    ) -> *mut directory;
    fn mk_entry_from_base(
        base: *const i8,
        attr: u8,
        fat: u32,
        size: uint32_t,
        date: time_t,
        ndir: *mut directory,
    ) -> *mut directory;
    fn getfreeMinClusters(Stream: *mut Stream_t, ref_0: uint32_t) -> i32;
    static mut progname: *const i8;
    fn getTimeNow(now: *mut time_t) -> time_t;
    static mut mversion: *const i8;
    fn helpFlag(_: i32, _: *mut *mut i8) -> i32;
    static mut mdate: *const i8;
    fn init_mp(MainParam: *mut MainParam_t);
    fn main_loop(MainParam: *mut MainParam_t, argv: *mut *mut i8, argc: i32) -> i32;
    fn initializeDirentry(entry: *mut direntry_t, Dir: *mut Stream_t);
    fn dir_write(entry: *mut direntry_t);
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
    fn OpenFileByDirentry(entry: *mut direntry_t) -> *mut Stream_t;
    fn fat32RootCluster(Dir: *mut Stream_t) -> uint32_t;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
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
    pub target: *mut i8,
    pub mp: MainParam_t,
    pub SrcDir: *mut Stream_t,
    pub entry: i32,
    pub ch: ClashHandling_t,
    pub targetDir: *mut Stream_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CreateArg_t {
    pub Dir: *mut Stream_t,
    pub NewDir: *mut Stream_t,
    pub attr: u8,
    pub mtime: time_t,
}
pub type write_data_callback = unsafe extern "C" fn(
    *mut dos_name_t,
    *mut i8,
    *mut libc::c_void,
    *mut direntry_t,
) -> i32;
unsafe extern "C" fn makeit(
    mut dosname: *mut dos_name_t,
    mut longname: *mut i8,
    mut arg0: *mut libc::c_void,
    mut targetEntry: *mut direntry_t,
) -> i32 {
    let mut Target: *mut Stream_t = 0 as *mut Stream_t;
    let mut arg: *mut CreateArg_t = arg0 as *mut CreateArg_t;
    let mut fat: uint32_t = 0;
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
    if getfreeMinClusters((*targetEntry).Dir, 1 as i32 as uint32_t) == 0 {
        return -(1 as i32);
    }
    mk_entry(
        dosname,
        0x10 as i32 as u8,
        1 as i32 as u32,
        0 as i32 as uint32_t,
        (*arg).mtime,
        &mut (*targetEntry).dir,
    );
    Target = OpenFileByDirentry(targetEntry);
    if Target.is_null() {
        fprintf(stderr, b"Could not open Target\n\0" as *const u8 as *const i8);
        return -(1 as i32);
    }
    initializeDirentry(&mut subEntry, Target);
    subEntry.entry = 1 as i32;
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
    mk_entry_from_base(
        b"..      \0" as *const u8 as *const i8,
        0x10 as i32 as u8,
        fat,
        0 as i32 as uint32_t,
        (*arg).mtime,
        &mut subEntry.dir,
    );
    dir_write(&mut subEntry);
    flush_stream(Target);
    subEntry.entry = 0 as i32;
    ((*(*Target).Class).get_data)
        .expect(
            "non-null function pointer",
        )(Target, 0 as *mut time_t, 0 as *mut mt_off_t, 0 as *mut i32, &mut fat);
    mk_entry_from_base(
        b".       \0" as *const u8 as *const i8,
        0x10 as i32 as u8,
        fat,
        0 as i32 as uint32_t,
        (*arg).mtime,
        &mut subEntry.dir,
    );
    dir_write(&mut subEntry);
    mk_entry(
        dosname,
        (0x10 as i32 | (*arg).attr as i32) as u8,
        fat,
        0 as i32 as uint32_t,
        (*arg).mtime,
        &mut (*targetEntry).dir,
    );
    (*arg).NewDir = Target;
    return 0 as i32;
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
        b"Usage: %s [-D clash_option] file targetfile\n\0" as *const u8 as *const i8,
        progname,
    );
    fprintf(
        stderr,
        b"       %s [-D clash_option] file [files...] target_directory\n\0" as *const u8
            as *const i8,
        progname,
    );
    exit(ret);
}
#[no_mangle]
pub unsafe extern "C" fn createDir(
    mut Dir: *mut Stream_t,
    mut filename: *const i8,
    mut ch: *mut ClashHandling_t,
    mut attr: u8,
    mut mtime: time_t,
) -> *mut Stream_t {
    let mut arg: CreateArg_t = CreateArg_t {
        Dir: 0 as *mut Stream_t,
        NewDir: 0 as *mut Stream_t,
        attr: 0,
        mtime: 0,
    };
    let mut ret: i32 = 0;
    arg.Dir = Dir;
    arg.attr = attr;
    arg.mtime = mtime;
    if getfreeMinClusters(Dir, 1 as i32 as uint32_t) == 0 {
        return 0 as *mut Stream_t;
    }
    ret = mwrite_one(
        Dir,
        filename,
        0 as *const i8,
        Some(
            makeit
                as unsafe extern "C" fn(
                    *mut dos_name_t,
                    *mut i8,
                    *mut libc::c_void,
                    *mut direntry_t,
                ) -> i32,
        ),
        &mut arg as *mut CreateArg_t as *mut libc::c_void,
        ch,
    );
    if ret < 1 as i32 { return 0 as *mut Stream_t } else { return arg.NewDir };
}
unsafe extern "C" fn createDirCallback(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> i32 {
    let mut ret: *mut Stream_t = 0 as *mut Stream_t;
    let mut now: time_t = 0;
    ret = createDir(
        (*mp).File,
        (*mp).targetName,
        &mut (*((*mp).arg as *mut Arg_t)).ch,
        0x10 as i32 as u8,
        getTimeNow(&mut now),
    );
    if ret.is_null() {
        return 16 as i32
    } else {
        free_stream(&mut ret);
        return 4 as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mmd(mut argc: i32, mut argv: *mut *mut i8, mut type_0: i32) {
    let mut arg: Arg_t = Arg_t {
        target: 0 as *mut i8,
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
        SrcDir: 0 as *mut Stream_t,
        entry: 0,
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
        targetDir: 0 as *mut Stream_t,
    };
    let mut c: i32 = 0;
    init_clash_handling(&mut arg.ch);
    if helpFlag(argc, argv) != 0 {
        usage(0 as i32);
    }
    loop {
        c = getopt(argc, argv, b"i:D:oh\0" as *const u8 as *const i8);
        if !(c != -(1 as i32)) {
            break;
        }
        match c {
            105 => {
                set_cmd_line_image(optarg);
            }
            63 => {
                usage(1 as i32);
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
            _ => {
                usage(1 as i32);
            }
        }
    }
    if argc - optind < 1 as i32 {
        usage(1 as i32);
    }
    init_mp(&mut arg.mp);
    arg.mp.arg = &mut arg as *mut Arg_t as *mut libc::c_void;
    arg.mp.openflags = 0o2 as i32;
    arg.mp.callback = Some(
        createDirCallback
            as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
    );
    arg.mp.lookupflags = 0x1000 as i32 | 0x400 as i32;
    exit(main_loop(&mut arg.mp, argv.offset(optind as isize), argc - optind));
}
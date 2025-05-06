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
    fn exit(_: i32) -> !;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn getopt(___argc: i32, ___argv: *const *mut i8, __shortopts: *const i8) -> i32;
    fn countBlocks(Dir: *mut Stream_t, block: u32) -> u32;
    fn set_cmd_line_image(img: *mut i8);
    fn getStart(Dir: *mut Stream_t, dir: *mut directory) -> u32;
    fn isRootDir(Stream: *mut Stream_t) -> i32;
    static mut progname: *const i8;
    fn helpFlag(_: i32, _: *mut *mut i8) -> i32;
    static mut mversion: *const i8;
    static mut mdate: *const i8;
    fn fprintPwd(f: *mut FILE, entry: *mut direntry_t, escape: i32);
    fn init_mp(MainParam: *mut MainParam_t);
    fn main_loop(MainParam: *mut MainParam_t, argv: *mut *mut i8, argc: i32) -> i32;
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
    pub all: i32,
    pub inDir: i32,
    pub summary: i32,
    pub parent: *mut Arg_t,
    pub target: *mut i8,
    pub path: *mut i8,
    pub blocks: u32,
    pub mp: MainParam_t,
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
        b"Usage: %s: msdosdirectory\n\0" as *const u8 as *const i8,
        progname,
    );
    exit(ret);
}
unsafe extern "C" fn file_mdu(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> i32 {
    let mut blocks: u32 = 0;
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    blocks = countBlocks((*entry).Dir, getStart((*entry).Dir, &mut (*entry).dir));
    if (*arg).all != 0 || (*arg).inDir == 0 {
        fprintPwd(stdout, entry, 0 as i32);
        printf(b" %d\n\0" as *const u8 as *const i8, blocks);
    }
    (*arg).blocks = ((*arg).blocks).wrapping_add(blocks);
    return 4 as i32;
}
unsafe extern "C" fn dir_mdu(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> i32 {
    let mut parentArg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    let mut arg: Arg_t = Arg_t {
        all: 0,
        inDir: 0,
        summary: 0,
        parent: 0 as *mut Arg_t,
        target: 0 as *mut i8,
        path: 0 as *mut i8,
        blocks: 0,
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
    };
    let mut ret: i32 = 0;
    arg = *parentArg;
    arg.mp.arg = &mut arg as *mut Arg_t as *mut libc::c_void;
    arg.parent = parentArg;
    arg.inDir = 1 as i32;
    if isRootDir((*entry).Dir) == 0 {
        arg.blocks = countBlocks(
            (*entry).Dir,
            getStart((*entry).Dir, &mut (*entry).dir),
        );
    } else {
        arg.blocks = 0 as i32 as u32;
    }
    ret = ((*mp).loop_0)
        .expect(
            "non-null function pointer",
        )((*mp).File, &mut arg.mp, b"*\0" as *const u8 as *const i8);
    if arg.summary == 0 || (*parentArg).inDir == 0 {
        fprintPwd(stdout, entry, 0 as i32);
        printf(b" %d\n\0" as *const u8 as *const i8, arg.blocks);
    }
    (*arg.parent).blocks = ((*arg.parent).blocks).wrapping_add(arg.blocks);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mdu(mut argc: i32, mut argv: *mut *mut i8, mut type_0: i32) {
    let mut arg: Arg_t = Arg_t {
        all: 0,
        inDir: 0,
        summary: 0,
        parent: 0 as *mut Arg_t,
        target: 0 as *mut i8,
        path: 0 as *mut i8,
        blocks: 0,
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
    };
    let mut c: i32 = 0;
    arg.all = 0 as i32;
    arg.inDir = 0 as i32;
    arg.summary = 0 as i32;
    if helpFlag(argc, argv) != 0 {
        usage(0 as i32);
    }
    loop {
        c = getopt(argc, argv, b"i:ash\0" as *const u8 as *const i8);
        if !(c != -(1 as i32)) {
            break;
        }
        match c {
            105 => {
                set_cmd_line_image(optarg);
            }
            97 => {
                arg.all = 1 as i32;
            }
            115 => {
                arg.summary = 1 as i32;
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
    if optind >= argc {
        usage(1 as i32);
    }
    if arg.summary != 0 && arg.all != 0 {
        fprintf(
            stderr,
            b"-a and -s options are mutually exclusive\n\0" as *const u8 as *const i8,
        );
        usage(1 as i32);
    }
    init_mp(&mut arg.mp);
    arg.mp.callback = Some(
        file_mdu as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
    );
    arg.mp.openflags = 0 as i32;
    arg.mp.dirCallback = Some(
        dir_mdu as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
    );
    arg.mp.arg = &mut arg as *mut Arg_t as *mut libc::c_void;
    arg.mp.lookupflags = 0x20 as i32 | 0x10 as i32 | 0x400 as i32 | 0x100 as i32;
    exit(main_loop(&mut arg.mp, argv.offset(optind as isize), argc - optind));
}
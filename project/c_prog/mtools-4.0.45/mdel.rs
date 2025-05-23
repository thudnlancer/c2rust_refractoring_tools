use ::libc;
extern "C" {
    pub type doscp_t;
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn ask_confirmation(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn free_stream(Stream: *mut *mut Stream_t) -> libc::c_int;
    static mut got_signal: libc::c_int;
    fn set_cmd_line_image(img: *mut libc::c_char);
    static mut progname: *const libc::c_char;
    static mut mversion: *const libc::c_char;
    fn helpFlag(_: libc::c_int, _: *mut *mut libc::c_char) -> libc::c_int;
    static mut mdate: *const libc::c_char;
    fn isRootEntry(entry: *mut direntry_t) -> libc::c_int;
    fn initializeDirentry(entry: *mut direntry_t, Dir: *mut Stream_t);
    fn dir_read(entry: *mut direntry_t, error: *mut libc::c_int) -> *mut directory;
    fn init_mp(MainParam: *mut MainParam_t);
    fn main_loop(
        MainParam: *mut MainParam_t,
        argv: *mut *mut libc::c_char,
        argc: libc::c_int,
    ) -> libc::c_int;
    fn vfat_lookup(
        entry: *mut direntry_t,
        filename: *const libc::c_char,
        length: size_t,
        flags: libc::c_int,
        shortname: *mut libc::c_char,
        shortname_len: size_t,
        longname: *mut libc::c_char,
        longname_len: size_t,
    ) -> libc::c_int;
    fn fatFreeWithDirentry(entry: *mut direntry_t) -> libc::c_int;
    fn setEntryToPos(entry: *mut direntry_t, pos: libc::c_uint);
    fn fprintPwd(f: *mut FILE, entry: *mut direntry_t, escape: libc::c_int);
    fn dir_write(entry: *mut direntry_t);
    fn OpenFileByDirentry(entry: *mut direntry_t) -> *mut Stream_t;
    fn wchar_to_native(
        wchar: *const wchar_t,
        native: *mut libc::c_char,
        len: size_t,
        out_len: size_t,
    ) -> size_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type mt_off_t = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream_t {
    pub Class: *mut Class_t,
    pub refs: libc::c_int,
    pub Next: *mut Stream_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Class_t {
    pub read: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut libc::c_char, size_t) -> ssize_t,
    >,
    pub write: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut libc::c_char, size_t) -> ssize_t,
    >,
    pub pread: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut libc::c_char,
            mt_off_t,
            size_t,
        ) -> ssize_t,
    >,
    pub pwrite: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut libc::c_char,
            mt_off_t,
            size_t,
        ) -> ssize_t,
    >,
    pub flush: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
    pub freeFunc: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
    pub set_geom: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut device_t, *mut device_t) -> libc::c_int,
    >,
    pub get_data: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut time_t,
            *mut mt_off_t,
            *mut libc::c_int,
            *mut uint32_t,
        ) -> libc::c_int,
    >,
    pub pre_allocate: Option::<
        unsafe extern "C" fn(*mut Stream_t, mt_off_t) -> libc::c_int,
    >,
    pub get_dosConvert: Option::<unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t>,
    pub discard: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
}
pub type device_t = device;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device {
    pub name: *const libc::c_char,
    pub drive: libc::c_char,
    pub fat_bits: libc::c_int,
    pub mode: libc::c_int,
    pub tracks: libc::c_uint,
    pub heads: uint16_t,
    pub sectors: uint16_t,
    pub hidden: libc::c_uint,
    pub offset: mt_off_t,
    pub partition: libc::c_uint,
    pub misc_flags: libc::c_uint,
    pub ssize: uint8_t,
    pub use_2m: libc::c_uint,
    pub precmd: *mut libc::c_char,
    pub file_nr: libc::c_int,
    pub blocksize: libc::c_uint,
    pub codepage: libc::c_uint,
    pub data_map: *const libc::c_char,
    pub tot_sectors: uint32_t,
    pub sector_size: uint16_t,
    pub postcmd: *mut libc::c_char,
    pub cfg_filename: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct directory {
    pub name: [libc::c_char; 8],
    pub ext: [libc::c_char; 3],
    pub attr: libc::c_uchar,
    pub Case: libc::c_uchar,
    pub ctime_ms: libc::c_uchar,
    pub ctime: [libc::c_uchar; 2],
    pub cdate: [libc::c_uchar; 2],
    pub adate: [libc::c_uchar; 2],
    pub startHi: [libc::c_uchar; 2],
    pub time: [libc::c_uchar; 2],
    pub date: [libc::c_uchar; 2],
    pub start: [libc::c_uchar; 2],
    pub size: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MainParam_t {
    pub loop_0: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut MainParam_t,
            *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub dirCallback: Option::<
        unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
    >,
    pub callback: Option::<
        unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
    >,
    pub unixcallback: Option::<unsafe extern "C" fn(*mut MainParam_t) -> libc::c_int>,
    pub arg: *mut libc::c_void,
    pub openflags: libc::c_int,
    pub lookupflags: libc::c_int,
    pub fast_quit: libc::c_int,
    pub shortname: bounded_string,
    pub longname: bounded_string,
    pub File: *mut Stream_t,
    pub direntry: *mut direntry_t,
    pub unixSourceName: *mut libc::c_char,
    pub targetDir: *mut Stream_t,
    pub targetName: *const libc::c_char,
    pub originalArg: *mut libc::c_char,
    pub basenameHasWildcard: libc::c_int,
    pub mcwd: [libc::c_char; 132],
    pub targetBuffer: [libc::c_char; 1021],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct direntry_t {
    pub Dir: *mut Stream_t,
    pub entry: libc::c_int,
    pub dir: directory,
    pub name: [wchar_t; 256],
    pub beginSlot: libc::c_uint,
    pub endSlot: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bounded_string {
    pub data: *mut libc::c_char,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Arg_t {
    pub deltype: libc::c_int,
    pub verbose: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn wipeEntry(mut entry: *mut direntry_t) {
    let mut longNameEntry: direntry_t = direntry_t {
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
    let mut i: libc::c_uint = 0;
    initializeDirentry(&mut longNameEntry, (*entry).Dir);
    i = (*entry).beginSlot;
    while i < (*entry).endSlot {
        let mut error: libc::c_int = 0;
        setEntryToPos(&mut longNameEntry, i);
        dir_read(&mut longNameEntry, &mut error);
        if error != 0 {
            break;
        }
        longNameEntry
            .dir
            .name[0 as libc::c_int as usize] = 0xe5 as libc::c_int as libc::c_char;
        dir_write(&mut longNameEntry);
        i = i.wrapping_add(1);
        i;
    }
    (*entry).dir.name[0 as libc::c_int as usize] = 0xe5 as libc::c_int as libc::c_char;
    dir_write(entry);
}
unsafe extern "C" fn del_entry(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    if got_signal != 0 {
        return 16 as libc::c_int;
    }
    if isRootEntry(entry) != 0 {
        fprintf(
            stderr,
            b"Cannot remove root directory\n\0" as *const u8 as *const libc::c_char,
        );
        return 16 as libc::c_int;
    }
    if (*arg).verbose != 0 {
        fprintf(stderr, b"Removing \0" as *const u8 as *const libc::c_char);
        fprintPwd(stderr, entry, 0 as libc::c_int);
        fputc('\n' as i32, stderr);
    }
    if (*entry).dir.attr as libc::c_int & (0x1 as libc::c_int | 0x4 as libc::c_int) != 0
    {
        let mut tmp: [libc::c_char; 1021] = [0; 1021];
        wchar_to_native(
            ((*entry).name).as_mut_ptr(),
            tmp.as_mut_ptr(),
            255 as libc::c_int as size_t,
            ::core::mem::size_of::<[libc::c_char; 1021]>() as libc::c_ulong,
        );
        if ask_confirmation(
            b"%s: \"%s\" is read only, erase anyway (y/n) ? \0" as *const u8
                as *const libc::c_char,
            progname,
            tmp.as_mut_ptr(),
        ) != 0
        {
            return 16 as libc::c_int;
        }
    }
    if fatFreeWithDirentry(entry) != 0 {
        return 16 as libc::c_int;
    }
    wipeEntry(entry);
    return 4 as libc::c_int;
}
unsafe extern "C" fn del_file(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> libc::c_int {
    let mut shortname: [libc::c_char; 13] = [0; 13];
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
    let mut SubDir: *mut Stream_t = 0 as *mut Stream_t;
    let mut arg: *mut Arg_t = (*mp).arg as *mut Arg_t;
    let mut sonmp: MainParam_t = MainParam_t {
        loop_0: None,
        dirCallback: None,
        callback: None,
        unixcallback: None,
        arg: 0 as *mut libc::c_void,
        openflags: 0,
        lookupflags: 0,
        fast_quit: 0,
        shortname: bounded_string {
            data: 0 as *mut libc::c_char,
            len: 0,
        },
        longname: bounded_string {
            data: 0 as *mut libc::c_char,
            len: 0,
        },
        File: 0 as *mut Stream_t,
        direntry: 0 as *mut direntry_t,
        unixSourceName: 0 as *mut libc::c_char,
        targetDir: 0 as *mut Stream_t,
        targetName: 0 as *const libc::c_char,
        originalArg: 0 as *mut libc::c_char,
        basenameHasWildcard: 0,
        mcwd: [0; 132],
        targetBuffer: [0; 1021],
    };
    let mut ret: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    sonmp = *mp;
    sonmp.arg = (*mp).arg;
    r = 0 as libc::c_int;
    if (*entry).dir.attr as libc::c_int & 0x10 as libc::c_int != 0 {
        SubDir = OpenFileByDirentry(entry);
        initializeDirentry(&mut subEntry, SubDir);
        ret = 0 as libc::c_int;
        loop {
            r = vfat_lookup(
                &mut subEntry,
                b"*\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
                0x10 as libc::c_int | 0x20 as libc::c_int,
                shortname.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong,
                0 as *mut libc::c_char,
                0 as libc::c_int as size_t,
            );
            if !(r == 0 as libc::c_int) {
                break;
            }
            if !(shortname[0 as libc::c_int as usize] as libc::c_int
                != 0xe5 as libc::c_int as libc::c_char as libc::c_int
                && shortname[0 as libc::c_int as usize] as libc::c_int != 0
                && shortname[0 as libc::c_int as usize] as libc::c_int != '.' as i32)
            {
                continue;
            }
            if (*arg).deltype != 2 as libc::c_int {
                fprintf(stderr, b"Directory \0" as *const u8 as *const libc::c_char);
                fprintPwd(stderr, entry, 0 as libc::c_int);
                fprintf(stderr, b" non empty\n\0" as *const u8 as *const libc::c_char);
                ret = 16 as libc::c_int;
                break;
            } else if got_signal != 0 {
                ret = 16 as libc::c_int;
                break;
            } else {
                ret = del_file(&mut subEntry, &mut sonmp);
                if ret & 16 as libc::c_int != 0 {
                    break;
                }
                ret = 0 as libc::c_int;
            }
        }
        free_stream(&mut SubDir);
        if r == -(2 as libc::c_int) {
            return 16 as libc::c_int;
        }
        if ret != 0 {
            return ret;
        }
    }
    return del_entry(entry, mp);
}
unsafe extern "C" fn usage(mut ret: libc::c_int) -> ! {
    fprintf(
        stderr,
        b"Mtools version %s, dated %s\n\0" as *const u8 as *const libc::c_char,
        mversion,
        mdate,
    );
    fprintf(
        stderr,
        b"Usage: %s [-v] msdosfile [msdosfiles...]\n\0" as *const u8
            as *const libc::c_char,
        progname,
    );
    exit(ret);
}
#[no_mangle]
pub unsafe extern "C" fn mdel(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut deltype: libc::c_int,
) {
    let mut arg: Arg_t = Arg_t { deltype: 0, verbose: 0 };
    let mut mp: MainParam_t = MainParam_t {
        loop_0: None,
        dirCallback: None,
        callback: None,
        unixcallback: None,
        arg: 0 as *mut libc::c_void,
        openflags: 0,
        lookupflags: 0,
        fast_quit: 0,
        shortname: bounded_string {
            data: 0 as *mut libc::c_char,
            len: 0,
        },
        longname: bounded_string {
            data: 0 as *mut libc::c_char,
            len: 0,
        },
        File: 0 as *mut Stream_t,
        direntry: 0 as *mut direntry_t,
        unixSourceName: 0 as *mut libc::c_char,
        targetDir: 0 as *mut Stream_t,
        targetName: 0 as *const libc::c_char,
        originalArg: 0 as *mut libc::c_char,
        basenameHasWildcard: 0,
        mcwd: [0; 132],
        targetBuffer: [0; 1021],
    };
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    arg.verbose = 0 as libc::c_int;
    if helpFlag(argc, argv) != 0 {
        usage(0 as libc::c_int);
    }
    loop {
        c = getopt(argc, argv, b"i:vh\0" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            105 => {
                set_cmd_line_image(optarg);
            }
            118 => {
                arg.verbose = 1 as libc::c_int;
            }
            104 => {
                usage(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    if argc == optind {
        usage(1 as libc::c_int);
    }
    init_mp(&mut mp);
    mp
        .callback = Some(
        del_file
            as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> libc::c_int,
    );
    mp.arg = &mut arg as *mut Arg_t as *mut libc::c_void;
    mp.openflags = 0o2 as libc::c_int;
    arg.deltype = deltype;
    match deltype {
        0 => {
            mp.lookupflags = 0x20 as libc::c_int;
        }
        1 => {
            mp.lookupflags = 0x10 as libc::c_int;
        }
        2 => {
            mp.lookupflags = 0x10 as libc::c_int | 0x20 as libc::c_int;
        }
        _ => {}
    }
    mp.lookupflags |= 0x100 as libc::c_int;
    i = optind;
    while i < argc {
        let mut b: size_t = 0;
        let mut l: size_t = 0;
        if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            != 0
            && *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int == ':' as i32
        {
            b = 2 as libc::c_int as size_t;
        } else {
            b = 0 as libc::c_int as size_t;
        }
        l = strlen((*argv.offset(i as isize)).offset(b as isize));
        if l > 1 as libc::c_int as libc::c_ulong
            && *(*argv.offset(i as isize))
                .offset(
                    b.wrapping_add(l).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as libc::c_int == '/' as i32
        {
            *(*argv.offset(i as isize))
                .offset(
                    b.wrapping_add(l).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as isize,
                ) = '\0' as i32 as libc::c_char;
        }
        i += 1;
        i;
    }
    exit(main_loop(&mut mp, argv.offset(optind as isize), argc - optind));
}

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
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn getopt(___argc: i32, ___argv: *const *mut i8, __shortopts: *const i8) -> i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn __errno_location() -> *mut i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strerror(_: i32) -> *mut i8;
    fn set_cmd_line_image(img: *mut i8);
    fn check_number_parse_errno(c: i8, optarg_0: *const i8, endptr: *mut i8);
    fn parseSize(sizeStr: *mut i8) -> uint32_t;
    fn strtoui(nptr: *const i8, endptr: *mut *mut i8, base: i32) -> u32;
    fn atoui(nptr: *const i8) -> u32;
    fn strtou8(nptr: *const i8, endptr: *mut *mut i8, base: i32) -> uint8_t;
    fn atou16(str: *const i8) -> uint16_t;
    static mut progname: *const i8;
    fn print_sector(message: *const i8, data: *mut u8, size: i32);
    fn helpFlag(_: i32, _: *mut *mut i8) -> i32;
    static mut devices: *mut device;
    fn free_stream(Stream: *mut *mut Stream_t) -> i32;
    fn expand(_: *const i8, _: *mut i8) -> *const i8;
    static mut mdate: *const i8;
    static mut mversion: *const i8;
    fn consistencyCheck(
        partTable: *mut partition,
        doprint: i32,
        verbose: i32,
        has_activated: *mut i32,
        tot_sectors: uint32_t,
        used_dev: *mut device,
        target_partition: u32,
    ) -> i32;
    fn findOverlap(
        partTable: *mut partition,
        until: u32,
        start: uint32_t,
        end: uint32_t,
    ) -> u32;
    fn OpenImage(
        out_dev: *mut device,
        dev: *mut device,
        name: *const i8,
        mode: i32,
        errmsg: *mut i8,
        flags: i32,
        lockMode: i32,
        maxSize: *mut mt_off_t,
        geomFailureP: *mut i32,
        xdf_info: *mut xdf_info,
    ) -> *mut Stream_t;
    fn compute_lba_geom_from_tot_sectors(dev: *mut device) -> i32;
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
pub struct partition {
    pub start: hsc,
    pub end: hsc,
    pub start_sect: [u8; 4],
    pub nr_sects: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hsc {
    pub byte0: u8,
    pub head: u8,
    pub sector: u8,
    pub cyl: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdf_info {
    pub FatSize: u32,
    pub RootDirSize: uint16_t,
    pub BadSectors: u32,
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
unsafe extern "C" fn set_dword(mut data: *mut u8, mut value: uint32_t) {
    *data.offset(3 as i32 as isize) = (value >> 24 as i32 & 0xff as i32 as u32) as u8;
    *data.offset(2 as i32 as isize) = (value >> 16 as i32 & 0xff as i32 as u32) as u8;
    *data.offset(1 as i32 as isize) = (value >> 8 as i32 & 0xff as i32 as u32) as u8;
    *data.offset(0 as i32 as isize) = (value >> 0 as i32 & 0xff as i32 as u32) as u8;
}
#[inline]
unsafe extern "C" fn set_word(mut data: *mut u8, mut value: libc::c_ushort) {
    *data.offset(1 as i32 as isize) = (value as i32 >> 8 as i32 & 0xff as i32) as u8;
    *data.offset(0 as i32 as isize) = (value as i32 >> 0 as i32 & 0xff as i32) as u8;
}
unsafe extern "C" fn set_offset(
    mut h: *mut hsc,
    mut offset: u64,
    mut heads: uint16_t,
    mut sectors: uint16_t,
) {
    let mut head: uint16_t = 0;
    let mut sector: uint16_t = 0;
    let mut cyl: u32 = 0;
    if heads == 0 || sectors == 0 {
        cyl = 0 as i32 as u32;
        sector = cyl as uint16_t;
        head = sector;
    } else {
        sector = offset.wrapping_rem(sectors as u64) as uint16_t;
        offset = offset.wrapping_div(sectors as u64);
        head = offset.wrapping_rem(heads as u64) as uint16_t;
        offset = offset.wrapping_div(heads as u64);
        if offset > 1023 as i32 as u64 {
            cyl = 1023 as i32 as u32;
        } else {
            cyl = offset as uint16_t as u32;
        }
    }
    if head as i32 > 255 as i32 {
        cyl = 0 as i32 as u32;
        sector = cyl as uint16_t;
        head = sector;
    }
    (*h).head = head as uint8_t;
    (*h).sector = ((sector as i32 + 1 as i32 & 0x3f as i32) as u32
        | (cyl & 0x300 as i32 as u32) >> 2 as i32) as u8;
    (*h).cyl = (cyl & 0xff as i32 as u32) as u8;
}
#[no_mangle]
pub unsafe extern "C" fn setBeginEnd(
    mut partTable: *mut partition,
    mut begin: uint32_t,
    mut end: uint32_t,
    mut iheads: uint16_t,
    mut isectors: uint16_t,
    mut activate: i32,
    mut type_0: uint8_t,
    mut fat_bits: u32,
) {
    let mut heads: uint8_t = 0;
    let mut sectors: uint8_t = 0;
    if iheads as i32 > 255 as i32 {
        fprintf(
            stderr,
            b"Too many heads for partition: %d\n\0" as *const u8 as *const i8,
            iheads as i32,
        );
        exit(1 as i32);
    }
    heads = iheads as uint8_t;
    if isectors as i32 > 255 as i32 {
        fprintf(
            stderr,
            b"Too many sectors for partition: %d\n\0" as *const u8 as *const i8,
            isectors as i32,
        );
        exit(1 as i32);
    }
    sectors = isectors as uint8_t;
    set_offset(
        &mut (*partTable).start,
        begin as u64,
        heads as uint16_t,
        sectors as uint16_t,
    );
    set_offset(
        &mut (*partTable).end,
        end.wrapping_sub(1 as i32 as u32) as u64,
        heads as uint16_t,
        sectors as uint16_t,
    );
    set_dword(((*partTable).start_sect).as_mut_ptr(), begin);
    set_dword(((*partTable).nr_sects).as_mut_ptr(), end.wrapping_sub(begin));
    if activate != 0 {
        (*partTable).start.byte0 = 0x80 as i32 as u8;
    } else {
        (*partTable).start.byte0 = 0 as i32 as u8;
    }
    if type_0 == 0 {
        if fat_bits == 0 as i32 as u32 {
            if end.wrapping_sub(begin) < 4096 as i32 as u32 {
                fat_bits = 12 as i32 as u32;
            } else {
                fat_bits = 16 as i32 as u32;
            }
        }
        if fat_bits == 32 as i32 as u32 {
            type_0 = 0xc as i32 as uint8_t;
        } else if end < 65536 as i32 as u32 {
            if fat_bits == 12 as i32 as u32 {
                type_0 = 0x1 as i32 as uint8_t;
            } else if fat_bits == 16 as i32 as u32 {
                type_0 = 0x4 as i32 as uint8_t;
            }
        } else if end
            < ((sectors as i32 * heads as i32) as u32).wrapping_mul(1024 as u32)
        {
            type_0 = 0x6 as i32 as uint8_t;
        } else {
            type_0 = 0xe as i32 as uint8_t;
        }
    }
    (*partTable).end.byte0 = type_0;
}
unsafe extern "C" fn setsize(
    mut capacity: u64,
    mut cyls: *mut u32,
    mut hds: *mut uint16_t,
    mut secs: *mut uint16_t,
) -> i32 {
    let mut rv: i32 = 0 as i32;
    let mut heads: u64 = 0;
    let mut sectors: u64 = 0;
    let mut cylinders: u64 = 0;
    let mut temp: u64 = 0;
    cylinders = 1024 as i64 as u64;
    sectors = 62 as i64 as u64;
    temp = cylinders.wrapping_mul(sectors);
    heads = capacity.wrapping_div(temp);
    if capacity.wrapping_rem(temp) != 0 {
        heads = heads.wrapping_add(1);
        heads;
        temp = cylinders.wrapping_mul(heads);
        sectors = capacity.wrapping_div(temp);
        if capacity.wrapping_rem(temp) != 0 {
            sectors = sectors.wrapping_add(1);
            sectors;
            temp = heads.wrapping_mul(sectors);
            cylinders = capacity.wrapping_div(temp);
        }
    }
    if cylinders == 0 as i32 as u64 {
        rv = -(1 as i32);
    }
    *cyls = cylinders as u32;
    *secs = sectors as uint16_t;
    *hds = heads as uint16_t;
    return rv;
}
unsafe extern "C" fn setsize0(
    mut capacity: uint32_t,
    mut cyls: *mut u32,
    mut hds: *mut uint16_t,
    mut secs: *mut uint16_t,
) {
    let mut r: i32 = 0;
    if capacity < (1024 as i32 * 2048 as i32) as u32
        && capacity.wrapping_rem(1024 as i32 as u32) == 0
    {
        *cyls = capacity >> 11 as i32;
        *hds = 64 as i32 as uint16_t;
        *secs = 32 as i32 as uint16_t;
        return;
    }
    r = setsize(capacity as u64, cyls, hds, secs);
    if r != 0 || *hds as i32 > 255 as i32 || *secs as i32 > 63 as i32 {
        *cyls = capacity >> 11 as i32;
        *hds = 64 as i32 as uint16_t;
        *secs = 32 as i32 as uint16_t;
        return;
    }
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
        b"Usage: %s [-pradcv] [-I] [-B bootsect-template] [-s sectors] [-t cylinders] [-h heads] [-T type] [-b begin] [-l length] drive\n\0"
            as *const u8 as *const i8,
        progname,
    );
    exit(ret);
}
#[no_mangle]
pub unsafe extern "C" fn mpartition(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut dummy: i32,
) {
    let mut Stream: *mut Stream_t = 0 as *mut Stream_t;
    let mut dummy2: u32 = 0;
    let mut i: u32 = 0;
    let mut sec_per_cyl: uint16_t = 0;
    let mut doprint: i32 = 0 as i32;
    let mut verbose: i32 = 0 as i32;
    let mut create: i32 = 0 as i32;
    let mut force: i32 = 0 as i32;
    let mut length: u32 = 0 as i32 as u32;
    let mut do_remove: i32 = 0 as i32;
    let mut initialize: i32 = 0 as i32;
    let mut tot_sectors: uint32_t = 0 as i32 as uint32_t;
    let mut type_0: uint8_t = 0 as i32 as uint8_t;
    let mut begin_set: i32 = 0 as i32;
    let mut size_set: i32 = 0 as i32;
    let mut end_set: i32 = 0 as i32;
    let mut activate: i32 = 0 as i32;
    let mut has_activated: i32 = 0 as i32;
    let mut inconsistency: i32 = 0 as i32;
    let mut begin: u32 = 0 as i32 as u32;
    let mut end: u32 = 0 as i32 as u32;
    let mut dirty: i32 = 0 as i32;
    let mut open2flags: i32 = 0 as i32;
    let mut c: i32 = 0;
    let mut used_dev: device = device {
        name: 0 as *const i8,
        drive: 0,
        fat_bits: 0,
        mode: 0,
        tracks: 0,
        heads: 0,
        sectors: 0,
        hidden: 0,
        offset: 0,
        partition: 0,
        misc_flags: 0,
        ssize: 0,
        use_2m: 0,
        precmd: 0 as *mut i8,
        file_nr: 0,
        blocksize: 0,
        codepage: 0,
        data_map: 0 as *const i8,
        tot_sectors: 0,
        sector_size: 0,
        postcmd: 0 as *mut i8,
        cfg_filename: 0 as *const i8,
    };
    let mut argtracks: u32 = 0;
    let mut argheads: uint16_t = 0;
    let mut argsectors: uint16_t = 0;
    let mut drive: i8 = 0;
    let mut name: [i8; 2048] = [0; 2048];
    let mut buf: [u8; 512] = [0; 512];
    let mut partTable: *mut partition = buf.as_mut_ptr().offset(0x1ae as i32 as isize)
        as *mut partition;
    let mut dev: *mut device = 0 as *mut device;
    let mut errmsg: [i8; 2100] = [0; 2100];
    let mut bootSector: *mut i8 = 0 as *mut i8;
    let mut tpartition: *mut partition = 0 as *mut partition;
    argtracks = 0 as i32 as u32;
    argheads = 0 as i32 as uint16_t;
    argsectors = 0 as i32 as uint16_t;
    if helpFlag(argc, argv) != 0 {
        usage(0 as i32);
    }
    loop {
        c = getopt(
            argc,
            argv,
            b"i:adprcIT:t:h:s:fvpb:l:S:B:\0" as *const u8 as *const i8,
        );
        if !(c != -(1 as i32)) {
            break;
        }
        let mut endptr: *mut i8 = 0 as *mut i8;
        *__errno_location() = 0 as i32;
        match c {
            105 => {
                set_cmd_line_image(optarg);
            }
            66 => {
                bootSector = optarg;
            }
            97 => {
                open2flags |= 1 as i32;
                activate = 1 as i32;
                dirty = 1 as i32;
            }
            100 => {
                activate = -(1 as i32);
                dirty = 1 as i32;
            }
            112 => {
                doprint = 1 as i32;
            }
            114 => {
                do_remove = 1 as i32;
                dirty = 1 as i32;
            }
            73 => {
                open2flags |= 1 as i32;
                initialize = 1 as i32;
                dirty = 1 as i32;
            }
            99 => {
                create = 1 as i32;
                dirty = 1 as i32;
            }
            84 => {
                open2flags |= 1 as i32;
                type_0 = strtou8(optarg, &mut endptr, 0 as i32);
            }
            116 => {
                argtracks = atoui(optarg);
            }
            104 => {
                argheads = atou16(optarg);
            }
            115 => {
                argsectors = atou16(optarg);
            }
            102 => {
                open2flags |= 1 as i32;
                force = 1 as i32;
            }
            118 => {
                verbose += 1;
                verbose;
            }
            98 => {
                begin_set = 1 as i32;
                begin = strtoui(optarg, &mut endptr, 0 as i32);
            }
            108 => {
                size_set = 1 as i32;
                length = parseSize(optarg);
            }
            _ => {
                usage(1 as i32);
            }
        }
        check_number_parse_errno(c as i8, optarg, endptr);
    }
    if argc - optind != 1 as i32
        || *(*argv.offset(optind as isize)).offset(0 as i32 as isize) == 0
        || *(*argv.offset(optind as isize)).offset(1 as i32 as isize) as i32
            != ':' as i32
    {
        usage(1 as i32);
    }
    drive = ch_toupper(*(*argv.offset(optind as isize)).offset(0 as i32 as isize));
    sprintf(
        errmsg.as_mut_ptr(),
        b"Drive '%c:' not supported\0" as *const u8 as *const i8,
        drive as i32,
    );
    Stream = 0 as *mut Stream_t;
    dev = devices;
    while (*dev).drive != 0 {
        let mut mode: i32 = 0;
        free_stream(&mut Stream);
        if !((*dev).drive as i32 != drive as i32) {
            if (*dev).partition < 1 as i32 as u32 || (*dev).partition > 4 as i32 as u32 {
                sprintf(
                    errmsg.as_mut_ptr(),
                    b"Drive '%c:' is not a partition\0" as *const u8 as *const i8,
                    drive as i32,
                );
            } else {
                used_dev = *dev;
                if argtracks != 0 {
                    used_dev.tracks = argtracks;
                }
                if argheads != 0 {
                    used_dev.heads = argheads;
                }
                if argsectors != 0 {
                    used_dev.sectors = argsectors;
                }
                expand((*dev).name, name.as_mut_ptr());
                mode = if dirty != 0 { 0o2 as i32 } else { 0 as i32 };
                if initialize != 0 {
                    mode |= 0o100 as i32;
                }
                Stream = OpenImage(
                    &mut used_dev,
                    dev,
                    name.as_mut_ptr(),
                    mode,
                    errmsg.as_mut_ptr(),
                    open2flags | 2 as i32 | 4 as i32,
                    mode,
                    0 as *mut mt_off_t,
                    0 as *mut i32,
                    0 as *mut xdf_info,
                );
                if Stream.is_null() {
                    snprintf(
                        errmsg.as_mut_ptr(),
                        (::core::mem::size_of::<[i8; 2100]>() as u64)
                            .wrapping_sub(1 as i32 as u64),
                        b"init: open: %s\0" as *const u8 as *const i8,
                        strerror(*__errno_location()),
                    );
                } else {
                    tot_sectors = used_dev.tot_sectors;
                    if ((*(*Stream).Class).pread)
                        .expect(
                            "non-null function pointer",
                        )(
                        Stream,
                        buf.as_mut_ptr() as *mut i8,
                        0 as i32 as mt_off_t,
                        512 as i32 as size_t,
                    ) != 512 as i32 as i64 && initialize == 0
                    {
                        snprintf(
                            errmsg.as_mut_ptr(),
                            (::core::mem::size_of::<[i8; 2100]>() as u64)
                                .wrapping_sub(1 as i32 as u64),
                            b"Error reading from '%s', wrong parameters?\0" as *const u8
                                as *const i8,
                            name.as_mut_ptr(),
                        );
                    } else {
                        if verbose >= 2 as i32 {
                            print_sector(
                                b"Read sector\0" as *const u8 as *const i8,
                                buf.as_mut_ptr(),
                                512 as i32,
                            );
                        }
                        break;
                    }
                }
            }
        }
        dev = dev.offset(1);
        dev;
    }
    if (*dev).drive as i32 == 0 as i32 {
        free_stream(&mut Stream);
        fprintf(
            stderr,
            b"%s: %s\n\0" as *const u8 as *const i8,
            *argv.offset(0 as i32 as isize),
            errmsg.as_mut_ptr(),
        );
        exit(1 as i32);
    }
    if (used_dev.sectors as i32 != 0 || used_dev.heads as i32 != 0)
        && (used_dev.sectors == 0 || used_dev.heads == 0)
    {
        fprintf(
            stderr,
            b"You should either indicate both the number of sectors and the number of heads,\n\0"
                as *const u8 as *const i8,
        );
        fprintf(stderr, b" or none of them\n\0" as *const u8 as *const i8);
        exit(1 as i32);
    }
    if initialize != 0 {
        if !bootSector.is_null() {
            let mut fd: i32 = 0;
            fd = open(bootSector, 0 as i32 | 0 as i32 | 0 as i32);
            if fd < 0 as i32 {
                perror(b"open MBR\0" as *const u8 as *const i8);
                exit(1 as i32);
            }
            if read(
                fd,
                buf.as_mut_ptr() as *mut i8 as *mut libc::c_void,
                512 as i32 as size_t,
            ) < 512 as i32 as i64
            {
                perror(b"read MBR\0" as *const u8 as *const i8);
                exit(1 as i32);
            }
            close(fd);
        }
        memset(
            partTable.offset(1 as i32 as isize) as *mut i8 as *mut libc::c_void,
            0 as i32,
            (4 as i32 as u64).wrapping_mul(::core::mem::size_of::<partition>() as u64),
        );
        set_word(
            buf.as_mut_ptr().offset(510 as i32 as isize),
            0xaa55 as i32 as libc::c_ushort,
        );
    }
    if buf[510 as i32 as usize] as i32 != 0x55 as i32
        || buf[511 as i32 as usize] as i32 != 0xaa as i32
    {
        fprintf(stderr, b"Boot signature not set\n\0" as *const u8 as *const i8);
        fprintf(
            stderr,
            b"Use the -I flag to initialize the partition table, and set the boot signature\n\0"
                as *const u8 as *const i8,
        );
        inconsistency = 1 as i32;
    }
    tpartition = &mut *partTable.offset((*dev).partition as isize) as *mut partition;
    if do_remove != 0 {
        if (*tpartition).end.byte0 == 0 {
            fprintf(
                stderr,
                b"Partition for drive %c: does not exist\n\0" as *const u8 as *const i8,
                drive as i32,
            );
        }
        if (*tpartition).end.byte0 as i32 & 0x3f as i32 == 5 as i32 {
            fprintf(
                stderr,
                b"Partition for drive %c: may be an extended partition\n\0" as *const u8
                    as *const i8,
                drive as i32,
            );
            fprintf(
                stderr,
                b"Use the -f flag to remove it anyways\n\0" as *const u8 as *const i8,
            );
            inconsistency = 1 as i32;
        }
        memset(
            tpartition as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<partition>() as u64,
        );
    }
    if create != 0 && (*tpartition).end.byte0 as i32 != 0 {
        fprintf(
            stderr,
            b"Partition for drive %c: already exists\n\0" as *const u8 as *const i8,
            drive as i32,
        );
        fprintf(
            stderr,
            b"Use the -r flag to remove it before attempting to recreate it\n\0"
                as *const u8 as *const i8,
        );
    }
    compute_lba_geom_from_tot_sectors(&mut used_dev);
    has_activated = 0 as i32;
    i = 1 as i32 as u32;
    while i < 5 as i32 as u32 {
        let mut partition: *mut partition = &mut *partTable.offset(i as isize)
            as *mut partition;
        if !((*partition).end.byte0 == 0) {
            if (*partition).start.byte0 != 0 {
                has_activated += 1;
                has_activated;
            }
            if i < (*dev).partition && begin_set == 0 {
                begin = ((((*partition).start_sect[0 as i32 as usize] as i32
                    + (((*partition).start_sect[1 as i32 as usize] as i32) << 8 as i32))
                    as uint16_t as i32
                    + (((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(0 as i32 as isize) as i32
                        + ((*((*partition).start_sect)
                            .as_mut_ptr()
                            .offset(2 as i32 as isize)
                            .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                        as i32) << 16 as i32)) as uint32_t)
                    .wrapping_add(
                        (((*partition).nr_sects[0 as i32 as usize] as i32
                            + (((*partition).nr_sects[1 as i32 as usize] as i32)
                                << 8 as i32)) as uint16_t as i32
                            + (((*((*partition).nr_sects)
                                .as_mut_ptr()
                                .offset(2 as i32 as isize)
                                .offset(0 as i32 as isize) as i32
                                + ((*((*partition).nr_sects)
                                    .as_mut_ptr()
                                    .offset(2 as i32 as isize)
                                    .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                                as i32) << 16 as i32)) as uint32_t,
                    );
            }
            if i > (*dev).partition && end_set == 0 && size_set == 0 {
                end = (((*partition).start_sect[0 as i32 as usize] as i32
                    + (((*partition).start_sect[1 as i32 as usize] as i32) << 8 as i32))
                    as uint16_t as i32
                    + (((*((*partition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(0 as i32 as isize) as i32
                        + ((*((*partition).start_sect)
                            .as_mut_ptr()
                            .offset(2 as i32 as isize)
                            .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                        as i32) << 16 as i32)) as uint32_t;
                end_set = 1 as i32;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if used_dev.sectors == 0 && used_dev.heads == 0 {
        if tot_sectors != 0 {
            setsize0(
                tot_sectors,
                &mut dummy2,
                &mut used_dev.heads,
                &mut used_dev.sectors,
            );
        } else {
            used_dev.heads = 64 as i32 as uint16_t;
            used_dev.sectors = 32 as i32 as uint16_t;
        }
    }
    if verbose != 0 {
        fprintf(
            stderr,
            b"sectors: %d heads: %d %u\n\0" as *const u8 as *const i8,
            used_dev.sectors as i32,
            used_dev.heads as i32,
            tot_sectors,
        );
    }
    sec_per_cyl = (used_dev.sectors as i32 * used_dev.heads as i32) as uint16_t;
    if create != 0 {
        let mut overlap: u32 = 0;
        if end_set == 0 && size_set == 0 && tot_sectors != 0 {
            end = tot_sectors.wrapping_sub(tot_sectors.wrapping_rem(sec_per_cyl as u32));
            end_set = 1 as i32;
        }
        if begin == 0 && begin_set == 0 {
            begin = (if used_dev.sectors as i32 != 0 {
                used_dev.sectors as i32
            } else {
                2048 as i32
            }) as u32;
        }
        if size_set != 0 {
            end = begin.wrapping_add(length);
        } else if end_set == 0 {
            fprintf(stderr, b"Unknown size\n\0" as *const u8 as *const i8);
            exit(1 as i32);
        }
        if begin >= end {
            fprintf(stderr, b"Begin larger than end\n\0" as *const u8 as *const i8);
            exit(1 as i32);
        }
        overlap = findOverlap(partTable, 4 as i32 as u32, begin, end);
        if overlap != 0 {
            fprintf(
                stderr,
                b"Partition would overlap with partition %d\n\0" as *const u8
                    as *const i8,
                overlap,
            );
            exit(1 as i32);
        }
        setBeginEnd(
            tpartition,
            begin,
            end,
            used_dev.heads,
            used_dev.sectors,
            (has_activated == 0) as i32,
            type_0,
            (if (*dev).fat_bits > 0 as i32 { (*dev).fat_bits } else { -(*dev).fat_bits })
                as u32,
        );
    }
    if activate != 0 {
        if (*tpartition).end.byte0 == 0 {
            fprintf(
                stderr,
                b"Partition for drive %c: does not exist\n\0" as *const u8 as *const i8,
                drive as i32,
            );
        } else {
            match activate {
                1 => {
                    (*tpartition).start.byte0 = 0x80 as i32 as u8;
                }
                -1 => {
                    (*tpartition).start.byte0 = 0 as i32 as u8;
                }
                _ => {}
            }
        }
    }
    inconsistency
        |= consistencyCheck(
            partTable,
            doprint,
            verbose,
            &mut has_activated,
            tot_sectors,
            &mut used_dev,
            (*dev).partition,
        );
    match has_activated {
        0 => {
            fprintf(
                stderr,
                b"Warning: no active (bootable) partition present\n\0" as *const u8
                    as *const i8,
            );
        }
        1 => {}
        _ => {
            fprintf(
                stderr,
                b"Warning: %d active (bootable) partitions present\n\0" as *const u8
                    as *const i8,
                has_activated,
            );
            fprintf(
                stderr,
                b"Usually, a disk should have exactly one active partition\n\0"
                    as *const u8 as *const i8,
            );
        }
    }
    if inconsistency != 0 && force == 0 {
        fprintf(stderr, b"inconsistency detected!\n\0" as *const u8 as *const i8);
        if dirty != 0 {
            fprintf(
                stderr,
                b"Retry with the -f switch to go ahead anyways\n\0" as *const u8
                    as *const i8,
            );
            exit(1 as i32);
        }
    }
    if doprint != 0 && (*tpartition).end.byte0 as i32 != 0 {
        printf(
            b"The following command will recreate the partition for drive %c:\n\0"
                as *const u8 as *const i8,
            drive as i32,
        );
        used_dev.tracks = ((((*tpartition).nr_sects[0 as i32 as usize] as i32
            + (((*tpartition).nr_sects[1 as i32 as usize] as i32) << 8 as i32))
            as uint16_t as i32
            + (((*((*tpartition).nr_sects)
                .as_mut_ptr()
                .offset(2 as i32 as isize)
                .offset(0 as i32 as isize) as i32
                + ((*((*tpartition).nr_sects)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t as i32)
                << 16 as i32)) as uint32_t)
            .wrapping_add(
                ((((*tpartition).start_sect[0 as i32 as usize] as i32
                    + (((*tpartition).start_sect[1 as i32 as usize] as i32) << 8 as i32))
                    as uint16_t as i32
                    + (((*((*tpartition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(0 as i32 as isize) as i32
                        + ((*((*tpartition).start_sect)
                            .as_mut_ptr()
                            .offset(2 as i32 as isize)
                            .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                        as i32) << 16 as i32)) as uint32_t)
                    .wrapping_rem(sec_per_cyl as u32),
            )
            .wrapping_div(sec_per_cyl as u32);
        printf(
            b"mpartition -c -b %d -l %d -t %d -h %d -s %d -b %u %c:\n\0" as *const u8
                as *const i8,
            (((*tpartition).start_sect[0 as i32 as usize] as i32
                + (((*tpartition).start_sect[1 as i32 as usize] as i32) << 8 as i32))
                as uint16_t as i32
                + (((*((*tpartition).start_sect)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(0 as i32 as isize) as i32
                    + ((*((*tpartition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                    as i32) << 16 as i32)) as uint32_t,
            (((*tpartition).nr_sects[0 as i32 as usize] as i32
                + (((*tpartition).nr_sects[1 as i32 as usize] as i32) << 8 as i32))
                as uint16_t as i32
                + (((*((*tpartition).nr_sects)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(0 as i32 as isize) as i32
                    + ((*((*tpartition).nr_sects)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                    as i32) << 16 as i32)) as uint32_t,
            used_dev.tracks,
            used_dev.heads as i32,
            used_dev.sectors as i32,
            (((*tpartition).start_sect[0 as i32 as usize] as i32
                + (((*tpartition).start_sect[1 as i32 as usize] as i32) << 8 as i32))
                as uint16_t as i32
                + (((*((*tpartition).start_sect)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(0 as i32 as isize) as i32
                    + ((*((*tpartition).start_sect)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                    as i32) << 16 as i32)) as uint32_t,
            drive as i32,
        );
    }
    if dirty != 0 {
        if verbose >= 2 as i32 {
            print_sector(
                b"Writing sector\0" as *const u8 as *const i8,
                buf.as_mut_ptr(),
                512 as i32,
            );
        }
        if ((*(*Stream).Class).pwrite)
            .expect(
                "non-null function pointer",
            )(
            Stream,
            buf.as_mut_ptr() as *mut i8,
            0 as i32 as mt_off_t,
            512 as i32 as size_t,
        ) != 512 as i32 as i64
        {
            fprintf(
                stderr,
                b"Error writing partition table\0" as *const u8 as *const i8,
            );
            exit(1 as i32);
        }
        if verbose >= 3 as i32 {
            print_sector(
                b"Sector written\0" as *const u8 as *const i8,
                buf.as_mut_ptr(),
                512 as i32,
            );
        }
    }
    free_stream(&mut Stream);
    exit(0 as i32);
}
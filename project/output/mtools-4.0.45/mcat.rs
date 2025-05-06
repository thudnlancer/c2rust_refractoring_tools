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
    static mut stdin: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn getopt(___argc: i32, ___argv: *const *mut i8, __shortopts: *const i8) -> i32;
    static mut stdout: *mut _IO_FILE;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn get_default_drive() -> i8;
    fn set_cmd_line_image(img: *mut i8);
    fn free_stream(Stream: *mut *mut Stream_t) -> i32;
    fn chs_to_totsectors(dev: *mut device, errmsg: *mut i8) -> i32;
    fn check_if_sectors_fit(
        tot_sectors: uint32_t,
        maxBytes: mt_off_t,
        sectorSize: uint32_t,
        errmsg: *mut i8,
    ) -> i32;
    static mut devices: *mut device;
    fn expand(_: *const i8, _: *mut i8) -> *const i8;
    static mut mdate: *const i8;
    static mut mversion: *const i8;
    static mut noPrivileges: i32;
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
unsafe extern "C" fn usage() -> ! {
    fprintf(
        stderr,
        b"Mtools version %s, dated %s\n\0" as *const u8 as *const i8,
        mversion,
        mdate,
    );
    fprintf(stderr, b"Usage: mcat [-V] [-w] device\n\0" as *const u8 as *const i8);
    fprintf(
        stderr,
        b"       -w write on device else read\n\0" as *const u8 as *const i8,
    );
    exit(1 as i32);
}
unsafe extern "C" fn bufLen(
    mut blocksize: size_t,
    mut totalSize: mt_off_t,
    mut address: mt_off_t,
) -> size_t {
    if totalSize == 0 as i32 as i64 {
        return blocksize;
    }
    if blocksize as mt_off_t > totalSize - address {
        return (totalSize - address) as size_t;
    }
    return blocksize;
}
#[no_mangle]
pub unsafe extern "C" fn mcat(mut argc: i32, mut argv: *mut *mut i8, mut type_0: i32) {
    let mut current_block: u64;
    let mut dev: *mut device = 0 as *mut device;
    let mut out_dev: device = device {
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
    let mut drive: i8 = 0;
    let mut name: [i8; 2048] = [0; 2048];
    let mut errmsg: [i8; 200] = [0; 200];
    let mut Stream: *mut Stream_t = 0 as *mut Stream_t;
    let mut buf: [i8; 16000] = [0; 16000];
    let mut address: mt_off_t = 0 as i32 as mt_off_t;
    let mut maxSize: mt_off_t = 0 as i32 as mt_off_t;
    let mut mode: i8 = 0 as i32 as i8;
    let mut c: i32 = 0;
    noPrivileges = 1 as i32;
    if argc < 2 as i32 {
        usage();
    }
    loop {
        c = getopt(argc, argv, b"wi:\0" as *const u8 as *const i8);
        if !(c != -(1 as i32)) {
            break;
        }
        match c {
            119 => {
                mode = 0o1 as i32 as i8;
            }
            105 => {
                set_cmd_line_image(optarg);
            }
            _ => {
                usage();
            }
        }
    }
    if argc - optind > 1 as i32 {
        usage();
    }
    if argc - optind == 1 as i32 {
        if *(*argv.offset(optind as isize)).offset(0 as i32 as isize) == 0
            || *(*argv.offset(optind as isize)).offset(1 as i32 as isize) as i32
                != ':' as i32
        {
            usage();
        }
        drive = ch_toupper(
            *(*argv.offset((argc - 1 as i32) as isize)).offset(0 as i32 as isize),
        );
    } else {
        drive = get_default_drive();
    }
    sprintf(
        errmsg.as_mut_ptr(),
        b"Drive '%c:' not supported\0" as *const u8 as *const i8,
        drive as i32,
    );
    Stream = 0 as *mut Stream_t;
    dev = devices;
    while !((*dev).name).is_null() {
        free_stream(&mut Stream);
        if !((*dev).drive as i32 != drive as i32) {
            out_dev = *dev;
            expand((*dev).name, name.as_mut_ptr());
            Stream = OpenImage(
                &mut out_dev,
                dev,
                name.as_mut_ptr(),
                mode as i32,
                errmsg.as_mut_ptr(),
                4 as i32,
                mode as i32,
                &mut maxSize,
                0 as *mut i32,
                0 as *mut xdf_info,
            );
            if !Stream.is_null() {
                break;
            }
        }
        dev = dev.offset(1);
        dev;
    }
    if !((*dev).drive as i32 == 0 as i32) {
        if mode as i32 == 0o1 as i32 {
            let mut len: size_t = 0;
            let mut size: mt_off_t = 0 as i32 as mt_off_t;
            if chs_to_totsectors(&mut out_dev, errmsg.as_mut_ptr()) < 0 as i32
                || check_if_sectors_fit(
                    out_dev.tot_sectors,
                    maxSize,
                    512 as i32 as uint32_t,
                    errmsg.as_mut_ptr(),
                ) != 0
            {
                current_block = 10732731936217475923;
            } else {
                size = 512 as i32 as i64 * out_dev.tot_sectors as mt_off_t;
                loop {
                    len = fread(
                        buf.as_mut_ptr() as *mut libc::c_void,
                        1 as i32 as size_t,
                        bufLen(16000 as u32 as size_t, size, address),
                        stdin,
                    );
                    if !(len > 0 as i32 as u64) {
                        break;
                    }
                    let mut r: ssize_t = ((*(*Stream).Class).pwrite)
                        .expect(
                            "non-null function pointer",
                        )(Stream, buf.as_mut_ptr(), address, len);
                    fprintf(
                        stderr,
                        b"Wrote to %d\n\0" as *const u8 as *const i8,
                        address as i32,
                    );
                    if r < 0 as i32 as i64 {
                        break;
                    }
                    address = (address as u64).wrapping_add(len) as mt_off_t as mt_off_t;
                }
                current_block = 5330834795799507926;
            }
        } else {
            let mut len_0: ssize_t = 0;
            loop {
                len_0 = ((*(*Stream).Class).pread)
                    .expect(
                        "non-null function pointer",
                    )(Stream, buf.as_mut_ptr(), address, 16000 as u32 as size_t);
                if !(len_0 > 0 as i32 as i64) {
                    break;
                }
                fwrite(
                    buf.as_mut_ptr() as *const libc::c_void,
                    1 as i32 as size_t,
                    len_0 as size_t,
                    stdout,
                );
                address = (address as u64).wrapping_add(len_0 as size_t) as mt_off_t
                    as mt_off_t;
            }
            current_block = 5330834795799507926;
        }
        match current_block {
            10732731936217475923 => {}
            _ => {
                free_stream(&mut Stream);
                exit(0 as i32);
            }
        }
    }
    free_stream(&mut Stream);
    fprintf(stderr, b"%s\n\0" as *const u8 as *const i8, errmsg.as_mut_ptr());
    exit(1 as i32);
}
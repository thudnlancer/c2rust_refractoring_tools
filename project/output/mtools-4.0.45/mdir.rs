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
    pub type Fs_t;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn getopt(___argc: i32, ___argv: *const *mut i8, __shortopts: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn strncasecmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn towlower(__wc: wint_t) -> wint_t;
    fn copy_stream(Stream: *mut Stream_t) -> *mut Stream_t;
    fn free_stream(Stream: *mut *mut Stream_t) -> i32;
    fn getfree(Stream: *mut Stream_t) -> mt_off_t;
    fn GetFs(Fs: *mut Stream_t) -> *mut Stream_t;
    static mut mtools_ignore_short_case: u32;
    static mut mtools_dotted_dir: u32;
    static mut mtools_twenty_four_hour_clock: u32;
    static mut mtools_date_string: *const i8;
    fn set_cmd_line_image(img: *mut i8);
    static mut progname: *const i8;
    fn helpFlag(_: i32, _: *mut *mut i8) -> i32;
    fn getDrive(Stream: *mut Stream_t) -> i8;
    static mut mversion: *const i8;
    static mut mdate: *const i8;
    fn getDirentry(Stream: *mut Stream_t) -> *mut direntry_t;
    fn getPwd(entry: *mut direntry_t) -> *mut i8;
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
    fn initializeDirentry(entry: *mut direntry_t, Dir: *mut Stream_t);
    fn OpenRoot(Dir: *mut Stream_t) -> *mut Stream_t;
    fn init_mp(MainParam: *mut MainParam_t);
    fn main_loop(MainParam: *mut MainParam_t, argv: *mut *mut i8, argc: i32) -> i32;
    fn getSerialized(File: *mut Fs_t) -> bool;
    fn getSerialNumber(File: *mut Fs_t) -> u64;
    fn dos_to_wchar(
        fromDos: *mut doscp_t,
        dos: *const i8,
        wchar: *mut wchar_t,
        len: size_t,
    ) -> size_t;
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
pub type C2RustUnnamed = u32;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type wint_t = u32;
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
#[inline]
unsafe extern "C" fn putchar(mut __c: i32) -> i32 {
    return _IO_putc(__c, stdout);
}
#[inline]
unsafe extern "C" fn ch_towlower(mut ch: wchar_t) -> wchar_t {
    return towlower(ch as wint_t) as wchar_t;
}
static mut recursive: i32 = 0;
static mut wide: i32 = 0;
static mut all: i32 = 0;
static mut concise: i32 = 0;
static mut fast: i32 = 0 as i32;
static mut dirPath: *const i8 = 0 as *const i8;
static mut dynDirPath: *mut i8 = 0 as *const i8 as *mut i8;
static mut currentDrive: i8 = 0;
static mut currentDir: *mut Stream_t = 0 as *const Stream_t as *mut Stream_t;
static mut filesInDir: i32 = 0;
static mut filesOnDrive: i32 = 0;
static mut dirsOnDrive: i32 = 0;
static mut debug: i32 = 0 as i32;
static mut bytesInDir: mt_off_t = 0;
static mut bytesOnDrive: mt_off_t = 0;
static mut RootDir: *mut Stream_t = 0 as *const Stream_t as *mut Stream_t;
static mut mdir_shortname: [i8; 49] = [0; 49];
static mut mdir_longname: [i8; 1021] = [0; 1021];
#[inline]
unsafe extern "C" fn print_date(mut dir: *mut directory) {
    let mut year: [i8; 5] = [0; 5];
    let mut day: [i8; 3] = [0; 3];
    let mut month: [i8; 3] = [0; 3];
    let mut p: *const i8 = 0 as *const i8;
    sprintf(
        year.as_mut_ptr(),
        b"%04d\0" as *const u8 as *const i8,
        ((*dir).date[1 as i32 as usize] as i32 >> 1 as i32) + 1980 as i32,
    );
    sprintf(
        day.as_mut_ptr(),
        b"%02d\0" as *const u8 as *const i8,
        (*dir).date[0 as i32 as usize] as i32 & 0x1f as i32,
    );
    sprintf(
        month.as_mut_ptr(),
        b"%02d\0" as *const u8 as *const i8,
        (((*dir).date[1 as i32 as usize] as i32 & 0x1 as i32) << 3 as i32)
            + ((*dir).date[0 as i32 as usize] as i32 >> 5 as i32),
    );
    p = mtools_date_string;
    while *p != 0 {
        if strncasecmp(p, b"yyyy\0" as *const u8 as *const i8, 4 as i32 as u64) == 0 {
            printf(
                b"%04d\0" as *const u8 as *const i8,
                ((*dir).date[1 as i32 as usize] as i32 >> 1 as i32) + 1980 as i32,
            );
            p = p.offset(3 as i32 as isize);
        } else if strncasecmp(p, b"yy\0" as *const u8 as *const i8, 2 as i32 as u64) == 0
        {
            printf(
                b"%02d\0" as *const u8 as *const i8,
                (((*dir).date[1 as i32 as usize] as i32 >> 1 as i32) + 1980 as i32)
                    % 100 as i32,
            );
            p = p.offset(1);
            p;
        } else if strncasecmp(p, b"dd\0" as *const u8 as *const i8, 2 as i32 as u64) == 0
        {
            printf(
                b"%02d\0" as *const u8 as *const i8,
                (*dir).date[0 as i32 as usize] as i32 & 0x1f as i32,
            );
            p = p.offset(1);
            p;
        } else if strncasecmp(p, b"mm\0" as *const u8 as *const i8, 2 as i32 as u64) == 0
        {
            printf(
                b"%02d\0" as *const u8 as *const i8,
                (((*dir).date[1 as i32 as usize] as i32 & 0x1 as i32) << 3 as i32)
                    + ((*dir).date[0 as i32 as usize] as i32 >> 5 as i32),
            );
            p = p.offset(1);
            p;
        } else {
            putchar(*p as i32);
        }
        p = p.offset(1);
        p;
    }
}
#[inline]
unsafe extern "C" fn print_time(mut dir: *mut directory) {
    let mut am_pm: i8 = 0;
    let mut hour: i32 = (*dir).time[1 as i32 as usize] as i32 >> 3 as i32;
    if mtools_twenty_four_hour_clock == 0 {
        am_pm = (if hour >= 12 as i32 { 'p' as i32 } else { 'a' as i32 }) as i8;
        if hour > 12 as i32 {
            hour = hour - 12 as i32;
        }
        if hour == 0 as i32 {
            hour = 12 as i32;
        }
    } else {
        am_pm = ' ' as i32 as i8;
    }
    printf(
        b"%2d:%02d%c\0" as *const u8 as *const i8,
        hour,
        (((*dir).time[1 as i32 as usize] as i32 & 0x7 as i32) << 3 as i32)
            + ((*dir).time[0 as i32 as usize] as i32 >> 5 as i32),
        am_pm as i32,
    );
}
unsafe extern "C" fn dotted_num(
    mut num: mt_off_t,
    mut width: size_t,
    mut buf: *mut *mut i8,
) -> *const i8 {
    let mut len: size_t = 0;
    let mut srcp: *mut i8 = 0 as *mut i8;
    let mut dstp: *mut i8 = 0 as *mut i8;
    let mut size: size_t = 0;
    let mut numlo: u64 = 0;
    let mut numhi: u64 = 0;
    size = width.wrapping_add(width);
    *buf = malloc(size.wrapping_add(1 as i32 as u64)) as *mut i8;
    if (*buf).is_null() {
        return b"\0" as *const u8 as *const i8;
    }
    numlo = (num % 1000000000 as i32 as i64) as u64;
    numhi = (num / 1000000000 as i32 as i64) as u64;
    if numhi != 0 && size > 9 as i32 as u64 {
        sprintf(
            *buf,
            b"%.*lu%09lu\0" as *const u8 as *const i8,
            size.wrapping_sub(9 as i32 as u64) as i32,
            numhi,
            numlo,
        );
    } else {
        sprintf(*buf, b"%.*lu\0" as *const u8 as *const i8, size as i32, numlo);
    }
    srcp = *buf;
    while *srcp.offset(1 as i32 as isize) as i32 != '\0' as i32 {
        if !(*srcp.offset(0 as i32 as isize) as i32 == '0' as i32) {
            break;
        }
        *srcp.offset(0 as i32 as isize) = ' ' as i32 as i8;
        srcp = srcp.offset(1);
        srcp;
    }
    len = strlen(*buf);
    srcp = (*buf).offset(len as isize);
    dstp = (*buf).offset(len as isize).offset(1 as i32 as isize);
    while dstp >= (*buf).offset(4 as i32 as isize)
        && *(*__ctype_b_loc())
            .offset(*srcp.offset(-(1 as i32) as isize) as u8 as i32 as isize) as i32
            & _ISdigit as i32 as libc::c_ushort as i32 != 0
    {
        srcp = srcp.offset(-(3 as i32 as isize));
        dstp = dstp.offset(-(4 as i32 as isize));
    }
    while dstp < (*buf).offset(len as isize) {
        *dstp.offset(0 as i32 as isize) = *srcp.offset(0 as i32 as isize);
        *dstp.offset(1 as i32 as isize) = *srcp.offset(1 as i32 as isize);
        *dstp.offset(2 as i32 as isize) = *srcp.offset(2 as i32 as isize);
        if dstp.offset(3 as i32 as isize) < (*buf).offset(len as isize) {
            *dstp.offset(3 as i32 as isize) = ' ' as i32 as i8;
        }
        srcp = srcp.offset(3 as i32 as isize);
        dstp = dstp.offset(4 as i32 as isize);
    }
    return (*buf).offset(len as isize).offset(-(width as isize));
}
#[inline]
unsafe extern "C" fn print_volume_label(mut Dir: *mut Stream_t, mut drive: i8) -> i32 {
    let mut Stream: *mut Stream_t = GetFs(Dir);
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
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    let mut shortname: [i8; 13] = [0; 13];
    let mut longname: [i8; 261] = [0; 261];
    let mut r: i32 = 0;
    RootDir = OpenRoot(Stream);
    if concise != 0 {
        return 0 as i32;
    }
    initializeDirentry(&mut entry, RootDir);
    r = vfat_lookup(
        &mut entry,
        0 as *const i8,
        0 as i32 as size_t,
        0x8 as i32 | 0x40 as i32,
        shortname.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 13]>() as u64,
        longname.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 261]>() as u64,
    );
    if r != 0 {
        if r == -(2 as i32) {
            return -(1 as i32);
        }
        printf(
            b" Volume in drive %c has no label\0" as *const u8 as *const i8,
            drive as i32,
        );
    } else if *longname.as_mut_ptr() != 0 {
        printf(
            b" Volume in drive %c is %s (abbr=%s)\0" as *const u8 as *const i8,
            drive as i32,
            longname.as_mut_ptr(),
            shortname.as_mut_ptr(),
        );
    } else {
        printf(
            b" Volume in drive %c is %s\0" as *const u8 as *const i8,
            drive as i32,
            shortname.as_mut_ptr(),
        );
    }
    if getSerialized(This) {
        let mut serial_number: u64 = getSerialNumber(This);
        printf(
            b"\n Volume Serial Number is %04lX-%04lX\0" as *const u8 as *const i8,
            serial_number >> 16 as i32 & 0xffff as i32 as u64,
            serial_number & 0xffff as i32 as u64,
        );
    }
    return 0 as i32;
}
unsafe extern "C" fn printSummary(mut files: i32, mut bytes: mt_off_t) {
    if filesInDir == 0 {
        printf(b"No files\n\0" as *const u8 as *const i8);
    } else {
        let mut s1: *mut i8 = 0 as *mut i8;
        printf(b"      %3d file\0" as *const u8 as *const i8, files);
        if files == 1 as i32 {
            putchar(' ' as i32);
        } else {
            putchar('s' as i32);
        }
        printf(
            b"       %s bytes\n\0" as *const u8 as *const i8,
            dotted_num(bytes, 13 as i32 as size_t, &mut s1),
        );
        if !s1.is_null() {
            free(s1 as *mut libc::c_void);
        }
    };
}
unsafe extern "C" fn leaveDrive(mut haveError: i32) {
    if currentDrive == 0 {
        return;
    }
    leaveDirectory(haveError);
    if concise == 0 && haveError == 0 {
        if dirsOnDrive > 1 as i32 {
            printf(b"\nTotal files listed:\n\0" as *const u8 as *const i8);
            printSummary(filesOnDrive, bytesOnDrive);
        }
        if !RootDir.is_null() && fast == 0 {
            let mut s1: *mut i8 = 0 as *mut i8;
            let mut bytes: mt_off_t = getfree(RootDir);
            if bytes == -(1 as i32) as i64 {
                fprintf(stderr, b"Fat error\n\0" as *const u8 as *const i8);
            } else {
                printf(
                    b"                  %s bytes free\n\n\0" as *const u8 as *const i8,
                    dotted_num(bytes, 17 as i32 as size_t, &mut s1),
                );
                if !s1.is_null() {
                    free(s1 as *mut libc::c_void);
                }
            }
        }
    }
    free_stream(&mut RootDir);
    currentDrive = '\0' as i32 as i8;
}
unsafe extern "C" fn enterDrive(mut Dir: *mut Stream_t, mut drive: i8) -> i32 {
    let mut r: i32 = 0;
    if currentDrive as i32 == drive as i32 {
        return 0 as i32;
    }
    leaveDrive(0 as i32);
    currentDrive = drive;
    r = print_volume_label(Dir, drive);
    if r != 0 {
        return r;
    }
    bytesOnDrive = 0 as i32 as mt_off_t;
    filesOnDrive = 0 as i32;
    dirsOnDrive = 0 as i32;
    return 0 as i32;
}
static mut emptyString: *const i8 = b"<out-of-memory>\0" as *const u8 as *const i8;
unsafe extern "C" fn leaveDirectory(mut haveError: i32) {
    if currentDir.is_null() {
        return;
    }
    if haveError == 0 {
        if !dirPath.is_null() && dirPath != emptyString {
            free(dynDirPath as *mut libc::c_void);
        }
        if wide != 0 {
            putchar('\n' as i32);
        }
        if concise == 0 {
            printSummary(filesInDir, bytesInDir);
        }
    }
    free_stream(&mut currentDir);
}
unsafe extern "C" fn enterDirectory(mut Dir: *mut Stream_t) -> i32 {
    let mut r: i32 = 0;
    let mut drive: i8 = 0;
    if currentDir == Dir {
        return 0 as i32;
    }
    leaveDirectory(0 as i32);
    drive = getDrive(Dir);
    r = enterDrive(Dir, drive);
    if r != 0 {
        return r;
    }
    currentDir = copy_stream(Dir);
    dynDirPath = getPwd(getDirentry(Dir));
    if dynDirPath.is_null() {
        dirPath = emptyString;
    } else {
        if *dynDirPath.offset(3 as i32 as isize) == 0 && concise != 0 {
            *dynDirPath.offset(2 as i32 as isize) = '\0' as i32 as i8;
        }
        dirPath = dynDirPath;
    }
    if concise == 0 {
        printf(b"\nDirectory for %s\n\0" as *const u8 as *const i8, dirPath);
    }
    if wide == 0 && concise == 0 {
        printf(b"\n\0" as *const u8 as *const i8);
    }
    dirsOnDrive += 1;
    dirsOnDrive;
    bytesInDir = 0 as i32 as mt_off_t;
    filesInDir = 0 as i32;
    return 0 as i32;
}
unsafe extern "C" fn list_file(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> i32 {
    let mut size: u64 = 0;
    let mut i: i32 = 0;
    let mut Case: i32 = 0;
    let mut r: i32 = 0;
    let mut ext: [wchar_t; 4] = [0; 4];
    let mut name: [wchar_t; 9] = [0; 9];
    let mut cp: *mut doscp_t = 0 as *mut doscp_t;
    if all == 0 && (*entry).dir.attr as i32 & 0x6 as i32 != 0 {
        return 0 as i32;
    }
    if concise != 0 && isSpecialW(((*entry).name).as_mut_ptr()) != 0 {
        return 0 as i32;
    }
    r = enterDirectory((*entry).Dir);
    if r != 0 {
        return 16 as i32;
    }
    if wide != 0 {
        if filesInDir % 5 as i32 != 0 {
            putchar(' ' as i32);
        } else {
            putchar('\n' as i32);
        }
    }
    if (*entry).dir.attr as i32 & 0x10 as i32 != 0 {
        size = 0 as i32 as u64;
    } else {
        size = (((*entry).dir.size[0 as i32 as usize] as i32
            + (((*entry).dir.size[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
            as i32
            + (((*((*entry).dir.size)
                .as_mut_ptr()
                .offset(2 as i32 as isize)
                .offset(0 as i32 as isize) as i32
                + ((*((*entry).dir.size)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t as i32)
                << 16 as i32)) as uint32_t as u64;
    }
    Case = (*entry).dir.Case as i32;
    if Case & (0x8 as i32 | 0x10 as i32) == 0 && mtools_ignore_short_case != 0 {
        Case |= 0x8 as i32 | 0x10 as i32;
    }
    cp = ((*(*(*entry).Dir).Class).get_dosConvert)
        .expect("non-null function pointer")((*entry).Dir);
    dos_to_wchar(
        cp,
        ((*entry).dir.ext).as_mut_ptr(),
        ext.as_mut_ptr(),
        3 as i32 as size_t,
    );
    if Case & 0x10 as i32 != 0 {
        i = 0 as i32;
        while i < 3 as i32 {
            ext[i as usize] = ch_towlower(ext[i as usize]);
            i += 1;
            i;
        }
    }
    ext[3 as i32 as usize] = '\0' as i32;
    if (*entry).dir.name[0 as i32 as usize] as i32 == '\u{5}' as i32 {
        dos_to_wchar(
            cp,
            b"\xE5\0" as *const u8 as *const i8,
            name.as_mut_ptr(),
            1 as i32 as size_t,
        );
        dos_to_wchar(
            cp,
            ((*entry).dir.name).as_mut_ptr().offset(1 as i32 as isize),
            name.as_mut_ptr().offset(1 as i32 as isize),
            7 as i32 as size_t,
        );
    } else {
        dos_to_wchar(
            cp,
            ((*entry).dir.name).as_mut_ptr(),
            name.as_mut_ptr(),
            8 as i32 as size_t,
        );
    }
    if Case & 0x8 as i32 != 0 {
        i = 0 as i32;
        while i < 8 as i32 {
            name[i as usize] = ch_towlower(name[i as usize]);
            i += 1;
            i;
        }
    }
    name[8 as i32 as usize] = '\0' as i32;
    if wide != 0 {
        if (*entry).dir.attr as i32 & 0x10 as i32 != 0 {
            printf(
                b"[%s]%*s\0" as *const u8 as *const i8,
                mdir_shortname.as_mut_ptr(),
                ((15 as i32 - 2 as i32) as u64)
                    .wrapping_sub(strlen(mdir_shortname.as_mut_ptr())) as i32,
                b"\0" as *const u8 as *const i8,
            );
        } else {
            printf(b"%-15s\0" as *const u8 as *const i8, mdir_shortname.as_mut_ptr());
        }
    } else if concise == 0 {
        let mut tmpBasename: [i8; 33] = [0; 33];
        let mut tmpExt: [i8; 13] = [0; 13];
        wchar_to_native(
            name.as_mut_ptr(),
            tmpBasename.as_mut_ptr(),
            8 as i32 as size_t,
            ::core::mem::size_of::<[i8; 33]>() as u64,
        );
        wchar_to_native(
            ext.as_mut_ptr(),
            tmpExt.as_mut_ptr(),
            3 as i32 as size_t,
            ::core::mem::size_of::<[i8; 13]>() as u64,
        );
        if name[0 as i32 as usize] == ' ' as i32 {
            printf(b"             \0" as *const u8 as *const i8);
        } else if mtools_dotted_dir != 0 {
            printf(b"%-12s \0" as *const u8 as *const i8, mdir_shortname.as_mut_ptr());
        } else {
            printf(
                b"%s %s \0" as *const u8 as *const i8,
                tmpBasename.as_mut_ptr(),
                tmpExt.as_mut_ptr(),
            );
        }
        if (*entry).dir.attr as i32 & 0x10 as i32 != 0 {
            printf(b"<DIR>    \0" as *const u8 as *const i8);
        } else {
            printf(b" %8ld\0" as *const u8 as *const i8, size as i64);
        }
        printf(b" \0" as *const u8 as *const i8);
        print_date(&mut (*entry).dir);
        printf(b"  \0" as *const u8 as *const i8);
        print_time(&mut (*entry).dir);
        if debug != 0 {
            printf(
                b" %s %d \0" as *const u8 as *const i8,
                tmpBasename.as_mut_ptr(),
                ((*entry).dir.start[0 as i32 as usize] as i32
                    + (((*entry).dir.start[1 as i32 as usize] as i32) << 8 as i32))
                    as uint16_t as i32,
            );
        }
        if *mdir_longname.as_mut_ptr() != 0 {
            printf(b" %s\0" as *const u8 as *const i8, mdir_longname.as_mut_ptr());
        }
        printf(b"\n\0" as *const u8 as *const i8);
    } else {
        let mut tmp: [i8; 1021] = [0; 1021];
        wchar_to_native(
            ((*entry).name).as_mut_ptr(),
            tmp.as_mut_ptr(),
            255 as i32 as size_t,
            ::core::mem::size_of::<[i8; 1021]>() as u64,
        );
        printf(b"%s/%s\0" as *const u8 as *const i8, dirPath, tmp.as_mut_ptr());
        if (*entry).dir.attr as i32 & 0x10 as i32 != 0 {
            putchar('/' as i32);
        }
        putchar('\n' as i32);
    }
    filesOnDrive += 1;
    filesOnDrive;
    filesInDir += 1;
    filesInDir;
    bytesOnDrive = (bytesOnDrive as u64).wrapping_add(size) as mt_off_t as mt_off_t;
    bytesInDir = (bytesInDir as u64).wrapping_add(size) as mt_off_t as mt_off_t;
    return 4 as i32;
}
unsafe extern "C" fn list_non_recurs_directory(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> i32 {
    let mut r: i32 = 0;
    if (*mp).basenameHasWildcard != 0 {
        return list_file(entry, mp)
    } else {
        let mut subMp: MainParam_t = MainParam_t {
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
        };
        r = enterDirectory((*mp).File);
        if r != 0 {
            return 16 as i32;
        }
        subMp = *mp;
        subMp.dirCallback = subMp.callback;
        return ((*mp).loop_0)
            .expect(
                "non-null function pointer",
            )((*mp).File, &mut subMp, b"*\0" as *const u8 as *const i8) | 4 as i32;
    };
}
unsafe extern "C" fn list_recurs_directory(
    mut entry: *mut direntry_t,
    mut mp: *mut MainParam_t,
) -> i32 {
    let mut subMp: MainParam_t = MainParam_t {
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
    };
    let mut ret: i32 = 0;
    subMp = *mp;
    subMp.lookupflags = 0x10 as i32 | 0x20 as i32;
    subMp.dirCallback = Some(
        list_file as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
    );
    subMp.callback = Some(
        list_file as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
    );
    ret = ((*mp).loop_0)
        .expect(
            "non-null function pointer",
        )((*mp).File, &mut subMp, b"*\0" as *const u8 as *const i8);
    subMp = *mp;
    subMp.lookupflags = 0x10 as i32 | 0x100 as i32 | 0x80 as i32 | 1 as i32;
    return ret
        | ((*mp).loop_0)
            .expect(
                "non-null function pointer",
            )((*mp).File, &mut subMp, b"*\0" as *const u8 as *const i8) | 4 as i32;
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
        b"Usage: %s: [-V] [-w] [-a] [-b] [-s] [-f] msdosdirectory\n\0" as *const u8
            as *const i8,
        progname,
    );
    fprintf(
        stderr,
        b"       %s: [-V] [-w] [-a] [-b] [-s] [-f] msdosfile [msdosfiles...]\n\0"
            as *const u8 as *const i8,
        progname,
    );
    exit(ret);
}
#[no_mangle]
pub unsafe extern "C" fn mdir(mut argc: i32, mut argv: *mut *mut i8, mut type_0: i32) {
    let mut ret: i32 = 0;
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
    };
    let mut c: i32 = 0;
    let mut fakedArgv: [*const i8; 1] = [b".\0" as *const u8 as *const i8];
    concise = 0 as i32;
    recursive = 0 as i32;
    all = 0 as i32;
    wide = all;
    if helpFlag(argc, argv) != 0 {
        usage(0 as i32);
    }
    loop {
        c = getopt(argc, argv, b"i:waXbfds/h\0" as *const u8 as *const i8);
        if !(c != -(1 as i32)) {
            break;
        }
        match c {
            105 => {
                set_cmd_line_image(optarg);
            }
            119 => {
                wide = 1 as i32;
            }
            97 => {
                all = 1 as i32;
            }
            98 | 88 => {
                concise = 1 as i32;
            }
            115 | 47 => {
                recursive = 1 as i32;
            }
            102 => {
                fast = 1 as i32;
            }
            100 => {
                debug = 1 as i32;
            }
            104 => {
                usage(0 as i32);
            }
            _ => {
                usage(1 as i32);
            }
        }
    }
    if optind == argc {
        argv = fakedArgv.as_mut_ptr() as *mut *mut i8;
        argc = 1 as i32;
        optind = 0 as i32;
    }
    init_mp(&mut mp);
    currentDrive = '\0' as i32 as i8;
    currentDir = 0 as *mut Stream_t;
    RootDir = 0 as *mut Stream_t;
    dirPath = 0 as *const i8;
    if recursive != 0 {
        mp.lookupflags = 0x10 as i32 | 0x20 as i32 | 0x400 as i32 | 0x100 as i32;
        mp.dirCallback = Some(
            list_recurs_directory
                as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
        );
        mp.callback = Some(
            list_file as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
        );
    } else {
        mp.lookupflags = 0x10 as i32 | 0x20 as i32 | 0x400 as i32;
        mp.dirCallback = Some(
            list_non_recurs_directory
                as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
        );
        mp.callback = Some(
            list_file as unsafe extern "C" fn(*mut direntry_t, *mut MainParam_t) -> i32,
        );
    }
    mp.longname.data = mdir_longname.as_mut_ptr();
    mp.longname.len = ::core::mem::size_of::<[i8; 1021]>() as u64;
    mp.shortname.data = mdir_shortname.as_mut_ptr();
    mp.shortname.len = ::core::mem::size_of::<[i8; 49]>() as u64;
    ret = main_loop(&mut mp, argv.offset(optind as isize), argc - optind);
    leaveDirectory(ret);
    leaveDrive(ret);
    exit(ret);
}
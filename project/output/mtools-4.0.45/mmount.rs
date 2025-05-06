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
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn close(__fd: i32) -> i32;
    fn execvp(__file: *const i8, __argv: *const *mut i8) -> i32;
    fn execlp(__file: *const i8, __arg: *const i8, _: ...) -> i32;
    fn fork() -> __pid_t;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strdup(_: *const i8) -> *mut i8;
    fn wait(__stat_loc: *mut i32) -> __pid_t;
    fn free_stream(Stream: *mut *mut Stream_t) -> i32;
    fn printOom();
    fn destroy_privs();
    fn find_device(
        drive: i8,
        mode: i32,
        out_dev: *mut device,
        boot: *mut bootsector,
        name: *mut i8,
        media: *mut i32,
        maxSize: *mut mt_off_t,
        isRop: *mut i32,
    ) -> *mut Stream_t;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
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
pub union bootsector {
    pub bytes: [u8; 4096],
    pub characters: [i8; 4096],
    pub boot: bootsector_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bootsector_s {
    pub jump: [u8; 3],
    pub banner: [i8; 8],
    pub secsiz: [u8; 2],
    pub clsiz: u8,
    pub nrsvsect: [u8; 2],
    pub nfat: u8,
    pub dirents: [u8; 2],
    pub psect: [u8; 2],
    pub descr: u8,
    pub fatlen: [u8; 2],
    pub nsect: [u8; 2],
    pub nheads: [u8; 2],
    pub nhs: [u8; 4],
    pub bigsect: [u8; 4],
    pub ext: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub fat32: fat32_t,
    pub old: oldboot_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oldboot_t {
    pub labelBlock: label_blk_t,
    pub res_2m: u8,
    pub CheckSum: u8,
    pub fmt_2mf: u8,
    pub wt: u8,
    pub rate_0: u8,
    pub rate_any: u8,
    pub BootP: [u8; 2],
    pub Infp0: [u8; 2],
    pub InfpX: [u8; 2],
    pub InfTm: [u8; 2],
    pub DateF: [u8; 2],
    pub TimeF: [u8; 2],
    pub junk: [u8; 944],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct label_blk_t {
    pub physdrive: u8,
    pub reserved: u8,
    pub dos4: u8,
    pub serial: [u8; 4],
    pub label: [i8; 11],
    pub fat_type: [i8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fat32_t {
    pub bigFat: [u8; 4],
    pub extFlags: [u8; 2],
    pub fsVersion: [u8; 2],
    pub rootCluster: [u8; 4],
    pub infoSector: [u8; 2],
    pub backupBoot: [u8; 2],
    pub reserved: [u8; 6],
    pub reserved2: [u8; 6],
    pub labelBlock: label_blk_t,
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
#[no_mangle]
pub unsafe extern "C" fn mmount(mut argc: i32, mut argv: *mut *mut i8, mut type_0: i32) {
    let mut drive: i8 = 0;
    let mut pid: i32 = 0;
    let mut status: i32 = 0;
    let mut dev: device = device {
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
    let mut name: [i8; 2048] = [0; 2048];
    let mut media: i32 = 0;
    let mut boot: bootsector = bootsector { bytes: [0; 4096] };
    let mut Stream: *mut Stream_t = 0 as *mut Stream_t;
    if argc < 2 as i32
        || *(*argv.offset(1 as i32 as isize)).offset(0 as i32 as isize) == 0
        || *(*argv.offset(1 as i32 as isize)).offset(1 as i32 as isize) as i32
            != ':' as i32
        || *(*argv.offset(1 as i32 as isize)).offset(2 as i32 as isize) as i32 != 0
    {
        fprintf(
            stderr,
            b"Usage: %s -V drive:\n\0" as *const u8 as *const i8,
            *argv.offset(0 as i32 as isize),
        );
        exit(1 as i32);
    }
    drive = ch_toupper(*(*argv.offset(1 as i32 as isize)).offset(0 as i32 as isize));
    Stream = find_device(
        drive,
        0 as i32,
        &mut dev,
        &mut boot,
        name.as_mut_ptr(),
        &mut media,
        0 as *mut mt_off_t,
        0 as *mut i32,
    );
    if Stream.is_null() {
        exit(1 as i32);
    }
    free_stream(&mut Stream);
    destroy_privs();
    if dev.partition != 0 {
        let mut part_name: [i8; 4] = [0; 4];
        sprintf(
            part_name.as_mut_ptr(),
            b"%d\0" as *const u8 as *const i8,
            (dev.partition).wrapping_rem(1000 as i32 as u32),
        );
        strcat(name.as_mut_ptr(), part_name.as_mut_ptr());
    }
    pid = fork();
    match pid {
        -1 => {
            fprintf(stderr, b"fork failed\n\0" as *const u8 as *const i8);
            exit(1 as i32);
        }
        0 => {
            close(2 as i32);
            open(
                b"/dev/null\0" as *const u8 as *const i8,
                0o2 as i32 | 0 as i32 | 0 as i32,
            );
            let ref mut fresh0 = *argv.offset(1 as i32 as isize);
            *fresh0 = strdup(b"mount\0" as *const u8 as *const i8);
            if argc > 2 as i32 {
                execvp(
                    b"mount\0" as *const u8 as *const i8,
                    argv.offset(1 as i32 as isize) as *const *mut i8,
                );
            } else {
                execlp(
                    b"mount\0" as *const u8 as *const i8,
                    b"mount\0" as *const u8 as *const i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
            }
            perror(b"exec mount\0" as *const u8 as *const i8);
            exit(1 as i32);
        }
        _ => while wait(&mut status) != pid {}
    }
    if (status & 0xff00 as i32) >> 8 as i32 == 0 as i32 {
        exit(0 as i32);
    }
    let ref mut fresh1 = *argv.offset(0 as i32 as isize);
    *fresh1 = strdup(b"mount\0" as *const u8 as *const i8);
    let ref mut fresh2 = *argv.offset(1 as i32 as isize);
    *fresh2 = strdup(b"-r\0" as *const u8 as *const i8);
    if (*argv.offset(0 as i32 as isize)).is_null()
        || (*argv.offset(1 as i32 as isize)).is_null()
    {
        printOom();
        exit(1 as i32);
    }
    if argc > 2 as i32 {
        execvp(b"mount\0" as *const u8 as *const i8, argv as *const *mut i8);
    } else {
        execlp(
            b"mount\0" as *const u8 as *const i8,
            b"mount\0" as *const u8 as *const i8,
            b"-r\0" as *const u8 as *const i8,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
    }
    exit(1 as i32);
}
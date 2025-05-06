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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
    fn close(__fd: i32) -> i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn __errno_location() -> *mut i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strerror(_: i32) -> *mut i8;
    fn lock_dev(fd: i32, mode: i32, dev: *mut device) -> i32;
    fn precmd(dev: *mut device);
    fn postcmd(cmd: *const i8);
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn closeExec(fd: i32);
    fn truncBytes32(off: mt_off_t) -> off_t;
    fn send_one_cmd(fd: i32, raw_cmd: *mut RawRequest_t, message: *const i8) -> i32;
    fn analyze_one_reply(
        raw_cmd: *mut RawRequest_t,
        bytes: *mut i32,
        do_print: i32,
    ) -> i32;
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
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type __caddr_t = *mut i8;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type caddr_t = __caddr_t;
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
pub union bootsector {
    pub bytes: [u8; 4096],
    pub characters: [i8; 4096],
    pub boot: bootsector_s,
}
pub type mt_off_t = off_t;
pub type smt_off_t = mt_off_t;
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
pub struct floppy_raw_cmd {
    pub flags: u32,
    pub data: *mut libc::c_void,
    pub kernel_data: *mut i8,
    pub next: *mut floppy_raw_cmd,
    pub length: i64,
    pub phys_length: i64,
    pub buffer_length: i32,
    pub rate: u8,
    pub cmd_count: u8,
    pub cmd: [u8; 16],
    pub reply_count: u8,
    pub reply: [u8; 16],
    pub track: i32,
    pub resultcode: i32,
    pub reserved1: i32,
    pub reserved2: i32,
}
pub type RawRequest_t = floppy_raw_cmd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdf_info {
    pub FatSize: u32,
    pub RootDirSize: uint16_t,
    pub BadSectors: u32,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Xdf_t {
    pub head: Stream_t,
    pub fd: i32,
    pub buffer: *mut i8,
    pub track_valid: bool,
    pub current_track: uint8_t,
    pub map: *mut sector_map_t,
    pub track_size: uint32_t,
    pub track0_size: i32,
    pub sector_size: uint16_t,
    pub FatSize: uint8_t,
    pub RootDirSize: uint16_t,
    pub track_map: *mut TrackMap_t,
    pub last_sector: u8,
    pub rate: u8,
    #[bitfield(name = "stretch", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "rootskip", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "drive", ty = "libc::c_int", bits = "2..=5")]
    pub stretch_rootskip_drive: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
    pub postcmd: *const i8,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct TrackMap_t {
    pub begin: u8,
    pub end: u8,
    pub sector: u8,
    pub sizecode: u8,
    #[bitfield(name = "dirty", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "phantom", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "valid", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "head", ty = "libc::c_uint", bits = "4..=4")]
    pub dirty_phantom_valid_head: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type sector_map_t = sector_map;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct sector_map {
    #[bitfield(name = "head", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "size", ty = "libc::c_uint", bits = "1..=7")]
    pub head_size: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Compactify_t {
    pub head: u8,
    pub sector: u8,
    pub ptr: u8,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub track_size: u8,
    #[bitfield(name = "track0_size", ty = "libc::c_uint", bits = "0..=6")]
    #[bitfield(name = "rootskip", ty = "libc::c_uint", bits = "7..=7")]
    pub track0_size_rootskip: [u8; 1],
    pub rate: u8,
    pub map: [sector_map_t; 9],
}
#[inline]
unsafe extern "C" fn gnu_dev_major(mut __dev: __dev_t) -> u32 {
    let mut __major: u32 = 0;
    __major = ((__dev & 0xfff00 as u32 as __dev_t) >> 8 as i32) as u32;
    __major = (__major as u64 | (__dev & 0xfffff00000000000 as u64) >> 32 as i32) as u32;
    return __major;
}
#[inline]
unsafe extern "C" fn gnu_dev_minor(mut __dev: __dev_t) -> u32 {
    let mut __minor: u32 = 0;
    __minor = ((__dev & 0xff as u32 as __dev_t) >> 0 as i32) as u32;
    __minor = (__minor as u64 | (__dev & 0xffffff00000 as u64) >> 12 as i32) as u32;
    return __minor;
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn RR_SETDATA(mut request: *mut floppy_raw_cmd, mut data: caddr_t) {
    (*request).data = data as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn RR_INIT(mut request: *mut floppy_raw_cmd) {
    (*request).data = 0 as *mut libc::c_void;
    (*request).length = 0 as i32 as i64;
    (*request).cmd_count = 9 as i32 as u8;
    (*request).flags = (8 as i32 | 0x80 as i32 | 0x40 as i32) as u32;
    (*request).cmd[1 as i32 as usize] = 0 as i32 as u8;
    (*request).cmd[6 as i32 as usize] = 0 as i32 as u8;
    (*request).cmd[7 as i32 as usize] = 0x1b as i32 as u8;
    (*request).cmd[8 as i32 as usize] = 0xff as i32 as u8;
    (*request).reply_count = 0 as i32 as u8;
}
#[inline]
unsafe extern "C" fn RR_SETRATE(mut request: *mut floppy_raw_cmd, mut rate: uint8_t) {
    (*request).rate = rate;
}
#[inline]
unsafe extern "C" fn RR_SETPTRACK(mut request: *mut floppy_raw_cmd, mut track: i32) {
    (*request).track = track;
}
#[inline]
unsafe extern "C" fn RR_SETHEAD(mut request: *mut floppy_raw_cmd, mut head: uint8_t) {
    if head != 0 {
        (*request).cmd[1 as i32 as usize] = ((*request).cmd[1 as i32 as usize] as i32
            | 4 as i32) as u8;
    } else {
        (*request).cmd[1 as i32 as usize] = ((*request).cmd[1 as i32 as usize] as i32
            & !(4 as i32)) as u8;
    }
    (*request).cmd[3 as i32 as usize] = head;
}
#[inline]
unsafe extern "C" fn RR_SETSECTOR(
    mut request: *mut floppy_raw_cmd,
    mut sector: uint8_t,
) {
    (*request).cmd[4 as i32 as usize] = sector;
    (*request).cmd[6 as i32 as usize] = (sector as i32 - 1 as i32) as u8;
}
#[inline]
unsafe extern "C" fn RR_SETSIZECODE(
    mut request: *mut floppy_raw_cmd,
    mut sizecode: uint8_t,
) {
    (*request).cmd[5 as i32 as usize] = sizecode;
    (*request).cmd[6 as i32 as usize] = ((*request).cmd[6 as i32 as usize])
        .wrapping_add(1);
    (*request).cmd[6 as i32 as usize];
    (*request).length += ((128 as i32) << sizecode as i32) as i64;
}
#[inline]
unsafe extern "C" fn RR_SETTRACK(mut request: *mut floppy_raw_cmd, mut track: uint8_t) {
    (*request).cmd[2 as i32 as usize] = track;
}
#[inline]
unsafe extern "C" fn RR_SETDIRECTION(
    mut request: *mut floppy_raw_cmd,
    mut direction: i32,
) {
    if direction == 1 as i32 {
        (*request).flags |= 1 as i32 as u32;
        (*request).cmd[0 as i32 as usize] = (0xe6 as i32 & !(0x80 as i32)) as u8;
    } else {
        (*request).flags |= 2 as i32 as u32;
        (*request).cmd[0 as i32 as usize] = (0xc5 as i32 & !(0x80 as i32)) as u8;
    };
}
#[inline]
unsafe extern "C" fn RR_SETDRIVE(mut request: *mut floppy_raw_cmd, mut drive: i32) {
    (*request).cmd[1 as i32 as usize] = ((*request).cmd[1 as i32 as usize] as i32
        & !(3 as i32) | drive & 3 as i32) as u8;
}
#[inline]
unsafe extern "C" fn RR_SETCONT(mut request: *mut floppy_raw_cmd) {
    (*request).flags |= 0x100 as i32 as u32;
}
#[inline]
unsafe extern "C" fn RR_SIZECODE(mut request: *mut floppy_raw_cmd) -> i32 {
    return (*request).cmd[5 as i32 as usize] as i32;
}
#[inline]
unsafe extern "C" fn GET_DRIVE(mut fd: i32) -> i32 {
    let mut statbuf: stat = stat {
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
    if fstat(fd, &mut statbuf) < 0 as i32 {
        perror(b"stat\0" as *const u8 as *const i8);
        return -(1 as i32);
    }
    if !(statbuf.st_mode & 0o170000 as i32 as u32 == 0o60000 as i32 as u32)
        || gnu_dev_major(statbuf.st_rdev) != 2 as i32 as u32
    {
        return -(1 as i32);
    }
    return gnu_dev_minor(statbuf.st_rdev) as i32;
}
static mut xdf_table: [C2RustUnnamed_0; 5] = [C2RustUnnamed_0 {
    track_size: 0,
    track0_size_rootskip: [0; 1],
    rate: 0,
    map: [sector_map_t {
        head_size: [0; 1],
        c2rust_padding: [0; 3],
    }; 9],
}; 5];
unsafe extern "C" fn analyze_reply(
    mut raw_cmd: *mut RawRequest_t,
    mut do_print: i32,
) -> i32 {
    let mut ret: i32 = 0;
    let mut bytes: i32 = 0;
    let mut newbytes: i32 = 0;
    bytes = 0 as i32;
    loop {
        ret = analyze_one_reply(raw_cmd, &mut newbytes, do_print);
        bytes += newbytes;
        match ret {
            0 => return bytes,
            1 => {
                raw_cmd = raw_cmd.offset(1);
                raw_cmd;
            }
            -1 => if bytes != 0 { return bytes } else { return 0 as i32 }
            _ => {}
        }
    };
}
unsafe extern "C" fn send_cmd(
    mut fd: i32,
    mut raw_cmd: *mut RawRequest_t,
    mut nr: i32,
    mut message: *const i8,
    mut retries: i32,
) -> i32 {
    let mut j: i32 = 0;
    let mut ret: i32 = -(1 as i32);
    if nr == 0 {
        return 0 as i32;
    }
    j = 0 as i32;
    while j < retries {
        match send_one_cmd(fd, raw_cmd, message) {
            -1 => return -(1 as i32),
            1 => {
                j += 1;
                j;
            }
            0 | _ => {
                ret = analyze_reply(raw_cmd, j);
                if ret > 0 as i32 {
                    return ret;
                }
            }
        }
        j += 1;
        j;
    }
    if j > 1 as i32 && j == retries {
        fprintf(stderr, b"Too many errors, giving up\n\0" as *const u8 as *const i8);
        return 0 as i32;
    }
    return -(1 as i32);
}
unsafe extern "C" fn add_to_request(
    mut This: *mut Xdf_t,
    mut ptr: u8,
    mut request: *mut RawRequest_t,
    mut nr: *mut i32,
    mut direction: i32,
    mut compactify: *mut Compactify_t,
) -> i32 {
    if (*((*This).track_map).offset(ptr as isize)).phantom() != 0 {
        if direction == 1 as i32 {
            memset(
                ((*This).buffer)
                    .offset((ptr as i32 * (*This).sector_size as i32) as isize)
                    as *mut libc::c_void,
                0 as i32,
                ((128 as u32)
                    << (*((*This).track_map).offset(ptr as isize)).sizecode as i32)
                    as u64,
            );
        }
        return 0 as i32;
    }
    if *nr != 0
        && RR_SIZECODE(request.offset(*nr as isize).offset(-(1 as i32 as isize)))
            == (*((*This).track_map).offset(ptr as isize)).sizecode as i32
        && (*compactify).head as i32
            == (*((*This).track_map).offset(ptr as isize)).head() as i32
        && (*compactify).ptr as i32 + 1 as i32 == ptr as i32
        && (*compactify).sector as i32 + 1 as i32
            == (*((*This).track_map).offset(ptr as isize)).sector as i32
    {
        RR_SETSIZECODE(
            request.offset(*nr as isize).offset(-(1 as i32 as isize)),
            (*((*This).track_map).offset(ptr as isize)).sizecode,
        );
    } else {
        if *nr != 0 {
            RR_SETCONT(request.offset(*nr as isize).offset(-(1 as i32 as isize)));
        }
        RR_INIT(request.offset(*nr as isize));
        RR_SETDRIVE(request.offset(*nr as isize), (*This).drive());
        RR_SETRATE(request.offset(*nr as isize), (*This).rate);
        RR_SETTRACK(request.offset(*nr as isize), (*This).current_track);
        RR_SETPTRACK(
            request.offset(*nr as isize),
            ((*This).current_track as i32) << (*This).stretch() as i32,
        );
        RR_SETHEAD(
            request.offset(*nr as isize),
            (*((*This).track_map).offset(ptr as isize)).head() as uint8_t,
        );
        RR_SETSECTOR(
            request.offset(*nr as isize),
            (*((*This).track_map).offset(ptr as isize)).sector,
        );
        RR_SETSIZECODE(
            request.offset(*nr as isize),
            (*((*This).track_map).offset(ptr as isize)).sizecode,
        );
        RR_SETDIRECTION(request.offset(*nr as isize), direction);
        RR_SETDATA(
            request.offset(*nr as isize),
            ((*This).buffer).offset((ptr as i32 * (*This).sector_size as i32) as isize),
        );
        *nr += 1;
        *nr;
    }
    (*compactify).ptr = ptr;
    (*compactify).head = (*((*This).track_map).offset(ptr as isize)).head() as u8;
    (*compactify).sector = (*((*This).track_map).offset(ptr as isize)).sector;
    return 0 as i32;
}
unsafe extern "C" fn add_to_request_if_invalid(
    mut This: *mut Xdf_t,
    mut ptr: u8,
    mut request: *mut RawRequest_t,
    mut nr: *mut i32,
    mut compactify: *mut Compactify_t,
) {
    if (*((*This).track_map).offset(ptr as isize)).valid() == 0 {
        add_to_request(This, ptr, request, nr, 1 as i32, compactify);
    }
}
unsafe extern "C" fn adjust_bounds(
    mut This: *mut Xdf_t,
    mut ibegin: uint32_t,
    mut iend: uint32_t,
    mut begin: *mut uint8_t,
    mut end: *mut uint8_t,
) {
    *begin = ibegin.wrapping_div((*This).sector_size as u32) as uint8_t;
    *end = iend
        .wrapping_add((*This).sector_size as u32)
        .wrapping_sub(1 as i32 as u32)
        .wrapping_div((*This).sector_size as u32) as uint8_t;
}
#[inline]
unsafe extern "C" fn try_flush_dirty(mut This: *mut Xdf_t) -> i32 {
    let mut ptr: u8 = 0;
    let mut nr: i32 = 0;
    let mut bytes: i32 = 0;
    let mut requests: [RawRequest_t; 100] = [RawRequest_t {
        flags: 0,
        data: 0 as *mut libc::c_void,
        kernel_data: 0 as *mut i8,
        next: 0 as *mut floppy_raw_cmd,
        length: 0,
        phys_length: 0,
        buffer_length: 0,
        rate: 0,
        cmd_count: 0,
        cmd: [0; 16],
        reply_count: 0,
        reply: [0; 16],
        track: 0,
        resultcode: 0,
        reserved1: 0,
        reserved2: 0,
    }; 100];
    let mut compactify: Compactify_t = Compactify_t {
        head: 0,
        sector: 0,
        ptr: 0,
    };
    if !(*This).track_valid {
        return 0 as i32;
    }
    nr = 0 as i32;
    ptr = 0 as i32 as u8;
    while (ptr as i32) < (*This).last_sector as i32 {
        if (*((*This).track_map).offset(ptr as isize)).dirty() != 0 {
            add_to_request(
                This,
                ptr,
                requests.as_mut_ptr(),
                &mut nr,
                2 as i32,
                &mut compactify,
            );
        }
        ptr = (*((*This).track_map).offset(ptr as isize)).end;
    }
    bytes = send_cmd(
        (*This).fd,
        requests.as_mut_ptr(),
        nr,
        b"writing\0" as *const u8 as *const i8,
        4 as i32,
    );
    if bytes < 0 as i32 {
        return bytes;
    }
    ptr = 0 as i32 as u8;
    while (ptr as i32) < (*This).last_sector as i32 {
        if (*((*This).track_map).offset(ptr as isize)).dirty() != 0 {
            if bytes
                >= (*((*This).track_map).offset(ptr as isize)).end as i32
                    - (*((*This).track_map).offset(ptr as isize)).begin as i32
            {
                bytes
                    -= (*((*This).track_map).offset(ptr as isize)).end as i32
                        - (*((*This).track_map).offset(ptr as isize)).begin as i32;
                let ref mut fresh0 = *((*This).track_map).offset(ptr as isize);
                (*fresh0).set_dirty(0 as i32 as u32);
            } else {
                return 1 as i32
            }
        }
        ptr = (*((*This).track_map).offset(ptr as isize)).end;
    }
    return 0 as i32;
}
unsafe extern "C" fn flush_dirty(mut This: *mut Xdf_t) -> i32 {
    let mut ret: i32 = 0;
    loop {
        ret = try_flush_dirty(This);
        if !(ret != 0) {
            break;
        }
        if ret < 0 as i32 {
            return ret;
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn load_data(
    mut This: *mut Xdf_t,
    mut ibegin: uint32_t,
    mut iend: uint32_t,
    mut retries: i32,
) -> ssize_t {
    let mut ptr: u8 = 0;
    let mut nr: i32 = 0;
    let mut bytes: i32 = 0;
    let mut requests: [RawRequest_t; 100] = [RawRequest_t {
        flags: 0,
        data: 0 as *mut libc::c_void,
        kernel_data: 0 as *mut i8,
        next: 0 as *mut floppy_raw_cmd,
        length: 0,
        phys_length: 0,
        buffer_length: 0,
        rate: 0,
        cmd_count: 0,
        cmd: [0; 16],
        reply_count: 0,
        reply: [0; 16],
        track: 0,
        resultcode: 0,
        reserved1: 0,
        reserved2: 0,
    }; 100];
    let mut compactify: Compactify_t = Compactify_t {
        head: 0,
        sector: 0,
        ptr: 0,
    };
    let mut begin: u8 = 0;
    let mut end: u8 = 0;
    adjust_bounds(This, ibegin, iend, &mut begin, &mut end);
    ptr = begin;
    nr = 0 as i32;
    ptr = (*((*This).track_map).offset(ptr as isize)).begin;
    while (ptr as i32) < end as i32 {
        add_to_request_if_invalid(
            This,
            ptr,
            requests.as_mut_ptr(),
            &mut nr,
            &mut compactify,
        );
        ptr = (*((*This).track_map).offset(ptr as isize)).end;
    }
    bytes = send_cmd(
        (*This).fd,
        requests.as_mut_ptr(),
        nr,
        b"reading\0" as *const u8 as *const i8,
        retries,
    );
    if bytes < 0 as i32 {
        return bytes as ssize_t;
    }
    ptr = begin;
    ptr = (*((*This).track_map).offset(ptr as isize)).begin;
    while (ptr as i32) < end as i32 {
        if (*((*This).track_map).offset(ptr as isize)).valid() == 0 {
            if bytes
                >= (*((*This).track_map).offset(ptr as isize)).end as i32
                    - (*((*This).track_map).offset(ptr as isize)).begin as i32
            {
                bytes
                    -= (*((*This).track_map).offset(ptr as isize)).end as i32
                        - (*((*This).track_map).offset(ptr as isize)).begin as i32;
                let ref mut fresh1 = *((*This).track_map).offset(ptr as isize);
                (*fresh1).set_valid(1 as i32 as u32);
            } else if ptr as i32 > begin as i32 {
                return (ptr as i32 * (*This).sector_size as i32) as ssize_t
            } else {
                return -(1 as i32) as ssize_t
            }
        }
        ptr = (*((*This).track_map).offset(ptr as isize)).end;
    }
    return (end as i32 * (*This).sector_size as i32) as ssize_t;
}
unsafe extern "C" fn mark_dirty(
    mut This: *mut Xdf_t,
    mut ibegin: uint32_t,
    mut iend: uint32_t,
) {
    let mut ptr: i32 = 0;
    let mut begin: u8 = 0;
    let mut end: u8 = 0;
    adjust_bounds(This, ibegin, iend, &mut begin, &mut end);
    ptr = begin as i32;
    ptr = (*((*This).track_map).offset(ptr as isize)).begin as i32;
    while ptr < end as i32 {
        let ref mut fresh2 = *((*This).track_map).offset(ptr as isize);
        (*fresh2).set_valid(1 as i32 as u32);
        if (*((*This).track_map).offset(ptr as isize)).phantom() == 0 {
            let ref mut fresh3 = *((*This).track_map).offset(ptr as isize);
            (*fresh3).set_dirty(1 as i32 as u32);
        }
        ptr = (*((*This).track_map).offset(ptr as isize)).end as i32;
    }
}
unsafe extern "C" fn load_bounds(
    mut This: *mut Xdf_t,
    mut begin: uint32_t,
    mut end: uint32_t,
) -> ssize_t {
    let mut lbegin: u8 = 0;
    let mut lend: u8 = 0;
    adjust_bounds(This, begin, end, &mut lbegin, &mut lend);
    if begin
        != ((*((*This).track_map).offset(lbegin as isize)).begin as i32
            * (*This).sector_size as i32) as u32
        && end
            != ((*((*This).track_map).offset(lend as isize)).begin as i32
                * (*This).sector_size as i32) as u32
        && (lend as i32)
            < (*((*This).track_map)
                .offset((*((*This).track_map).offset(lbegin as isize)).end as isize))
                .end as i32
    {
        return load_data(This, begin, end, 4 as i32);
    }
    if begin
        != ((*((*This).track_map).offset(lbegin as isize)).begin as i32
            * (*This).sector_size as i32) as u32
    {
        let mut ret: ssize_t = load_data(This, begin, begin, 4 as i32);
        if ret < 0 as i32 as i64 {
            return ret;
        }
    }
    if end
        != ((*((*This).track_map).offset(lend as isize)).begin as i32
            * (*This).sector_size as i32) as u32
    {
        let mut ret_0: ssize_t = load_data(This, end, end, 4 as i32);
        if ret_0 < 0 as i32 as i64 {
            return ((*((*This).track_map).offset(lend as isize)).begin as i32
                * (*This).sector_size as i32) as ssize_t;
        }
    }
    return (lend as i32 * (*This).sector_size as i32) as ssize_t;
}
unsafe extern "C" fn fill_boot(mut This: *mut Xdf_t) {
    let mut ptr: uint8_t = 0 as i32 as uint8_t;
    let ref mut fresh4 = *((*This).track_map).offset(ptr as isize);
    (*fresh4).set_head(0 as i32 as u32);
    (*((*This).track_map).offset(ptr as isize)).sector = 129 as i32 as u8;
    let ref mut fresh5 = *((*This).track_map).offset(ptr as isize);
    (*fresh5).set_phantom(0 as i32 as u32);
    (*((*This).track_map).offset(ptr as isize)).begin = ptr;
    (*((*This).track_map).offset(ptr as isize)).end = (ptr as i32 + 1 as i32) as u8;
    (*((*This).track_map).offset(ptr as isize)).sizecode = 2 as i32 as u8;
    let ref mut fresh6 = *((*This).track_map).offset(ptr as isize);
    (*fresh6).set_valid(0 as i32 as u32);
    let ref mut fresh7 = *((*This).track_map).offset(ptr as isize);
    (*fresh7).set_dirty(0 as i32 as u32);
    (*This).last_sector = 1 as i32 as u8;
    (*This).current_track = 0 as i32 as uint8_t;
}
unsafe extern "C" fn fill_t0(
    mut This: *mut Xdf_t,
    mut ptr: uint8_t,
    mut size: u32,
    mut sector: *mut uint8_t,
    mut head: *mut uint8_t,
) -> uint8_t {
    let mut n: u32 = 0;
    n = 0 as i32 as u32;
    while n < size {
        let ref mut fresh8 = *((*This).track_map).offset(ptr as isize);
        (*fresh8).set_head(*head as u32);
        (*((*This).track_map).offset(ptr as isize)).sector = (*sector as i32
            + 129 as i32) as u8;
        let ref mut fresh9 = *((*This).track_map).offset(ptr as isize);
        (*fresh9).set_phantom(0 as i32 as u32);
        *sector = (*sector).wrapping_add(1);
        *sector;
        if *head == 0 && *sector as i32 >= (*This).track0_size - 8 as i32 {
            *sector = 0 as i32 as uint8_t;
            *head = 1 as i32 as uint8_t;
        }
        ptr = ptr.wrapping_add(1);
        ptr;
        n = n.wrapping_add(1);
        n;
    }
    return ptr;
}
unsafe extern "C" fn fill_phantoms(
    mut This: *mut Xdf_t,
    mut ptr: uint8_t,
    mut size: uint8_t,
) -> uint8_t {
    let mut n: u32 = 0;
    n = 0 as i32 as u32;
    while n < size as u32 {
        let ref mut fresh10 = *((*This).track_map).offset(ptr as isize);
        (*fresh10).set_phantom(1 as i32 as u32);
        ptr = ptr.wrapping_add(1);
        ptr;
        n = n.wrapping_add(1);
        n;
    }
    return ptr;
}
unsafe extern "C" fn decompose(
    mut This: *mut Xdf_t,
    mut iwhere: mt_off_t,
    mut len: size_t,
    mut begin: *mut uint32_t,
    mut end: *mut uint32_t,
    mut boot: uint8_t,
) -> i32 {
    let mut ptr: uint8_t = 0;
    let mut map: *mut sector_map_t = 0 as *mut sector_map_t;
    let mut lbegin: uint8_t = 0;
    let mut lend: uint8_t = 0;
    let mut track_size: uint32_t = ((*This).track_size).wrapping_mul(1024 as i32 as u32);
    let mut track: smt_off_t = iwhere / track_size as i64;
    let mut where_0: uint32_t = (iwhere % track_size as i64) as uint32_t;
    *begin = where_0;
    if (where_0 as u64).wrapping_add(len) > track_size as u64 {
        *end = track_size;
    } else {
        *end = (where_0 as u64).wrapping_add(len) as uint32_t;
    }
    if (*This).current_track as i64 == track && boot == 0 {
        return 0 as i32;
    }
    if boot == 0 {
        flush_dirty(This);
    }
    if track >= 80 as i32 as i64 {
        return -(1 as i32);
    }
    (*This).current_track = track as uint8_t;
    (*This).track_valid = 1 as i32 != 0;
    if track != 0 {
        ptr = 0 as i32 as uint8_t;
        map = (*This).map;
        while (*map).size() != 0 {
            lbegin = ptr;
            lend = (ptr as i32
                + ((128 as u32) << (*map).size() as i32)
                    .wrapping_div((*This).sector_size as u32) as uint8_t as i32)
                as uint8_t;
            while (ptr as i32) < lend as i32 {
                (*((*This).track_map).offset(ptr as isize)).begin = lbegin;
                (*((*This).track_map).offset(ptr as isize)).end = lend;
                let ref mut fresh11 = *((*This).track_map).offset(ptr as isize);
                (*fresh11).set_head((*map).head());
                (*((*This).track_map).offset(ptr as isize)).sector = ((*map).size()
                    as i32 + 128 as i32) as u8;
                (*((*This).track_map).offset(ptr as isize)).sizecode = (*map).size()
                    as u8;
                let ref mut fresh12 = *((*This).track_map).offset(ptr as isize);
                (*fresh12).set_valid(0 as i32 as u32);
                let ref mut fresh13 = *((*This).track_map).offset(ptr as isize);
                (*fresh13).set_dirty(0 as i32 as u32);
                let ref mut fresh14 = *((*This).track_map).offset(ptr as isize);
                (*fresh14).set_phantom(0 as i32 as u32);
                ptr = ptr.wrapping_add(1);
                ptr;
            }
            map = map.offset(1);
            map;
        }
        let ref mut fresh15 = (*((*This).track_map).offset(ptr as isize)).end;
        *fresh15 = ptr;
        (*((*This).track_map).offset(ptr as isize)).begin = *fresh15;
    } else {
        let mut sector: uint8_t = 0;
        let mut head: uint8_t = 0;
        head = 0 as i32 as uint8_t;
        sector = 0 as i32 as uint8_t;
        ptr = boot;
        while (ptr as u32) < (2 as i32 as u32).wrapping_mul((*This).track_size) {
            (*((*This).track_map).offset(ptr as isize)).begin = ptr;
            (*((*This).track_map).offset(ptr as isize)).end = (ptr as i32 + 1 as i32)
                as u8;
            (*((*This).track_map).offset(ptr as isize)).sizecode = 2 as i32 as u8;
            let ref mut fresh16 = *((*This).track_map).offset(ptr as isize);
            (*fresh16).set_valid(0 as i32 as u32);
            let ref mut fresh17 = *((*This).track_map).offset(ptr as isize);
            (*fresh17).set_dirty(0 as i32 as u32);
            ptr = ptr.wrapping_add(1);
            ptr;
        }
        ptr = fill_t0(
            This,
            0 as i32 as uint8_t,
            (1 as i32 + (*This).FatSize as i32) as u32,
            &mut sector,
            &mut head,
        );
        ptr = fill_phantoms(This, ptr, (*This).FatSize);
        ptr = fill_t0(This, ptr, (*This).RootDirSize as u32, &mut sector, &mut head);
        ptr = fill_phantoms(This, ptr, 5 as i32 as uint8_t);
        if (*This).rootskip() != 0 {
            sector = sector.wrapping_add(1);
            sector;
        }
        ptr = fill_t0(
            This,
            ptr,
            ((*This).track_size)
                .wrapping_sub((*This).FatSize as u32)
                .wrapping_mul(2 as i32 as u32)
                .wrapping_sub((*This).RootDirSize as u32)
                .wrapping_sub(6 as i32 as u32),
            &mut sector,
            &mut head,
        );
    }
    (*This).last_sector = ptr;
    return 0 as i32;
}
unsafe extern "C" fn xdf_pread(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut where_0: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut begin: uint32_t = 0;
    let mut end: uint32_t = 0;
    let mut ret: ssize_t = 0;
    let mut This: *mut Xdf_t = Stream as *mut Xdf_t;
    if decompose(
        This,
        truncBytes32(where_0),
        len,
        &mut begin,
        &mut end,
        0 as i32 as uint8_t,
    ) < 0 as i32
    {
        return 0 as i32 as ssize_t;
    }
    ret = load_data(This, begin, end, 4 as i32);
    if ret < 0 as i32 as i64 || (ret as size_t) < begin as u64 {
        return -(1 as i32) as ssize_t;
    }
    if len > (ret as size_t).wrapping_sub(begin as u64) {
        len = (ret as size_t).wrapping_sub(begin as u64);
    }
    memcpy(
        buf as *mut libc::c_void,
        ((*This).buffer).offset(begin as isize) as *const libc::c_void,
        len,
    );
    return end.wrapping_sub(begin) as ssize_t;
}
unsafe extern "C" fn xdf_pwrite(
    mut Stream: *mut Stream_t,
    mut buf: *mut i8,
    mut where_0: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut begin: uint32_t = 0;
    let mut end: uint32_t = 0;
    let mut len2: ssize_t = 0;
    let mut This: *mut Xdf_t = Stream as *mut Xdf_t;
    if decompose(
        This,
        truncBytes32(where_0),
        len,
        &mut begin,
        &mut end,
        0 as i32 as uint8_t,
    ) < 0 as i32
    {
        *__errno_location() = 27 as i32;
        return -(1 as i32) as ssize_t;
    }
    len2 = load_bounds(This, begin, end);
    if len2 < 0 as i32 as i64 {
        return -(1 as i32) as ssize_t;
    }
    if end > len2 as uint32_t {
        end = len2 as uint32_t;
    }
    len2 -= begin as i64;
    if len > len2 as size_t {
        len = len2 as size_t;
    }
    memcpy(
        ((*This).buffer).offset(begin as isize) as *mut libc::c_void,
        buf as *const libc::c_void,
        len,
    );
    mark_dirty(This, begin, end);
    return end.wrapping_sub(begin) as ssize_t;
}
unsafe extern "C" fn xdf_flush(mut Stream: *mut Stream_t) -> i32 {
    let mut This: *mut Xdf_t = Stream as *mut Xdf_t;
    return flush_dirty(This);
}
unsafe extern "C" fn xdf_free(mut Stream: *mut Stream_t) -> i32 {
    let mut ret: i32 = 0;
    let mut This: *mut Xdf_t = Stream as *mut Xdf_t;
    free((*This).track_map as *mut i8 as *mut libc::c_void);
    free((*This).buffer as *mut libc::c_void);
    ret = close((*This).fd);
    postcmd((*This).postcmd);
    return ret;
}
unsafe extern "C" fn check_geom(mut This: *mut Xdf_t, mut dev: *mut device) -> i32 {
    let mut sect: u32 = 0;
    if !(!dev.is_null() && (*dev).misc_flags & 0x10 as u32 != 0) {
        if (*dev).sectors as i32 != 0 && (*dev).sectors as i32 != 19 as i32
            && ((*dev).sectors as i32 != 0 && (*dev).sectors as i32 != 23 as i32)
            && ((*dev).sectors as i32 != 0 && (*dev).sectors as i32 != 24 as i32)
            && ((*dev).sectors as i32 != 0 && (*dev).sectors as i32 != 46 as i32)
            && ((*dev).sectors as i32 != 0 && (*dev).sectors as i32 != 48 as i32)
        {
            return 1 as i32;
        }
        if (*dev).heads as i32 != 0 && (*dev).heads as i32 != 2 as i32 {
            return 1 as i32;
        }
    }
    if !This.is_null() {
        sect = (*This).track_size;
        if sect != 19 as i32 as u32 && sect != 23 as i32 as u32
            && sect != 24 as i32 as u32 && sect != 46 as i32 as u32
            && sect != 48 as i32 as u32
            || !(!dev.is_null() && (*dev).misc_flags & 0x10 as u32 != 0)
                && ((*dev).sectors as i32 != 0 && (*dev).sectors as u32 != sect)
        {
            return 1 as i32;
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn set_geom(mut This: *mut Xdf_t, mut dev: *mut device) {
    (*dev).heads = 2 as i32 as uint16_t;
    (*dev).use_2m = 0xff as i32 as u32;
    (*dev).sectors = (*This).track_size as uint16_t;
    (*dev).tracks = 80 as i32 as u32;
}
unsafe extern "C" fn config_geom(
    mut Stream: *mut Stream_t,
    mut dev: *mut device,
    mut orig_dev: *mut device,
) -> i32 {
    let mut This: *mut Xdf_t = Stream as *mut Xdf_t;
    if check_geom(This, dev) != 0 {
        return 1 as i32;
    }
    set_geom(This, dev);
    return 0 as i32;
}
static mut XdfClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: None,
            write: None,
            pread: Some(
                xdf_pread
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                xdf_pwrite
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            flush: Some(xdf_flush as unsafe extern "C" fn(*mut Stream_t) -> i32),
            freeFunc: Some(xdf_free as unsafe extern "C" fn(*mut Stream_t) -> i32),
            set_geom: Some(
                config_geom
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut device,
                        *mut device,
                    ) -> i32,
            ),
            get_data: None,
            pre_allocate: None,
            get_dosConvert: None,
            discard: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn XdfOpen(
    mut dev: *mut device,
    mut name: *const i8,
    mut mode: i32,
    mut errmsg: *mut i8,
    mut info: *mut xdf_info,
) -> *mut Stream_t {
    let mut current_block: u64;
    let mut This: *mut Xdf_t = 0 as *mut Xdf_t;
    let mut begin: uint32_t = 0;
    let mut end: uint32_t = 0;
    let mut boot: *mut bootsector = 0 as *mut bootsector;
    let mut type_0: u32 = 0;
    let mut fatSize: uint16_t = 0;
    if !dev.is_null()
        && (!(!dev.is_null() && (*dev).misc_flags & 0x8 as u32 != 0)
            && (getenv(b"MTOOLS_USE_XDF\0" as *const u8 as *const i8)).is_null()
            || check_geom(0 as *mut Xdf_t, dev) != 0)
    {
        return 0 as *mut Stream_t;
    }
    This = calloc(1 as i32 as u64, ::core::mem::size_of::<Xdf_t>() as u64) as *mut Xdf_t;
    if This.is_null() {
        return 0 as *mut Stream_t;
    }
    init_head(&mut (*This).head, &mut XdfClass, 0 as *mut Stream_t);
    (*This).sector_size = 512 as i32 as uint16_t;
    (*This).set_stretch(0 as i32 as u32);
    (*This).postcmd = 0 as *const i8;
    precmd(dev);
    if !dev.is_null() {
        (*This).postcmd = (*dev).postcmd;
    }
    (*This).fd = open(
        name,
        (mode | (*dev).mode) & !(0o3 as i32) | 0o200 as i32 | 0o4000 as i32 | 0o2 as i32,
    );
    if (*This).fd < 0 as i32 {
        snprintf(
            errmsg,
            199 as i32 as u64,
            b"xdf floppy: open: \"%s\"\0" as *const u8 as *const i8,
            strerror(*__errno_location()),
        );
    } else {
        closeExec((*This).fd);
        (*This).set_drive(GET_DRIVE((*This).fd));
        if !((*This).drive() < 0 as i32) {
            (*This).buffer = malloc((96 as i32 * 512 as i32) as u64) as *mut i8;
            if !((*This).buffer).is_null() {
                (*This).track_valid = 0 as i32 != 0;
                (*This).track_map = calloc(
                    96 as i32 as u64,
                    ::core::mem::size_of::<TrackMap_t>() as u64,
                ) as *mut TrackMap_t;
                if !((*This).track_map).is_null() {
                    if lock_dev((*This).fd, (mode == 0o2 as i32) as i32, dev) != 0 {
                        snprintf(
                            errmsg,
                            199 as i32 as u64,
                            b"xdf floppy: device \"%s\" busy:\0" as *const u8
                                as *const i8,
                            (*dev).name,
                        );
                    } else {
                        fill_boot(This);
                        (*This).rate = 0 as i32 as u8;
                        if load_data(
                            This,
                            0 as i32 as uint32_t,
                            1 as i32 as uint32_t,
                            4 as i32,
                        ) < 0 as i32 as i64
                        {
                            (*This).rate = 0x43 as i32 as u8;
                            if load_data(
                                This,
                                0 as i32 as uint32_t,
                                1 as i32 as uint32_t,
                                4 as i32,
                            ) < 0 as i32 as i64
                            {
                                current_block = 14655808593877514425;
                            } else {
                                current_block = 17478428563724192186;
                            }
                        } else {
                            current_block = 17478428563724192186;
                        }
                        match current_block {
                            14655808593877514425 => {}
                            _ => {
                                boot = (*This).buffer as *mut bootsector;
                                fatSize = ((*boot).boot.fatlen[0 as i32 as usize] as i32
                                    + (((*boot).boot.fatlen[1 as i32 as usize] as i32)
                                        << 8 as i32)) as uint16_t;
                                if fatSize as i32 > 255 as i32 {
                                    fprintf(
                                        stderr,
                                        b"Fat size %d too large\n\0" as *const u8 as *const i8,
                                        fatSize as i32,
                                    );
                                    exit(1 as i32);
                                }
                                (*This).FatSize = fatSize as uint8_t;
                                (*This).RootDirSize = (((*boot)
                                    .boot
                                    .dirents[0 as i32 as usize] as i32
                                    + (((*boot).boot.dirents[1 as i32 as usize] as i32)
                                        << 8 as i32)) as uint16_t as i32 / 16 as i32) as uint16_t;
                                (*This).track_size = ((*boot).boot.nsect[0 as i32 as usize]
                                    as i32
                                    + (((*boot).boot.nsect[1 as i32 as usize] as i32)
                                        << 8 as i32)) as uint16_t as uint32_t;
                                type_0 = 0 as i32 as u32;
                                while (type_0 as u64)
                                    < (::core::mem::size_of::<[C2RustUnnamed_0; 5]>() as u64)
                                        .wrapping_div(
                                            ::core::mem::size_of::<C2RustUnnamed_0>() as u64,
                                        )
                                {
                                    if xdf_table[type_0 as usize].track_size as u32
                                        == (*This).track_size
                                    {
                                        (*This).map = (xdf_table[type_0 as usize].map).as_mut_ptr();
                                        (*This).track0_size = (xdf_table[type_0 as usize])
                                            .track0_size() as i32;
                                        (*This)
                                            .set_rootskip((xdf_table[type_0 as usize]).rootskip());
                                        (*This).rate = xdf_table[type_0 as usize].rate;
                                        break;
                                    } else {
                                        type_0 = type_0.wrapping_add(1);
                                        type_0;
                                    }
                                }
                                if !(type_0 as u64
                                    == (::core::mem::size_of::<[C2RustUnnamed_0; 5]>() as u64)
                                        .wrapping_div(
                                            ::core::mem::size_of::<C2RustUnnamed_0>() as u64,
                                        ))
                                {
                                    if !info.is_null() {
                                        (*info).RootDirSize = (*This).RootDirSize;
                                        (*info).FatSize = (*This).FatSize as u32;
                                        (*info).BadSectors = 5 as i32 as u32;
                                    }
                                    decompose(
                                        This,
                                        0 as i32 as mt_off_t,
                                        512 as i32 as size_t,
                                        &mut begin,
                                        &mut end,
                                        1 as i32 as uint8_t,
                                    );
                                    if !dev.is_null() {
                                        set_geom(This, dev);
                                    }
                                    return &mut (*This).head;
                                }
                            }
                        }
                    }
                    free((*This).track_map as *mut i8 as *mut libc::c_void);
                }
                free((*This).buffer as *mut libc::c_void);
            }
        }
        close((*This).fd);
    }
    free(This as *mut i8 as *mut libc::c_void);
    return 0 as *mut Stream_t;
}
unsafe extern "C" fn run_static_initializers() {
    xdf_table = [
        {
            let mut init = C2RustUnnamed_0 {
                track0_size_rootskip: [0; 1],
                track_size: 19 as i32 as u8,
                rate: 0 as i32 as u8,
                map: [
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as i32 as u32);
                        init.set_size(3 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as i32 as u32);
                        init.set_size(6 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as i32 as u32);
                        init.set_size(2 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as i32 as u32);
                        init.set_size(2 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as i32 as u32);
                        init.set_size(6 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as i32 as u32);
                        init.set_size(3 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as i32 as u32);
                        init.set_size(0 as i32 as u32);
                        init
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                ],
            };
            init.set_track0_size(16 as i32 as u32);
            init.set_rootskip(0 as i32 as u32);
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                track0_size_rootskip: [0; 1],
                track_size: 23 as i32 as u8,
                rate: 0 as i32 as u8,
                map: [
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as i32 as u32);
                        init.set_size(3 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as i32 as u32);
                        init.set_size(4 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as i32 as u32);
                        init.set_size(6 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as i32 as u32);
                        init.set_size(2 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as i32 as u32);
                        init.set_size(2 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as i32 as u32);
                        init.set_size(6 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as i32 as u32);
                        init.set_size(4 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as i32 as u32);
                        init.set_size(3 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as i32 as u32);
                        init.set_size(0 as i32 as u32);
                        init
                    },
                ],
            };
            init.set_track0_size(19 as i32 as u32);
            init.set_rootskip(0 as i32 as u32);
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                track0_size_rootskip: [0; 1],
                track_size: 46 as i32 as u8,
                rate: 0x43 as i32 as u8,
                map: [
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as i32 as u32);
                        init.set_size(3 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as i32 as u32);
                        init.set_size(4 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as i32 as u32);
                        init.set_size(5 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as i32 as u32);
                        init.set_size(7 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as i32 as u32);
                        init.set_size(3 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as i32 as u32);
                        init.set_size(4 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as i32 as u32);
                        init.set_size(5 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as i32 as u32);
                        init.set_size(7 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as i32 as u32);
                        init.set_size(0 as i32 as u32);
                        init
                    },
                ],
            };
            init.set_track0_size(37 as i32 as u32);
            init.set_rootskip(1 as i32 as u32);
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                track0_size_rootskip: [0; 1],
                track_size: 24 as i32 as u8,
                rate: 0 as i32 as u8,
                map: [
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as i32 as u32);
                        init.set_size(5 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as i32 as u32);
                        init.set_size(6 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as i32 as u32);
                        init.set_size(6 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as i32 as u32);
                        init.set_size(5 as i32 as u32);
                        init
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                ],
            };
            init.set_track0_size(20 as i32 as u32);
            init.set_rootskip(1 as i32 as u32);
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                track0_size_rootskip: [0; 1],
                track_size: 48 as i32 as u8,
                rate: 0 as i32 as u8,
                map: [
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as i32 as u32);
                        init.set_size(6 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as i32 as u32);
                        init.set_size(7 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(0 as i32 as u32);
                        init.set_size(7 as i32 as u32);
                        init
                    },
                    {
                        let mut init = sector_map {
                            head_size: [0; 1],
                            c2rust_padding: [0; 3],
                        };
                        init.set_head(1 as i32 as u32);
                        init.set_size(6 as i32 as u32);
                        init
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                    sector_map_t {
                        head_size: [0; 1],
                        c2rust_padding: [0; 3],
                    },
                ],
            };
            init.set_track0_size(41 as i32 as u32);
            init.set_rootskip(1 as i32 as u32);
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
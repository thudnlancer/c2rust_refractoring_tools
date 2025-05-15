use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type doscp_t;
    pub type FatMap_t;
    fn random() -> i64;
    fn srandom(__seed: u32);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn exit(_: i32) -> !;
    static mut stderr: *mut _IO_FILE;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn perror(__s: *const i8);
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn getopt(___argc: i32, ___argv: *const *mut i8, __shortopts: *const i8) -> i32;
    fn time(__timer: *mut time_t) -> time_t;
    fn __errno_location() -> *mut i32;
    fn strdup(_: *const i8) -> *mut i8;
    fn strspn(_: *const i8, _: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    static mut got_signal: i32;
    fn GetFs(Fs: *mut Stream_t) -> *mut Stream_t;
    fn set_cmd_line_image(img: *mut i8);
    fn atoui(nptr: *const i8) -> u32;
    fn strtou32(nptr: *const i8, endptr: *mut *mut i8, base: i32) -> uint32_t;
    static mut progname: *const i8;
    fn open_root_dir(drivename: i8, flags: i32, isRop: *mut i32) -> *mut Stream_t;
    fn printOom();
    static mut mdate: *const i8;
    static mut mversion: *const i8;
    fn force_pread(
        Stream: *mut Stream_t,
        buf: *mut i8,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn force_pwrite(
        Stream: *mut Stream_t,
        buf: *mut i8,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn free_stream(Stream: *mut *mut Stream_t) -> i32;
    fn sectorsToBytes(This: *mut Fs_t, off: uint32_t) -> mt_off_t;
    fn fatEncode(This: *mut Fs_t, pos: u32, value: u32);
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
pub struct Fs_t {
    pub head: Stream_t,
    pub serialized: i32,
    pub serial_number: u64,
    pub cluster_size: uint8_t,
    pub sector_size: uint16_t,
    pub fat_error: i32,
    pub fat_decode: Option<unsafe extern "C" fn(*mut Fs_t, u32) -> u32>,
    pub fat_encode: Option<unsafe extern "C" fn(*mut Fs_t, u32, u32) -> ()>,
    pub fat_dirty: i32,
    pub fat_start: uint16_t,
    pub fat_len: uint32_t,
    pub num_fat: uint8_t,
    pub end_fat: uint32_t,
    pub last_fat: uint32_t,
    pub fat_bits: u32,
    pub FatMap: *mut FatMap_t,
    pub dir_start: uint32_t,
    pub dir_len: uint16_t,
    pub clus_start: uint32_t,
    pub num_clus: uint32_t,
    pub drive: i8,
    pub primaryFat: uint32_t,
    pub writeAllFats: uint32_t,
    pub rootCluster: uint32_t,
    pub infoSectorLoc: uint32_t,
    pub backupBoot: uint16_t,
    pub last: uint32_t,
    pub freeSpace: uint32_t,
    pub preallocatedClusters: u32,
    pub lastFatSectorNr: uint32_t,
    pub lastFatSectorData: *mut u8,
    pub lastFatAccessMode: fatAccessMode_t,
    pub sectorMask: u32,
    pub sectorShift: u32,
    pub cp: *mut doscp_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum fatAccessMode_t {
    FAT_ACCESS_READ,
    FAT_ACCESS_WRITE,
}
impl fatAccessMode_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            fatAccessMode_t::FAT_ACCESS_READ => 0,
            fatAccessMode_t::FAT_ACCESS_WRITE => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> fatAccessMode_t {
        match value {
            0 => fatAccessMode_t::FAT_ACCESS_READ,
            1 => fatAccessMode_t::FAT_ACCESS_WRITE,
            _ => panic!("Invalid value for fatAccessMode_t: {}", value),
        }
    }
}
impl AddAssign<u32> for fatAccessMode_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = fatAccessMode_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for fatAccessMode_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = fatAccessMode_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for fatAccessMode_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = fatAccessMode_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for fatAccessMode_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = fatAccessMode_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for fatAccessMode_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = fatAccessMode_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for fatAccessMode_t {
    type Output = fatAccessMode_t;
    fn add(self, rhs: u32) -> fatAccessMode_t {
        fatAccessMode_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for fatAccessMode_t {
    type Output = fatAccessMode_t;
    fn sub(self, rhs: u32) -> fatAccessMode_t {
        fatAccessMode_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for fatAccessMode_t {
    type Output = fatAccessMode_t;
    fn mul(self, rhs: u32) -> fatAccessMode_t {
        fatAccessMode_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for fatAccessMode_t {
    type Output = fatAccessMode_t;
    fn div(self, rhs: u32) -> fatAccessMode_t {
        fatAccessMode_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for fatAccessMode_t {
    type Output = fatAccessMode_t;
    fn rem(self, rhs: u32) -> fatAccessMode_t {
        fatAccessMode_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[inline]
unsafe extern "C" fn init_random() {
    srandom(time(0 as *mut time_t) as u32);
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
        b"Usage: %s: [-c clusterList] [-s sectorList] [-c] [-V] device\n\0" as *const u8
            as *const i8,
        progname,
    );
    exit(ret);
}
unsafe extern "C" fn checkListTwice(mut filename: *mut i8) {
    if !filename.is_null() {
        fprintf(
            stderr,
            b"Only one of the -c or -s options may be given\n\0" as *const u8
                as *const i8,
        );
        exit(1 as i32);
    }
}
unsafe extern "C" fn mark(
    mut Fs: *mut Fs_t,
    mut offset: uint32_t,
    mut badClus: uint32_t,
) {
    let mut old: uint32_t = ((*Fs).fat_decode)
        .expect("non-null function pointer")(Fs, offset);
    if old == 0 as i32 as u32 {
        fatEncode(Fs, offset, badClus);
        return;
    }
    if old == badClus {
        fprintf(
            stderr,
            b"Cluster %d already marked\n\0" as *const u8 as *const i8,
            offset,
        );
    } else {
        fprintf(stderr, b"Cluster %d is busy\n\0" as *const u8 as *const i8, offset);
    };
}
static mut in_buf: *mut i8 = 0 as *const i8 as *mut i8;
static mut pat_buf: *mut i8 = 0 as *const i8 as *mut i8;
static mut in_len: size_t = 0;
unsafe extern "C" fn progress(mut i: u32, mut total: u32) {
    if i.wrapping_rem(10 as i32 as u32) == 0 as i32 as u32 {
        fprintf(
            stderr,
            b"                     \r%d/%d\r\0" as *const u8 as *const i8,
            i,
            total,
        );
    }
}
unsafe extern "C" fn scan(
    mut Fs: *mut Fs_t,
    mut dev: *mut Stream_t,
    mut cluster: uint32_t,
    mut badClus: uint32_t,
    mut buffer: *mut i8,
    mut doWrite: i32,
) -> i32 {
    let mut start: uint32_t = 0;
    let mut ret: off_t = 0;
    let mut pos: mt_off_t = 0;
    let mut bad: i32 = 0 as i32;
    if ((*Fs).fat_decode).expect("non-null function pointer")(Fs, cluster) != 0 {
        return 0 as i32;
    }
    start = cluster
        .wrapping_sub(2 as i32 as u32)
        .wrapping_mul((*Fs).cluster_size as u32)
        .wrapping_add((*Fs).clus_start);
    pos = sectorsToBytes(Fs, start);
    if doWrite != 0 {
        ret = force_pwrite(dev, buffer, pos, in_len);
        if ret < 0 as i32 as i64 || (ret as size_t) < in_len {
            bad = 1 as i32;
        }
    } else {
        ret = force_pread(dev, in_buf, pos, in_len);
        if ret < in_len as off_t {
            bad = 1 as i32;
        } else if !buffer.is_null() {
            let mut i: size_t = 0;
            i = 0 as i32 as size_t;
            while i < in_len {
                if *in_buf.offset(i as isize) as i32 != *buffer.offset(i as isize) as i32
                {
                    bad = 1 as i32;
                    break;
                } else {
                    i = i.wrapping_add(1);
                    i;
                }
            }
        }
    }
    if bad != 0 {
        printf(b"Bad cluster %d found\n\0" as *const u8 as *const i8, cluster);
        fatEncode(Fs, cluster, badClus);
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mbadblocks(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut type_0: i32,
) {
    let mut current_block: u64;
    let mut i: u32 = 0;
    let mut startSector: u32 = 2 as i32 as u32;
    let mut endSector: u32 = 0 as i32 as u32;
    let mut Fs: *mut Fs_t = 0 as *mut Fs_t;
    let mut Dir: *mut Stream_t = 0 as *mut Stream_t;
    let mut ret: i32 = 0;
    let mut filename: *mut i8 = 0 as *mut i8;
    let mut c: i32 = 0;
    let mut badClus: u32 = 0;
    let mut sectorMode: i32 = 0 as i32;
    let mut writeMode: i32 = 0 as i32;
    loop {
        c = getopt(argc, argv, b"i:s:cwS:E:\0" as *const u8 as *const i8);
        if !(c != -(1 as i32)) {
            break;
        }
        match c {
            105 => {
                set_cmd_line_image(optarg);
            }
            99 => {
                checkListTwice(filename);
                filename = strdup(optarg);
            }
            115 => {
                checkListTwice(filename);
                filename = strdup(optarg);
                sectorMode = 1 as i32;
            }
            83 => {
                startSector = atoui(optarg);
            }
            69 => {
                endSector = atoui(optarg);
            }
            119 => {
                writeMode = 1 as i32;
            }
            104 => {
                usage(0 as i32);
            }
            _ => {
                usage(1 as i32);
            }
        }
    }
    if argc != optind + 1 as i32
        || *(*argv.offset(optind as isize)).offset(0 as i32 as isize) == 0
        || *(*argv.offset(optind as isize)).offset(1 as i32 as isize) as i32
            != ':' as i32
        || *(*argv.offset(optind as isize)).offset(2 as i32 as isize) as i32 != 0
    {
        usage(1 as i32);
    }
    Dir = open_root_dir(
        *(*argv.offset(optind as isize)).offset(0 as i32 as isize),
        0o2 as i32,
        0 as *mut i32,
    );
    if Dir.is_null() {
        fprintf(
            stderr,
            b"%s: Cannot initialize drive\n\0" as *const u8 as *const i8,
            *argv.offset(0 as i32 as isize),
        );
        exit(1 as i32);
    }
    Fs = GetFs(Dir) as *mut Fs_t;
    in_len = ((*Fs).cluster_size as i32 * (*Fs).sector_size as i32) as size_t;
    in_buf = malloc(in_len) as *mut i8;
    if in_buf.is_null() {
        printOom();
        ret = 1 as i32;
    } else {
        if writeMode != 0 {
            pat_buf = malloc(in_len.wrapping_mul(311 as i32 as u64)) as *mut i8;
            if pat_buf.is_null() {
                printOom();
                ret = 1 as i32;
                current_block = 2254909331804672544;
            } else {
                init_random();
                i = 0 as i32 as u32;
                while (i as u64) < in_len.wrapping_mul(311 as i32 as u64) {
                    *pat_buf.offset(i as isize) = random() as i8;
                    i = i.wrapping_add(1);
                    i;
                }
                current_block = 14832935472441733737;
            }
        } else {
            current_block = 14832935472441733737;
        }
        match current_block {
            2254909331804672544 => {}
            _ => {
                i = 0 as i32 as u32;
                loop {
                    if !(i < (*Fs).clus_start) {
                        current_block = 1434579379687443766;
                        break;
                    }
                    let mut r: ssize_t = 0;
                    r = ((*(*(*Fs).head.Next).Class).pread)
                        .expect(
                            "non-null function pointer",
                        )(
                        (*Fs).head.Next,
                        in_buf,
                        sectorsToBytes(Fs, i),
                        (*Fs).sector_size as size_t,
                    );
                    if r < 0 as i32 as i64 {
                        perror(b"early error\0" as *const u8 as *const i8);
                        ret = -(1 as i32);
                        current_block = 2254909331804672544;
                        break;
                    } else if (r as size_t) < (*Fs).sector_size as u64 {
                        fprintf(
                            stderr,
                            b"end of file in file_read\n\0" as *const u8 as *const i8,
                        );
                        ret = 1 as i32;
                        current_block = 2254909331804672544;
                        break;
                    } else {
                        i = i.wrapping_add(1);
                        i;
                    }
                }
                match current_block {
                    2254909331804672544 => {}
                    _ => {
                        ret = 0 as i32;
                        badClus = ((*Fs).last_fat).wrapping_add(1 as i32 as u32);
                        if startSector < 2 as i32 as u32 {
                            startSector = 2 as i32 as u32;
                        }
                        if endSector > ((*Fs).num_clus).wrapping_add(2 as i32 as u32)
                            || endSector <= 0 as i32 as u32
                        {
                            endSector = ((*Fs).num_clus).wrapping_add(2 as i32 as u32);
                        }
                        if !filename.is_null() {
                            let mut line: [i8; 80] = [0; 80];
                            let mut f: *mut FILE = fopen(
                                filename,
                                b"r\0" as *const u8 as *const i8,
                            );
                            if f.is_null() {
                                fprintf(
                                    stderr,
                                    b"Could not open %s (%s)\n\0" as *const u8 as *const i8,
                                    filename,
                                    strerror(*__errno_location()),
                                );
                                ret = 1 as i32;
                            } else {
                                while !(fgets(
                                    line.as_mut_ptr(),
                                    ::core::mem::size_of::<[i8; 80]>() as u64 as i32,
                                    f,
                                ))
                                    .is_null()
                                {
                                    let mut ptr: *mut i8 = line
                                        .as_mut_ptr()
                                        .offset(
                                            strspn(
                                                line.as_mut_ptr(),
                                                b" \t\0" as *const u8 as *const i8,
                                            ) as isize,
                                        );
                                    let mut offset: uint32_t = strtou32(
                                        ptr,
                                        0 as *mut *mut i8,
                                        0 as i32,
                                    );
                                    if sectorMode != 0 {
                                        offset = offset
                                            .wrapping_sub((*Fs).clus_start)
                                            .wrapping_div((*Fs).cluster_size as u32)
                                            .wrapping_add(2 as i32 as u32);
                                    }
                                    if offset < 2 as i32 as u32 {
                                        fprintf(
                                            stderr,
                                            b"Sector before start\n\0" as *const u8 as *const i8,
                                        );
                                    } else if offset >= (*Fs).num_clus {
                                        fprintf(
                                            stderr,
                                            b"Sector beyond end\n\0" as *const u8 as *const i8,
                                        );
                                    } else {
                                        mark(Fs, offset, badClus);
                                        ret = 1 as i32;
                                    }
                                }
                            }
                        } else {
                            let mut dev: *mut Stream_t = 0 as *mut Stream_t;
                            dev = (*Fs).head.Next;
                            if !((*dev).Next).is_null() {
                                dev = (*dev).Next;
                            }
                            in_len = ((*Fs).cluster_size as i32
                                * (*Fs).sector_size as i32) as size_t;
                            if writeMode != 0 {
                                i = startSector;
                                while i < endSector {
                                    if got_signal != 0 {
                                        break;
                                    }
                                    progress(i, (*Fs).num_clus);
                                    ret
                                        |= scan(
                                            Fs,
                                            dev,
                                            i,
                                            badClus,
                                            pat_buf
                                                .offset(
                                                    in_len
                                                        .wrapping_mul(i.wrapping_rem(311 as i32 as u32) as u64)
                                                        as isize,
                                                ),
                                            1 as i32,
                                        );
                                    i = i.wrapping_add(1);
                                    i;
                                }
                                if got_signal == 0 {
                                    ((*(*dev).Class).discard)
                                        .expect("non-null function pointer")(dev);
                                }
                                i = startSector;
                                while i < endSector {
                                    if got_signal != 0 {
                                        break;
                                    }
                                    progress(i, (*Fs).num_clus);
                                    ret
                                        |= scan(
                                            Fs,
                                            dev,
                                            i,
                                            badClus,
                                            pat_buf
                                                .offset(
                                                    in_len
                                                        .wrapping_mul(i.wrapping_rem(311 as i32 as u32) as u64)
                                                        as isize,
                                                ),
                                            0 as i32,
                                        );
                                    i = i.wrapping_add(1);
                                    i;
                                }
                            } else {
                                i = startSector;
                                while i < endSector {
                                    if got_signal != 0 {
                                        break;
                                    }
                                    progress(i, (*Fs).num_clus);
                                    ret |= scan(Fs, dev, i, badClus, 0 as *mut i8, 0 as i32);
                                    i = i.wrapping_add(1);
                                    i;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free_stream(&mut Dir);
    exit(ret);
}
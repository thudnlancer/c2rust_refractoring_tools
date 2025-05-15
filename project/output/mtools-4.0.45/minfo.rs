use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type doscp_t;
    pub type FatMap_t;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn exit(_: i32) -> !;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn getopt(___argc: i32, ___argv: *const *mut i8, __shortopts: *const i8) -> i32;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn safe_malloc(size: size_t) -> *mut libc::c_void;
    fn get_default_drive() -> i8;
    fn set_cmd_line_image(img: *mut i8);
    fn free_stream(Stream: *mut *mut Stream_t) -> i32;
    fn print_sector(message: *const i8, data: *mut u8, size: i32);
    fn force_pread(
        Stream: *mut Stream_t,
        buf: *mut i8,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
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
    static mut progname: *const i8;
    static mut mdate: *const i8;
    static mut mversion: *const i8;
    fn helpFlag(_: i32, _: *mut *mut i8) -> i32;
    fn initFsForFormat(Fs: *mut Fs_t);
    fn calc_fs_parameters(
        dev: *mut device,
        fat32: bool,
        tot_sectors: uint32_t,
        Fs: *mut Fs_t,
        descr: *mut uint8_t,
    ) -> i32;
    fn setFsSectorSize(Fs: *mut Fs_t, dev: *mut device, msize: uint16_t);
    fn parseFsParams(
        This: *mut Fs_t,
        boot: *mut bootsector,
        media: i32,
        cylinder_size: u32,
    ) -> uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InfoSector_t {
    pub signature1: [u8; 4],
    pub filler1: [u8; 480],
    pub signature2: [u8; 4],
    pub count: [u8; 4],
    pub pos: [u8; 4],
    pub filler2: [u8; 14],
    pub signature3: [u8; 2],
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
pub type device_t = device;
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
unsafe extern "C" fn usage(mut ret: i32) -> ! {
    fprintf(
        stderr,
        b"Mtools version %s, dated %s\n\0" as *const u8 as *const i8,
        mversion,
        mdate,
    );
    fprintf(stderr, b"Usage: %s [-v] drive\n\0" as *const u8 as *const i8, progname);
    exit(ret);
}
unsafe extern "C" fn displayInfosector(
    mut Stream: *mut Stream_t,
    mut boot: *mut bootsector,
) {
    let mut infosec: *mut InfoSector_t = 0 as *mut InfoSector_t;
    if ((*boot).boot.ext.fat32.infoSector[0 as i32 as usize] as i32
        + (((*boot).boot.ext.fat32.infoSector[1 as i32 as usize] as i32) << 8 as i32))
        as uint16_t as i32 == 0xffff as i32
    {
        return;
    }
    infosec = safe_malloc(
        ((*boot).boot.secsiz[0 as i32 as usize] as i32
            + (((*boot).boot.secsiz[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
            as size_t,
    ) as *mut InfoSector_t;
    force_pread(
        Stream,
        infosec as *mut i8,
        ((*boot).boot.secsiz[0 as i32 as usize] as i32
            + (((*boot).boot.secsiz[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
            as mt_off_t
            * ((*boot).boot.ext.fat32.infoSector[0 as i32 as usize] as i32
                + (((*boot).boot.ext.fat32.infoSector[1 as i32 as usize] as i32)
                    << 8 as i32)) as uint16_t as i64,
        ((*boot).boot.secsiz[0 as i32 as usize] as i32
            + (((*boot).boot.secsiz[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
            as size_t,
    );
    printf(b"\nInfosector:\n\0" as *const u8 as *const i8);
    printf(
        b"signature=0x%08x\n\0" as *const u8 as *const i8,
        (((*infosec).signature1[0 as i32 as usize] as i32
            + (((*infosec).signature1[1 as i32 as usize] as i32) << 8 as i32))
            as uint16_t as i32
            + (((*((*infosec).signature1)
                .as_mut_ptr()
                .offset(2 as i32 as isize)
                .offset(0 as i32 as isize) as i32
                + ((*((*infosec).signature1)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t as i32)
                << 16 as i32)) as uint32_t,
    );
    if (((*infosec).count[0 as i32 as usize] as i32
        + (((*infosec).count[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t as i32
        + (((*((*infosec).count)
            .as_mut_ptr()
            .offset(2 as i32 as isize)
            .offset(0 as i32 as isize) as i32
            + ((*((*infosec).count)
                .as_mut_ptr()
                .offset(2 as i32 as isize)
                .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t as i32)
            << 16 as i32)) as uint32_t != 0xffffffff as u32
    {
        printf(
            b"free clusters=%u\n\0" as *const u8 as *const i8,
            (((*infosec).count[0 as i32 as usize] as i32
                + (((*infosec).count[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
                as i32
                + (((*((*infosec).count)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(0 as i32 as isize) as i32
                    + ((*((*infosec).count)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                    as i32) << 16 as i32)) as uint32_t,
        );
    }
    if (((*infosec).pos[0 as i32 as usize] as i32
        + (((*infosec).pos[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t as i32
        + (((*((*infosec).pos)
            .as_mut_ptr()
            .offset(2 as i32 as isize)
            .offset(0 as i32 as isize) as i32
            + ((*((*infosec).pos)
                .as_mut_ptr()
                .offset(2 as i32 as isize)
                .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t as i32)
            << 16 as i32)) as uint32_t != 0xffffffff as u32
    {
        printf(
            b"last allocated cluster=%u\n\0" as *const u8 as *const i8,
            (((*infosec).pos[0 as i32 as usize] as i32
                + (((*infosec).pos[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
                as i32
                + (((*((*infosec).pos)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(0 as i32 as isize) as i32
                    + ((*((*infosec).pos)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                    as i32) << 16 as i32)) as uint32_t,
        );
    }
}
unsafe extern "C" fn getHidden(mut boot: *mut bootsector) -> uint32_t {
    return if ((*boot).boot.psect[0 as i32 as usize] as i32
        + (((*boot).boot.psect[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
        as i32 != 0
    {
        ((*boot).boot.nhs[0 as i32 as usize] as i32
            + (((*boot).boot.nhs[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
            as u32
    } else {
        (((*boot).boot.nhs[0 as i32 as usize] as i32
            + (((*boot).boot.nhs[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
            as i32
            + (((*((*boot).boot.nhs)
                .as_mut_ptr()
                .offset(2 as i32 as isize)
                .offset(0 as i32 as isize) as i32
                + ((*((*boot).boot.nhs)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t as i32)
                << 16 as i32)) as uint32_t
    };
}
unsafe extern "C" fn displayBPB(mut Stream: *mut Stream_t, mut boot: *mut bootsector) {
    let mut labelBlock: *mut label_blk_t = 0 as *mut label_blk_t;
    printf(b"bootsector information\n\0" as *const u8 as *const i8);
    printf(b"======================\n\0" as *const u8 as *const i8);
    printf(
        b"banner:\"%.8s\"\n\0" as *const u8 as *const i8,
        ((*boot).boot.banner).as_mut_ptr(),
    );
    printf(
        b"sector size: %d bytes\n\0" as *const u8 as *const i8,
        ((*boot).boot.secsiz[0 as i32 as usize] as i32
            + (((*boot).boot.secsiz[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
            as i32,
    );
    printf(
        b"cluster size: %d sectors\n\0" as *const u8 as *const i8,
        (*boot).boot.clsiz as i32,
    );
    printf(
        b"reserved (boot) sectors: %d\n\0" as *const u8 as *const i8,
        ((*boot).boot.nrsvsect[0 as i32 as usize] as i32
            + (((*boot).boot.nrsvsect[1 as i32 as usize] as i32) << 8 as i32))
            as uint16_t as i32,
    );
    printf(b"fats: %d\n\0" as *const u8 as *const i8, (*boot).boot.nfat as i32);
    printf(
        b"max available root directory slots: %d\n\0" as *const u8 as *const i8,
        ((*boot).boot.dirents[0 as i32 as usize] as i32
            + (((*boot).boot.dirents[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
            as i32,
    );
    printf(
        b"small size: %d sectors\n\0" as *const u8 as *const i8,
        ((*boot).boot.psect[0 as i32 as usize] as i32
            + (((*boot).boot.psect[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
            as i32,
    );
    printf(
        b"media descriptor byte: 0x%x\n\0" as *const u8 as *const i8,
        (*boot).boot.descr as i32,
    );
    printf(
        b"sectors per fat: %d\n\0" as *const u8 as *const i8,
        ((*boot).boot.fatlen[0 as i32 as usize] as i32
            + (((*boot).boot.fatlen[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
            as i32,
    );
    printf(
        b"sectors per track: %d\n\0" as *const u8 as *const i8,
        ((*boot).boot.nsect[0 as i32 as usize] as i32
            + (((*boot).boot.nsect[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
            as i32,
    );
    printf(
        b"heads: %d\n\0" as *const u8 as *const i8,
        ((*boot).boot.nheads[0 as i32 as usize] as i32
            + (((*boot).boot.nheads[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
            as i32,
    );
    printf(b"hidden sectors: %d\n\0" as *const u8 as *const i8, getHidden(boot));
    if ((*boot).boot.psect[0 as i32 as usize] as i32
        + (((*boot).boot.psect[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t == 0
    {
        printf(
            b"big size: %u sectors\n\0" as *const u8 as *const i8,
            (((*boot).boot.bigsect[0 as i32 as usize] as i32
                + (((*boot).boot.bigsect[1 as i32 as usize] as i32) << 8 as i32))
                as uint16_t as i32
                + (((*((*boot).boot.bigsect)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(0 as i32 as isize) as i32
                    + ((*((*boot).boot.bigsect)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                    as i32) << 16 as i32)) as uint32_t,
        );
    }
    if ((*boot).boot.fatlen[0 as i32 as usize] as i32
        + (((*boot).boot.fatlen[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t != 0
    {
        labelBlock = &mut (*boot).boot.ext.old.labelBlock;
    } else {
        labelBlock = &mut (*boot).boot.ext.fat32.labelBlock;
    }
    if (*labelBlock).dos4 as i32 == 0x28 as i32
        || (*labelBlock).dos4 as i32 == 0x29 as i32
    {
        printf(
            b"physical drive id: 0x%x\n\0" as *const u8 as *const i8,
            (*labelBlock).physdrive as i32,
        );
        printf(
            b"reserved=0x%x\n\0" as *const u8 as *const i8,
            (*labelBlock).reserved as i32,
        );
        printf(b"dos4=0x%x\n\0" as *const u8 as *const i8, (*labelBlock).dos4 as i32);
        printf(
            b"serial number: %08X\n\0" as *const u8 as *const i8,
            (((*labelBlock).serial[0 as i32 as usize] as i32
                + (((*labelBlock).serial[1 as i32 as usize] as i32) << 8 as i32))
                as uint16_t as i32
                + (((*((*labelBlock).serial)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(0 as i32 as isize) as i32
                    + ((*((*labelBlock).serial)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                    as i32) << 16 as i32)) as uint32_t,
        );
        printf(
            b"disk label=\"%11.11s\"\n\0" as *const u8 as *const i8,
            ((*labelBlock).label).as_mut_ptr(),
        );
        printf(
            b"disk type=\"%8.8s\"\n\0" as *const u8 as *const i8,
            ((*labelBlock).fat_type).as_mut_ptr(),
        );
    }
    if ((*boot).boot.fatlen[0 as i32 as usize] as i32
        + (((*boot).boot.fatlen[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t == 0
    {
        printf(
            b"Big fatlen=%u\n\0" as *const u8 as *const i8,
            (((*boot).boot.ext.fat32.bigFat[0 as i32 as usize] as i32
                + (((*boot).boot.ext.fat32.bigFat[1 as i32 as usize] as i32)
                    << 8 as i32)) as uint16_t as i32
                + (((*((*boot).boot.ext.fat32.bigFat)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(0 as i32 as isize) as i32
                    + ((*((*boot).boot.ext.fat32.bigFat)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                    as i32) << 16 as i32)) as uint32_t,
        );
        printf(
            b"Extended flags=0x%04x\n\0" as *const u8 as *const i8,
            ((*boot).boot.ext.fat32.extFlags[0 as i32 as usize] as i32
                + (((*boot).boot.ext.fat32.extFlags[1 as i32 as usize] as i32)
                    << 8 as i32)) as uint16_t as i32,
        );
        printf(
            b"FS version=0x%04x\n\0" as *const u8 as *const i8,
            ((*boot).boot.ext.fat32.fsVersion[0 as i32 as usize] as i32
                + (((*boot).boot.ext.fat32.fsVersion[1 as i32 as usize] as i32)
                    << 8 as i32)) as uint16_t as i32,
        );
        printf(
            b"rootCluster=%u\n\0" as *const u8 as *const i8,
            (((*boot).boot.ext.fat32.rootCluster[0 as i32 as usize] as i32
                + (((*boot).boot.ext.fat32.rootCluster[1 as i32 as usize] as i32)
                    << 8 as i32)) as uint16_t as i32
                + (((*((*boot).boot.ext.fat32.rootCluster)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(0 as i32 as isize) as i32
                    + ((*((*boot).boot.ext.fat32.rootCluster)
                        .as_mut_ptr()
                        .offset(2 as i32 as isize)
                        .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t
                    as i32) << 16 as i32)) as uint32_t,
        );
        if ((*boot).boot.ext.fat32.infoSector[0 as i32 as usize] as i32
            + (((*boot).boot.ext.fat32.infoSector[1 as i32 as usize] as i32)
                << 8 as i32)) as uint16_t as i32 != 0xffff as i32
        {
            printf(
                b"infoSector location=%d\n\0" as *const u8 as *const i8,
                ((*boot).boot.ext.fat32.infoSector[0 as i32 as usize] as i32
                    + (((*boot).boot.ext.fat32.infoSector[1 as i32 as usize] as i32)
                        << 8 as i32)) as uint16_t as i32,
            );
        }
        if ((*boot).boot.ext.fat32.backupBoot[0 as i32 as usize] as i32
            + (((*boot).boot.ext.fat32.backupBoot[1 as i32 as usize] as i32)
                << 8 as i32)) as uint16_t as i32 != 0xffff as i32
        {
            printf(
                b"backup boot sector=%d\n\0" as *const u8 as *const i8,
                ((*boot).boot.ext.fat32.backupBoot[0 as i32 as usize] as i32
                    + (((*boot).boot.ext.fat32.backupBoot[1 as i32 as usize] as i32)
                        << 8 as i32)) as uint16_t as i32,
            );
        }
        displayInfosector(Stream, boot);
    }
}
unsafe extern "C" fn try_0(
    mut tot_sectors: uint32_t,
    mut masterFs: *mut Fs_t,
    mut tryFs: *mut Fs_t,
    mut master_dev: *mut device,
    mut try_dev: *mut device,
    mut bootDescr: *mut uint8_t,
) -> i32 {
    *tryFs = *masterFs;
    *try_dev = *master_dev;
    return calc_fs_parameters(try_dev, 0 as i32 != 0, tot_sectors, tryFs, bootDescr);
}
unsafe extern "C" fn print_mformat_commandline(
    mut imgFile: *const i8,
    mut drive: i8,
    mut dev: *mut device,
    mut boot: *mut bootsector,
    mut media: i32,
    mut haveBPB: i32,
) {
    let mut size_code: uint8_t = 0;
    let mut sect_per_track: uint32_t = 0;
    let mut hidden: uint32_t = 0;
    let mut tot_sectors: uint32_t = 0;
    let mut tracks_match: i32 = 0 as i32;
    let mut masterFs: Fs_t = Fs_t {
        head: Stream_t {
            Class: 0 as *mut Class_t,
            refs: 0,
            Next: 0 as *mut Stream_t,
        },
        serialized: 0,
        serial_number: 0,
        cluster_size: 0,
        sector_size: 0,
        fat_error: 0,
        fat_decode: None,
        fat_encode: None,
        fat_dirty: 0,
        fat_start: 0,
        fat_len: 0,
        num_fat: 0,
        end_fat: 0,
        last_fat: 0,
        fat_bits: 0,
        FatMap: 0 as *mut FatMap_t,
        dir_start: 0,
        dir_len: 0,
        clus_start: 0,
        num_clus: 0,
        drive: 0,
        primaryFat: 0,
        writeAllFats: 0,
        rootCluster: 0,
        infoSectorLoc: 0,
        backupBoot: 0,
        last: 0,
        freeSpace: 0,
        preallocatedClusters: 0,
        lastFatSectorNr: 0,
        lastFatSectorData: 0 as *mut u8,
        lastFatAccessMode: fatAccessMode_t::FAT_ACCESS_READ,
        sectorMask: 0,
        sectorShift: 0,
        cp: 0 as *mut doscp_t,
    };
    let mut tryFs: Fs_t = Fs_t {
        head: Stream_t {
            Class: 0 as *mut Class_t,
            refs: 0,
            Next: 0 as *mut Stream_t,
        },
        serialized: 0,
        serial_number: 0,
        cluster_size: 0,
        sector_size: 0,
        fat_error: 0,
        fat_decode: None,
        fat_encode: None,
        fat_dirty: 0,
        fat_start: 0,
        fat_len: 0,
        num_fat: 0,
        end_fat: 0,
        last_fat: 0,
        fat_bits: 0,
        FatMap: 0 as *mut FatMap_t,
        dir_start: 0,
        dir_len: 0,
        clus_start: 0,
        num_clus: 0,
        drive: 0,
        primaryFat: 0,
        writeAllFats: 0,
        rootCluster: 0,
        infoSectorLoc: 0,
        backupBoot: 0,
        last: 0,
        freeSpace: 0,
        preallocatedClusters: 0,
        lastFatSectorNr: 0,
        lastFatSectorData: 0 as *mut u8,
        lastFatAccessMode: fatAccessMode_t::FAT_ACCESS_READ,
        sectorMask: 0,
        sectorShift: 0,
        cp: 0 as *mut doscp_t,
    };
    let mut actual: Fs_t = Fs_t {
        head: Stream_t {
            Class: 0 as *mut Class_t,
            refs: 0,
            Next: 0 as *mut Stream_t,
        },
        serialized: 0,
        serial_number: 0,
        cluster_size: 0,
        sector_size: 0,
        fat_error: 0,
        fat_decode: None,
        fat_encode: None,
        fat_dirty: 0,
        fat_start: 0,
        fat_len: 0,
        num_fat: 0,
        end_fat: 0,
        last_fat: 0,
        fat_bits: 0,
        FatMap: 0 as *mut FatMap_t,
        dir_start: 0,
        dir_len: 0,
        clus_start: 0,
        num_clus: 0,
        drive: 0,
        primaryFat: 0,
        writeAllFats: 0,
        rootCluster: 0,
        infoSectorLoc: 0,
        backupBoot: 0,
        last: 0,
        freeSpace: 0,
        preallocatedClusters: 0,
        lastFatSectorNr: 0,
        lastFatSectorData: 0 as *mut u8,
        lastFatAccessMode: fatAccessMode_t::FAT_ACCESS_READ,
        sectorMask: 0,
        sectorShift: 0,
        cp: 0 as *mut doscp_t,
    };
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
    let mut tryMedia: uint8_t = 0;
    let mut bad: i32 = 0;
    sect_per_track = ((*dev).sectors as i32 * (*dev).heads as i32) as uint32_t;
    if sect_per_track == 0 as i32 as u32 {
        return;
    }
    tot_sectors = parseFsParams(
        &mut actual,
        boot,
        media | (if haveBPB != 0 { 0x100 as i32 } else { 0 as i32 }),
        sect_per_track,
    );
    if tot_sectors == 0 as i32 as u32 {
        return;
    }
    printf(b"mformat command line:\n  mformat \0" as *const u8 as *const i8);
    if haveBPB != 0 {
        if media == 0xf0 as i32 {
            hidden = getHidden(boot);
        } else {
            hidden = 0 as i32 as uint32_t;
        }
        size_code = (actual.sectorShift as uint8_t as i32 - 7 as i32) as uint8_t;
    } else {
        size_code = 2 as i32 as uint8_t;
        hidden = 0 as i32 as uint32_t;
    }
    if tot_sectors
        == ((*dev).tracks)
            .wrapping_mul(sect_per_track)
            .wrapping_sub(hidden.wrapping_rem(sect_per_track))
    {
        tracks_match = 1 as i32;
        printf(b"-t %d \0" as *const u8 as *const i8, (*dev).tracks);
    } else {
        printf(b"-T %d \0" as *const u8 as *const i8, tot_sectors);
    }
    printf(
        b"-h %d -s %d \0" as *const u8 as *const i8,
        (*dev).heads as i32,
        (*dev).sectors as i32,
    );
    if haveBPB != 0 && (hidden != 0 || tracks_match == 0) {
        printf(b"-H %d \0" as *const u8 as *const i8, hidden);
    }
    used_dev = *dev;
    if size_code as i32 != 2 as i32 {
        printf(b"-S %d \0" as *const u8 as *const i8, size_code as i32);
        used_dev.ssize = size_code;
    }
    initFsForFormat(&mut masterFs);
    setFsSectorSize(&mut masterFs, &mut used_dev, 0 as i32 as uint16_t);
    if actual.num_fat as i32 != 2 as i32 {
        masterFs.num_fat = actual.num_fat;
        printf(b"-d %d \0" as *const u8 as *const i8, actual.num_fat as i32);
    }
    bad = try_0(
        tot_sectors,
        &mut masterFs,
        &mut tryFs,
        dev,
        &mut used_dev,
        &mut tryMedia,
    );
    if bad != 0 || actual.dir_len as i32 != tryFs.dir_len as i32 {
        masterFs.dir_len = actual.dir_len;
        printf(b"-r %d \0" as *const u8 as *const i8, actual.dir_len as i32);
        bad = try_0(
            tot_sectors,
            &mut masterFs,
            &mut tryFs,
            dev,
            &mut used_dev,
            &mut tryMedia,
        );
    }
    if bad != 0 || actual.cluster_size as i32 != tryFs.cluster_size as i32 {
        masterFs.cluster_size = actual.cluster_size;
        printf(b"-c %d \0" as *const u8 as *const i8, actual.cluster_size as i32);
        bad = try_0(
            tot_sectors,
            &mut masterFs,
            &mut tryFs,
            dev,
            &mut used_dev,
            &mut tryMedia,
        );
    }
    if bad != 0 || actual.fat_start as i32 != tryFs.fat_start as i32 {
        masterFs.fat_start = actual.fat_start;
        printf(b"-R %d \0" as *const u8 as *const i8, actual.fat_start as i32);
        bad = try_0(
            tot_sectors,
            &mut masterFs,
            &mut tryFs,
            dev,
            &mut used_dev,
            &mut tryMedia,
        );
    }
    if bad != 0 || actual.fat_len != tryFs.fat_len {
        masterFs.fat_len = actual.fat_len;
        printf(b"-L %d \0" as *const u8 as *const i8, actual.fat_len);
        bad = try_0(
            tot_sectors,
            &mut masterFs,
            &mut tryFs,
            dev,
            &mut used_dev,
            &mut tryMedia,
        );
    }
    if bad == 0 {} else {
        __assert_fail(
            b"!bad\0" as *const u8 as *const i8,
            b"minfo.c\0" as *const u8 as *const i8,
            231 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 98],
                &[i8; 98],
            >(
                b"void print_mformat_commandline(const char *, char, struct device *, union bootsector *, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_9351: {
        if bad == 0 {} else {
            __assert_fail(
                b"!bad\0" as *const u8 as *const i8,
                b"minfo.c\0" as *const u8 as *const i8,
                231 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 98],
                    &[i8; 98],
                >(
                    b"void print_mformat_commandline(const char *, char, struct device *, union bootsector *, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if media & 0xff as i32 != tryMedia as i32 & 0xff as i32 {
        printf(b"-m %d \0" as *const u8 as *const i8, media & 0xff as i32);
    }
    if actual.fat_bits == 32 as i32 as u32 {
        if actual.backupBoot as i32 != tryFs.backupBoot as i32 {
            printf(b"-K %d \0" as *const u8 as *const i8, actual.backupBoot as i32);
        }
    }
    if !imgFile.is_null() {
        printf(b"-i \"%s\" \0" as *const u8 as *const i8, imgFile);
    }
    printf(b"%c:\n\0" as *const u8 as *const i8, ch_toupper(drive) as i32);
    printf(b"\n\0" as *const u8 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn minfo(mut argc: i32, mut argv: *mut *mut i8, mut type_0: i32) {
    let mut boot: bootsector = bootsector { bytes: [0; 4096] };
    let mut name: [i8; 2048] = [0; 2048];
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
    let mut drive: i8 = 0;
    let mut verbose: i32 = 0 as i32;
    let mut c: i32 = 0;
    let mut Stream: *mut Stream_t = 0 as *mut Stream_t;
    let mut have_drive: i32 = 0 as i32;
    let mut ex: i32 = 0 as i32;
    let mut imgFile: *mut i8 = 0 as *mut i8;
    if helpFlag(argc, argv) != 0 {
        usage(0 as i32);
    }
    loop {
        c = getopt(argc, argv, b"i:vh\0" as *const u8 as *const i8);
        if !(c != -(1 as i32)) {
            break;
        }
        match c {
            105 => {
                set_cmd_line_image(optarg);
                imgFile = optarg;
            }
            118 => {
                verbose = 1 as i32;
            }
            104 => {
                usage(0 as i32);
            }
            _ => {
                usage(1 as i32);
            }
        }
    }
    while optind <= argc {
        let mut media: i32 = 0;
        let mut haveBPB: i32 = 0;
        if optind == argc {
            if have_drive != 0 {
                break;
            }
            drive = get_default_drive();
        } else {
            if *(*argv.offset(optind as isize)).offset(0 as i32 as isize) == 0
                || *(*argv.offset(optind as isize)).offset(1 as i32 as isize) as i32
                    != ':' as i32
            {
                usage(1 as i32);
            }
            drive = ch_toupper(
                *(*argv.offset(optind as isize)).offset(0 as i32 as isize),
            );
        }
        have_drive = 1 as i32;
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
            fprintf(
                stderr,
                b"Could not open drive %c:\n\0" as *const u8 as *const i8,
                drive as i32,
            );
            ex = 1 as i32;
        } else {
            haveBPB = (media >= 0x100 as i32) as i32;
            media = media & 0xff as i32;
            printf(b"device information:\n\0" as *const u8 as *const i8);
            printf(b"===================\n\0" as *const u8 as *const i8);
            printf(b"filename=\"%s\"\n\0" as *const u8 as *const i8, name.as_mut_ptr());
            printf(
                b"sectors per track: %d\n\0" as *const u8 as *const i8,
                dev.sectors as i32,
            );
            printf(b"heads: %d\n\0" as *const u8 as *const i8, dev.heads as i32);
            printf(b"cylinders: %d\n\n\0" as *const u8 as *const i8, dev.tracks);
            printf(
                b"media byte: %02x\n\n\0" as *const u8 as *const i8,
                media & 0xff as i32,
            );
            print_mformat_commandline(
                imgFile,
                drive,
                &mut dev,
                &mut boot,
                media,
                haveBPB,
            );
            if haveBPB != 0 || verbose != 0 {
                displayBPB(Stream, &mut boot);
            }
            if verbose != 0 {
                let mut size: uint16_t = 0;
                let mut ssize: ssize_t = 0;
                let mut buf: *mut u8 = 0 as *mut u8;
                printf(b"\n\0" as *const u8 as *const i8);
                size = (boot.boot.secsiz[0 as i32 as usize] as i32
                    + ((boot.boot.secsiz[1 as i32 as usize] as i32) << 8 as i32))
                    as uint16_t;
                buf = malloc(size as u64) as *mut u8;
                if buf.is_null() {
                    fprintf(
                        stderr,
                        b"Out of memory error\n\0" as *const u8 as *const i8,
                    );
                    exit(1 as i32);
                }
                ssize = ((*(*Stream).Class).pread)
                    .expect(
                        "non-null function pointer",
                    )(Stream, buf as *mut i8, 0 as i32 as mt_off_t, size as size_t);
                if ssize < 0 as i32 as i64 {
                    perror(b"read boot sector\0" as *const u8 as *const i8);
                    exit(1 as i32);
                }
                print_sector(
                    b"Boot sector hexdump\0" as *const u8 as *const i8,
                    buf,
                    ssize as uint16_t as i32,
                );
            }
        }
        optind += 1;
        optind;
    }
    free_stream(&mut Stream);
    exit(ex);
}
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type doscp_t;
    pub type FatMap_t;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: i32) -> !;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
    fn __errno_location() -> *mut i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strerror(_: i32) -> *mut i8;
    fn getfreeMinClusters(Stream: *mut Stream_t, ref_0: uint32_t) -> i32;
    static mut mtools_skip_check: u32;
    fn expand(_: *const i8, _: *mut i8) -> *const i8;
    fn get_data_pass_through(
        Stream: *mut Stream_t,
        date: *mut time_t,
        size: *mut mt_off_t,
        type_0: *mut i32,
        address: *mut uint32_t,
    ) -> i32;
    fn pwrite_pass_through(
        Stream: *mut Stream_t,
        buf: *mut i8,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn pread_pass_through(
        Stream: *mut Stream_t,
        buf: *mut i8,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    static mut devices: *mut device;
    fn check_if_sectors_fit(
        tot_sectors: uint32_t,
        maxBytes: mt_off_t,
        sectorSize: uint32_t,
        errmsg: *mut i8,
    ) -> i32;
    fn free_stream(Stream: *mut *mut Stream_t) -> i32;
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn force_pread(
        Stream: *mut Stream_t,
        buf: *mut i8,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn log_2(_: u32) -> u32;
    fn getOldDosByMedia(media: i32) -> *mut OldDos_t;
    fn setDeviceFromOldDos(media: i32, dev: *mut device) -> i32;
    fn set_fat(This: *mut Fs_t, haveBigFatLen: bool);
    fn fs_free(Stream: *mut Stream_t) -> i32;
    fn fat_read(This: *mut Fs_t, boot: *mut bootsector, nodups: i32) -> i32;
    fn fat_write(This: *mut Fs_t);
    fn buf_init(
        Next: *mut Stream_t,
        size: size_t,
        cylinderSize: size_t,
        sectorSize: size_t,
    ) -> *mut Stream_t;
    fn cp_open(codepage: u32) -> *mut doscp_t;
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
pub type mt_off_t = off_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OldDos_t {
    pub tracks: u32,
    pub sectors: uint16_t,
    pub heads: uint16_t,
    pub dir_len: uint16_t,
    pub cluster_size: uint8_t,
    pub fat_len: uint32_t,
    pub media: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xdf_info {
    pub FatSize: u32,
    pub RootDirSize: uint16_t,
    pub BadSectors: u32,
}
#[no_mangle]
pub unsafe extern "C" fn sectorsToBytes(
    mut This: *mut Fs_t,
    mut off: uint32_t,
) -> mt_off_t {
    return (off as mt_off_t) << (*This).sectorShift;
}
unsafe extern "C" fn read_boot(
    mut Stream: *mut Stream_t,
    mut boot: *mut bootsector,
    mut size: size_t,
) -> i32 {
    let mut boot_sector_size: size_t = 0;
    if size == 0 {
        size = 256 as i32 as size_t;
    }
    if size > 4096 as i32 as u64 {
        size = 4096 as i32 as size_t;
    }
    if force_pread(Stream, ((*boot).characters).as_mut_ptr(), 0 as i32 as mt_off_t, size)
        != size as ssize_t
    {
        return -(1 as i32);
    }
    boot_sector_size = ((*boot).boot.secsiz[0 as i32 as usize] as i32
        + (((*boot).boot.secsiz[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
        as size_t;
    if boot_sector_size < ::core::mem::size_of::<[u8; 4096]>() as u64 {
        memset(
            ((*boot).bytes).as_mut_ptr().offset(boot_sector_size as isize)
                as *mut libc::c_void,
            0 as i32,
            (::core::mem::size_of::<[u8; 4096]>() as u64).wrapping_sub(boot_sector_size),
        );
    }
    return 0 as i32;
}
unsafe extern "C" fn fs_flush(mut Stream: *mut Stream_t) -> i32 {
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    fat_write(This);
    return 0 as i32;
}
unsafe extern "C" fn get_dosConvert(mut Stream: *mut Stream_t) -> *mut doscp_t {
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    return (*This).cp;
}
#[no_mangle]
pub static mut FsClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: None,
            write: None,
            pread: Some(
                pread_pass_through
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                pwrite_pass_through
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut i8,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            flush: Some(fs_flush as unsafe extern "C" fn(*mut Stream_t) -> i32),
            freeFunc: Some(fs_free as unsafe extern "C" fn(*mut Stream_t) -> i32),
            set_geom: None,
            get_data: Some(
                get_data_pass_through
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut time_t,
                        *mut mt_off_t,
                        *mut i32,
                        *mut uint32_t,
                    ) -> i32,
            ),
            pre_allocate: None,
            get_dosConvert: Some(
                get_dosConvert as unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t,
            ),
            discard: None,
        };
        init
    }
};
unsafe extern "C" fn get_media_type(
    mut St: *mut Stream_t,
    mut boot: *mut bootsector,
) -> i32 {
    let mut media: i32 = 0;
    media = (*boot).boot.descr as i32;
    if media < 0xf0 as i32 {
        let mut temp: [i8; 512] = [0; 512];
        if force_pread(
            St,
            temp.as_mut_ptr(),
            512 as i32 as mt_off_t,
            512 as i32 as size_t,
        ) == 512 as i32 as i64
        {
            media = temp[0 as i32 as usize] as u8 as i32;
        } else {
            media = 0 as i32;
        }
    } else {
        media += 0x100 as i32;
    }
    return media;
}
#[no_mangle]
pub unsafe extern "C" fn GetFs(mut Fs: *mut Stream_t) -> *mut Stream_t {
    while !Fs.is_null() && (*Fs).Class != &mut FsClass as *mut Class_t {
        Fs = (*Fs).Next;
    }
    return Fs;
}
unsafe extern "C" fn boot_to_geom(
    mut dev: *mut device,
    mut media: i32,
    mut boot: *mut bootsector,
) {
    let mut tot_sectors: uint32_t = 0;
    let mut BootP: i32 = 0;
    let mut Infp0: i32 = 0;
    let mut InfpX: i32 = 0;
    let mut InfTm: i32 = 0;
    let mut j: i32 = 0;
    let mut sum: u8 = 0;
    let mut sect_per_track: uint16_t = 0;
    let mut labelBlock: *mut label_blk_t = 0 as *mut label_blk_t;
    (*dev).ssize = 2 as i32 as uint8_t;
    (*dev).use_2m = 0x80 as i32 as u32;
    if media == 0xf0 as i32 || media >= 0x100 as i32 {
        (*dev).heads = ((*boot).boot.nheads[0 as i32 as usize] as i32
            + (((*boot).boot.nheads[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t;
        (*dev).sectors = ((*boot).boot.nsect[0 as i32 as usize] as i32
            + (((*boot).boot.nsect[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t;
        tot_sectors = (((*boot).boot.bigsect[0 as i32 as usize] as i32
            + (((*boot).boot.bigsect[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
            as i32
            + (((*((*boot).boot.bigsect)
                .as_mut_ptr()
                .offset(2 as i32 as isize)
                .offset(0 as i32 as isize) as i32
                + ((*((*boot).boot.bigsect)
                    .as_mut_ptr()
                    .offset(2 as i32 as isize)
                    .offset(1 as i32 as isize) as i32) << 8 as i32)) as uint16_t as i32)
                << 16 as i32)) as uint32_t;
        if ((*boot).boot.psect[0 as i32 as usize] as i32
            + (((*boot).boot.psect[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
            != 0
        {
            tot_sectors = ((*boot).boot.psect[0 as i32 as usize] as i32
                + (((*boot).boot.psect[1 as i32 as usize] as i32) << 8 as i32))
                as uint16_t as uint32_t;
        }
        sect_per_track = ((*dev).heads as i32 * (*dev).sectors as i32) as uint16_t;
        if sect_per_track as i32 == 0 as i32 {
            if mtools_skip_check != 0 {
                (*dev).heads = 1 as i32 as uint16_t;
                (*dev).sectors = 1 as i32 as uint16_t;
                sect_per_track = 1 as i32 as uint16_t;
            } else {
                fprintf(
                    stderr,
                    b"The devil is in the details: zero number of heads or sectors\n\0"
                        as *const u8 as *const i8,
                );
                exit(1 as i32);
            }
        }
        (*dev).tracks = tot_sectors.wrapping_div(sect_per_track as u32);
        if tot_sectors.wrapping_rem(sect_per_track as u32) != 0 {
            (*dev).tracks = ((*dev).tracks).wrapping_add(1);
            (*dev).tracks;
        }
        BootP = ((*boot).boot.ext.old.BootP[0 as i32 as usize] as i32
            + (((*boot).boot.ext.old.BootP[1 as i32 as usize] as i32) << 8 as i32))
            as uint16_t as i32;
        Infp0 = ((*boot).boot.ext.old.Infp0[0 as i32 as usize] as i32
            + (((*boot).boot.ext.old.Infp0[1 as i32 as usize] as i32) << 8 as i32))
            as uint16_t as i32;
        InfpX = ((*boot).boot.ext.old.InfpX[0 as i32 as usize] as i32
            + (((*boot).boot.ext.old.InfpX[1 as i32 as usize] as i32) << 8 as i32))
            as uint16_t as i32;
        InfTm = ((*boot).boot.ext.old.InfTm[0 as i32 as usize] as i32
            + (((*boot).boot.ext.old.InfTm[1 as i32 as usize] as i32) << 8 as i32))
            as uint16_t as i32;
        if ((*boot).boot.fatlen[0 as i32 as usize] as i32
            + (((*boot).boot.fatlen[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
            != 0
        {
            labelBlock = &mut (*boot).boot.ext.old.labelBlock;
        } else {
            labelBlock = &mut (*boot).boot.ext.fat32.labelBlock;
        }
        if (*boot).boot.descr as i32 >= 0xf0 as i32
            && ((*labelBlock).dos4 as i32 == 0x28 as i32
                || (*labelBlock).dos4 as i32 == 0x29 as i32)
            && strncmp(
                ((*boot).boot.banner).as_mut_ptr(),
                b"2M\0" as *const u8 as *const i8,
                2 as i32 as u64,
            ) == 0 as i32 && BootP < 512 as i32 && Infp0 < 512 as i32
            && InfpX < 512 as i32 && InfTm < 512 as i32 && BootP >= InfTm + 2 as i32
            && InfTm >= InfpX && InfpX >= Infp0 && Infp0 >= 76 as i32
        {
            sum = 0 as i32 as u8;
            j = 63 as i32;
            while j < BootP {
                sum = (sum as i32 + (*boot).bytes[j as usize] as i32) as u8;
                j += 1;
                j;
            }
            (*dev).ssize = (*boot).bytes[InfTm as usize];
            if sum == 0 && (*dev).ssize as i32 <= 7 as i32 {
                (*dev).use_2m = 0xff as i32 as u32;
                (*dev).ssize = ((*dev).ssize as i32 | 0x80 as i32) as uint8_t;
            }
        }
        (*dev).sector_size = ((*boot).boot.secsiz[0 as i32 as usize] as i32
            + (((*boot).boot.secsiz[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t;
    } else if setDeviceFromOldDos(media, dev) < 0 as i32 {
        exit(1 as i32);
    }
}
unsafe extern "C" fn try_device(
    mut dev: *mut device,
    mut mode: i32,
    mut out_dev: *mut device,
    mut boot: *mut bootsector,
    mut name: *mut i8,
    mut media: *mut i32,
    mut maxSize: *mut mt_off_t,
    mut isRop: *mut i32,
    mut try_writable: i32,
    mut errmsg: *mut i8,
) -> *mut Stream_t {
    let mut retry_write: i32 = 0;
    let mut have_read_bootsector: i32 = 0 as i32;
    let mut modeFlags: i32 = mode & !(0o3 as i32);
    let mut openMode: i32 = 0;
    let mut lockMode: i32 = 0;
    *out_dev = *dev;
    expand((*dev).name, name);
    if try_writable != 0 {
        openMode = 0o2 as i32 | modeFlags;
    } else {
        openMode = mode;
    }
    lockMode = openMode;
    retry_write = 0 as i32;
    while retry_write < 2 as i32 {
        let mut Stream: *mut Stream_t = 0 as *mut Stream_t;
        let mut r: i32 = 0;
        let mut geomFailure: i32 = 0 as i32;
        if retry_write != 0 {
            mode |= 0o2 as i32;
        }
        Stream = OpenImage(
            out_dev,
            dev,
            name,
            openMode,
            errmsg,
            0 as i32,
            lockMode,
            maxSize,
            &mut geomFailure,
            0 as *mut xdf_info,
        );
        if Stream.is_null() {
            if geomFailure != 0 && mode & 0o3 as i32 == 0 as i32 {
                openMode = modeFlags | 0o2 as i32;
            } else if try_writable != 0
                && (*__errno_location() == 1 as i32 || *__errno_location() == 13 as i32
                    || *__errno_location() == 30 as i32)
            {
                openMode = modeFlags | 0 as i32;
                lockMode = openMode;
            } else {
                return 0 as *mut Stream_t
            }
        } else {
            if have_read_bootsector == 0 {
                r = read_boot(Stream, boot, (*out_dev).blocksize as size_t);
                if r < 0 as i32 {
                    sprintf(
                        errmsg,
                        b"init %c: could not read boot sector\0" as *const u8
                            as *const i8,
                        (*dev).drive as i32,
                    );
                    free_stream(&mut Stream);
                    return 0 as *mut Stream_t;
                }
                *media = get_media_type(Stream, boot);
                if *media <= 0xf0 as i32 {
                    if (*boot).boot.jump[2 as i32 as usize] as i32 == 'L' as i32 {
                        sprintf(
                            errmsg,
                            b"diskette %c: is Linux LILO, not DOS\0" as *const u8
                                as *const i8,
                            (*dev).drive as i32,
                        );
                    } else {
                        sprintf(
                            errmsg,
                            b"init %c: non DOS media\0" as *const u8 as *const i8,
                            (*dev).drive as i32,
                        );
                    }
                    free_stream(&mut Stream);
                    return 0 as *mut Stream_t;
                }
                have_read_bootsector = 1 as i32;
            }
            *__errno_location() = 0 as i32;
            boot_to_geom(out_dev, *media, boot);
            if ((*(*Stream).Class).set_geom)
                .expect("non-null function pointer")(Stream, out_dev, dev) != 0
            {
                if *__errno_location() == 9 as i32 || *__errno_location() == 1 as i32 {
                    free_stream(&mut Stream);
                    openMode = modeFlags | 0o2 as i32;
                } else {
                    if *__errno_location() != 0 {
                        snprintf(
                            errmsg,
                            199 as i32 as u64,
                            b"Can't set disk parameters for %c: %s\0" as *const u8
                                as *const i8,
                            (*dev).drive as i32,
                            strerror(*__errno_location()),
                        );
                    } else {
                        sprintf(
                            errmsg,
                            b"Can't set disk parameters for %c\0" as *const u8
                                as *const i8,
                            (*dev).drive as i32,
                        );
                    }
                    free_stream(&mut Stream);
                    return 0 as *mut Stream_t;
                }
            } else {
                if !isRop.is_null() {
                    *isRop = (openMode & 0o3 as i32 == 0 as i32) as i32;
                }
                return Stream;
            }
        }
        retry_write += 1;
        retry_write;
    }
    return 0 as *mut Stream_t;
}
#[no_mangle]
pub unsafe extern "C" fn calc_clus_start(mut Fs: *mut Fs_t) -> uint32_t {
    return ((*Fs).fat_start as u32)
        .wrapping_add(((*Fs).fat_len).wrapping_mul((*Fs).num_fat as u32))
        .wrapping_add((*Fs).dir_len as u32);
}
#[no_mangle]
pub unsafe extern "C" fn calc_num_clus(
    mut Fs: *mut Fs_t,
    mut tot_sectors: uint32_t,
) -> i32 {
    (*Fs).clus_start = calc_clus_start(Fs);
    if tot_sectors <= (*Fs).clus_start {
        return -(1 as i32);
    }
    (*Fs).num_clus = tot_sectors
        .wrapping_sub((*Fs).clus_start)
        .wrapping_div((*Fs).cluster_size as u32);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn find_device(
    mut drive: i8,
    mut mode: i32,
    mut out_dev: *mut device,
    mut boot: *mut bootsector,
    mut name: *mut i8,
    mut media: *mut i32,
    mut maxSize: *mut mt_off_t,
    mut isRop: *mut i32,
) -> *mut Stream_t {
    let mut errmsg: [i8; 200] = [0; 200];
    let mut dev: *mut device = 0 as *mut device;
    sprintf(
        errmsg.as_mut_ptr(),
        b"Drive '%c:' not supported\0" as *const u8 as *const i8,
        drive as i32,
    );
    dev = devices;
    while !((*dev).name).is_null() {
        let mut Stream: *mut Stream_t = 0 as *mut Stream_t;
        let mut isRo: i32 = 0;
        isRo = 0 as i32;
        if !((*dev).drive as i32 != drive as i32) {
            Stream = try_device(
                dev,
                mode,
                out_dev,
                boot,
                name,
                media,
                maxSize,
                &mut isRo,
                (isRop != 0 as *mut libc::c_void as *mut i32) as i32,
                errmsg.as_mut_ptr(),
            );
            if !Stream.is_null() {
                if !isRop.is_null() {
                    *isRop = isRo;
                }
                return Stream;
            }
        }
        dev = dev.offset(1);
        dev;
    }
    fprintf(stderr, b"%s\n\0" as *const u8 as *const i8, errmsg.as_mut_ptr());
    return 0 as *mut Stream_t;
}
#[no_mangle]
pub unsafe extern "C" fn parseFsParams(
    mut This: *mut Fs_t,
    mut boot: *mut bootsector,
    mut media: i32,
    mut cylinder_size: u32,
) -> uint32_t {
    let mut tot_sectors: uint32_t = 0;
    let mut haveBigFatLen: i32 = 0 as i32;
    if media & !(7 as i32) == 0xf8 as i32 {
        let mut params: *mut OldDos_t = getOldDosByMedia(media);
        if params.is_null() {
            fprintf(
                stderr,
                b"Unknown media byte %02x\n\0" as *const u8 as *const i8,
                media,
            );
            return 0 as i32 as uint32_t;
        }
        (*This).cluster_size = (*params).cluster_size;
        tot_sectors = cylinder_size.wrapping_mul((*params).tracks);
        (*This).fat_start = 1 as i32 as uint16_t;
        (*This).fat_len = (*params).fat_len;
        (*This).dir_len = (*params).dir_len;
        (*This).num_fat = 2 as i32 as uint8_t;
        (*This).sector_size = 512 as i32 as uint16_t;
        (*This).sectorShift = 9 as i32 as u32;
        (*This).sectorMask = 511 as i32 as u32;
    } else {
        let mut labelBlock: *mut label_blk_t = 0 as *mut label_blk_t;
        let mut i: u32 = 0;
        (*This).sector_size = ((*boot).boot.secsiz[0 as i32 as usize] as i32
            + (((*boot).boot.secsiz[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t;
        if (*This).sector_size as i32 > 8192 as i32 {
            fprintf(stderr, b"init: sector size too big\n\0" as *const u8 as *const i8);
            return 0 as i32 as uint32_t;
        }
        i = log_2((*This).sector_size as u32);
        if i == 24 as i32 as u32 {
            fprintf(
                stderr,
                b"init: sector size (%d) not a small power of two\n\0" as *const u8
                    as *const i8,
                (*This).sector_size as i32,
            );
            return 0 as i32 as uint32_t;
        }
        (*This).sectorShift = i;
        (*This).sectorMask = ((*This).sector_size as i32 - 1 as i32) as u32;
        tot_sectors = ((*boot).boot.psect[0 as i32 as usize] as i32
            + (((*boot).boot.psect[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
            as uint32_t;
        if tot_sectors == 0 {
            tot_sectors = (((*boot).boot.bigsect[0 as i32 as usize] as i32
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
                    as i32) << 16 as i32)) as uint32_t;
        }
        (*This).cluster_size = (*boot).boot.clsiz;
        (*This).fat_start = ((*boot).boot.nrsvsect[0 as i32 as usize] as i32
            + (((*boot).boot.nrsvsect[1 as i32 as usize] as i32) << 8 as i32))
            as uint16_t;
        (*This).fat_len = ((*boot).boot.fatlen[0 as i32 as usize] as i32
            + (((*boot).boot.fatlen[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
            as uint32_t;
        (*This).dir_len = (((*boot).boot.dirents[0 as i32 as usize] as i32
            + (((*boot).boot.dirents[1 as i32 as usize] as i32) << 8 as i32)) as uint16_t
            as i32 * 32 as i32 / (*This).sector_size as i32) as uint16_t;
        (*This).num_fat = (*boot).boot.nfat;
        if (*This).fat_len != 0 {
            labelBlock = &mut (*boot).boot.ext.old.labelBlock;
        } else {
            labelBlock = &mut (*boot).boot.ext.fat32.labelBlock;
            (*This).fat_len = (((*boot).boot.ext.fat32.bigFat[0 as i32 as usize] as i32
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
                    as i32) << 16 as i32)) as uint32_t;
            haveBigFatLen = 1 as i32;
            (*This).backupBoot = ((*boot).boot.ext.fat32.backupBoot[0 as i32 as usize]
                as i32
                + (((*boot).boot.ext.fat32.backupBoot[1 as i32 as usize] as i32)
                    << 8 as i32)) as uint16_t;
        }
        if (*labelBlock).dos4 as i32 == 0x28 as i32
            || (*labelBlock).dos4 as i32 == 0x29 as i32
        {
            (*This).serialized = 1 as i32;
            (*This).serial_number = (((*labelBlock).serial[0 as i32 as usize] as i32
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
                    as i32) << 16 as i32)) as uint32_t as u64;
        }
    }
    if calc_num_clus(This, tot_sectors) < 0 as i32 {
        return 0 as i32 as uint32_t;
    }
    set_fat(This, haveBigFatLen != 0);
    return tot_sectors;
}
#[no_mangle]
pub unsafe extern "C" fn fs_init(
    mut drive: i8,
    mut mode: i32,
    mut isRop: *mut i32,
) -> *mut Stream_t {
    let mut blocksize: uint32_t = 0;
    let mut media: i32 = 0;
    let mut disk_size: size_t = 0 as i32 as size_t;
    let mut tot_sectors: uint32_t = 0;
    let mut name: [i8; 2048] = [0; 2048];
    let mut cylinder_size: u32 = 0;
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
    let mut maxSize: mt_off_t = 0;
    let mut errmsg: [i8; 81] = [0; 81];
    let mut boot: bootsector = bootsector { bytes: [0; 4096] };
    let mut This: *mut Fs_t = 0 as *mut Fs_t;
    This = calloc(1 as i32 as u64, ::core::mem::size_of::<Fs_t>() as u64) as *mut Fs_t;
    if This.is_null() {
        return 0 as *mut Stream_t;
    }
    init_head(&mut (*This).head, &mut FsClass, 0 as *mut Stream_t);
    (*This).preallocatedClusters = 0 as i32 as u32;
    (*This).lastFatSectorNr = 0 as i32 as uint32_t;
    (*This).lastFatAccessMode = fatAccessMode_t::FAT_ACCESS_READ;
    (*This).lastFatSectorData = 0 as *mut u8;
    (*This).drive = drive;
    (*This).last = 0 as i32 as uint32_t;
    (*This).head.Next = find_device(
        drive,
        mode,
        &mut dev,
        &mut boot,
        name.as_mut_ptr(),
        &mut media,
        &mut maxSize,
        isRop,
    );
    if ((*This).head.Next).is_null() {
        return 0 as *mut Stream_t;
    }
    cylinder_size = (dev.heads as i32 * dev.sectors as i32) as u32;
    (*This).serialized = 0 as i32;
    tot_sectors = parseFsParams(This, &mut boot, media, cylinder_size);
    if tot_sectors == 0 as i32 as u32 {
        return 0 as *mut Stream_t;
    }
    if check_if_sectors_fit(
        tot_sectors,
        maxSize,
        (*This).sector_size as uint32_t,
        errmsg.as_mut_ptr(),
    ) < 0 as i32
    {
        fprintf(stderr, b"%s\0" as *const u8 as *const i8, errmsg.as_mut_ptr());
        return 0 as *mut Stream_t;
    }
    disk_size = (if dev.tracks != 0 { cylinder_size } else { 512 as i32 as u32 })
        as size_t;
    disk_size = cylinder_size as size_t;
    if disk_size > 256 as i32 as u64 {
        disk_size = dev.sectors as size_t;
        if dev.sectors as i32 % 2 as i32 != 0 {
            disk_size <<= 1 as i32;
        }
    }
    if disk_size.wrapping_rem(2 as i32 as u64) != 0 {
        disk_size = (disk_size as u64).wrapping_mul(2 as i32 as u64) as size_t as size_t;
    }
    if dev.blocksize == 0 || dev.blocksize < (*This).sector_size as u32 {
        blocksize = (*This).sector_size as uint32_t;
    } else {
        blocksize = dev.blocksize;
    }
    if disk_size != 0 {
        let mut Buffer: *mut Stream_t = buf_init(
            (*This).head.Next,
            disk_size.wrapping_mul(blocksize as u64),
            disk_size.wrapping_mul(blocksize as u64),
            (*This).sector_size as size_t,
        );
        if !Buffer.is_null() {
            (*This).head.Next = Buffer;
        } else {
            perror(b"init: allocate buffer\0" as *const u8 as *const i8);
        }
    }
    if fat_read(This, &mut boot, (dev.use_2m & 0x7f as i32 as u32) as i32) != 0 {
        fprintf(stderr, b"Error reading FAT\n\0" as *const u8 as *const i8);
        (*This).num_fat = 1 as i32 as uint8_t;
        free_stream(&mut (*This).head.Next);
        free((*This).head.Next as *mut i8 as *mut libc::c_void);
        return 0 as *mut Stream_t;
    }
    (*This).cp = cp_open(dev.codepage);
    if ((*This).cp).is_null() {
        fprintf(stderr, b"Error setting code page\n\0" as *const u8 as *const i8);
        fs_free(This as *mut Stream_t);
        free_stream(&mut (*This).head.Next);
        free((*This).head.Next as *mut i8 as *mut libc::c_void);
        return 0 as *mut Stream_t;
    }
    return This as *mut Stream_t;
}
#[no_mangle]
pub unsafe extern "C" fn getDrive(mut Stream: *mut Stream_t) -> i8 {
    let mut This: *mut Fs_t = Stream as *mut Fs_t;
    if (*This).head.Class != &mut FsClass as *mut Class_t {
        return getDrive(GetFs(Stream))
    } else {
        return (*This).drive
    };
}
#[no_mangle]
pub unsafe extern "C" fn fsPreallocateClusters(
    mut Fs: *mut Fs_t,
    mut size: uint32_t,
) -> i32 {
    if size > 0 as i32 as u32
        && getfreeMinClusters(Fs as *mut Stream_t, size) != 1 as i32
    {
        return -(1 as i32);
    }
    (*Fs).preallocatedClusters = ((*Fs).preallocatedClusters).wrapping_add(size);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn fsReleasePreallocateClusters(
    mut Fs: *mut Fs_t,
    mut size: uint32_t,
) {
    (*Fs).preallocatedClusters = ((*Fs).preallocatedClusters).wrapping_sub(size);
}
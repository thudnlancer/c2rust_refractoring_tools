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
    fn free_stream(Stream: *mut *mut Stream_t) -> i32;
    static max_off_t_31: mt_off_t;
    fn compute_lba_geom_from_tot_sectors(dev: *mut device) -> i32;
    fn XdfOpen(
        dev: *mut device,
        name: *const i8,
        mode: i32,
        errmsg: *mut i8,
        info: *mut xdf_info,
    ) -> *mut Stream_t;
    fn SimpleFileOpenWithLm(
        dev: *mut device,
        orig_dev: *mut device,
        name: *const i8,
        mode: i32,
        errmsg: *mut i8,
        mode2: i32,
        locked: i32,
        lockMode: i32,
        maxSize: *mut mt_off_t,
        geomFailure: *mut i32,
    ) -> *mut Stream_t;
    fn FloppydOpen(
        dev: *mut device,
        name: *const i8,
        mode: i32,
        errmsg: *mut i8,
        maxSize: *mut mt_off_t,
    ) -> *mut Stream_t;
    fn OpenScsi(
        dev: *mut device,
        name: *const i8,
        mode: i32,
        errmsg: *mut i8,
        mode2: i32,
        locked: i32,
        lockMode: i32,
        maxSize: *mut mt_off_t,
    ) -> *mut Stream_t;
    fn Remap(Next: *mut Stream_t, dev: *mut device, errmsg: *mut i8) -> *mut Stream_t;
    fn OpenPartition(
        Next: *mut Stream_t,
        dev: *mut device,
        errmsg: *mut i8,
        maxSize: *mut mt_off_t,
    ) -> *mut Stream_t;
    fn OpenOffset(
        Next: *mut Stream_t,
        dev: *mut device,
        offset: off_t,
        errmsg: *mut i8,
        maxSize: *mut mt_off_t,
    ) -> *mut Stream_t;
    fn OpenSwap(Next: *mut Stream_t) -> *mut Stream_t;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type __time_t = i64;
pub type __ssize_t = i64;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
#[no_mangle]
pub unsafe extern "C" fn OpenImage(
    mut out_dev: *mut device,
    mut dev: *mut device,
    mut name: *const i8,
    mut mode: i32,
    mut errmsg: *mut i8,
    mut flags: i32,
    mut lockMode: i32,
    mut maxSize: *mut mt_off_t,
    mut geomFailureP: *mut i32,
    mut xdf_info: *mut xdf_info,
) -> *mut Stream_t {
    let mut current_block: u64;
    let mut Stream: *mut Stream_t = 0 as *mut Stream_t;
    let mut geomFailure: i32 = 0 as i32;
    if (*out_dev).misc_flags & 0x40 as u32 != 0 {
        Stream = FloppydOpen(out_dev, name, mode, errmsg, maxSize);
    } else {
        Stream = XdfOpen(out_dev, name, mode, errmsg, xdf_info);
        if !Stream.is_null() {
            (*out_dev).use_2m = 0x7f as i32 as u32;
            if !maxSize.is_null() {
                *maxSize = max_off_t_31;
            }
        }
        if Stream.is_null() {
            Stream = OpenScsi(
                out_dev,
                name,
                mode,
                errmsg,
                flags,
                0 as i32,
                lockMode,
                maxSize,
            );
        }
        if Stream.is_null() {
            Stream = SimpleFileOpenWithLm(
                out_dev,
                dev,
                name,
                mode,
                errmsg,
                flags,
                0 as i32,
                lockMode,
                maxSize,
                &mut geomFailure,
            );
        }
        if geomFailure != 0 {
            if *geomFailureP != 0 {
                *geomFailureP = geomFailure;
            }
            return 0 as *mut Stream_t;
        }
    }
    if Stream.is_null() {
        return 0 as *mut Stream_t;
    }
    if !((*dev).data_map).is_null() {
        let mut Remapped: *mut Stream_t = Remap(Stream, out_dev, errmsg);
        if Remapped.is_null() {
            current_block = 16644063200701690463;
        } else {
            Stream = Remapped;
            current_block = 5783071609795492627;
        }
    } else {
        current_block = 5783071609795492627;
    }
    match current_block {
        5783071609795492627 => {
            if (*dev).offset != 0 {
                let mut Offset: *mut Stream_t = OpenOffset(
                    Stream,
                    out_dev,
                    (*dev).offset,
                    errmsg,
                    maxSize,
                );
                if Offset.is_null() {
                    current_block = 16644063200701690463;
                } else {
                    Stream = Offset;
                    current_block = 9828876828309294594;
                }
            } else {
                current_block = 9828876828309294594;
            }
            match current_block {
                16644063200701690463 => {}
                _ => {
                    if !dev.is_null() && (*dev).misc_flags & 0x100 as u32 != 0 {
                        let mut Swap: *mut Stream_t = OpenSwap(Stream);
                        if Swap.is_null() {
                            current_block = 16644063200701690463;
                        } else {
                            Stream = Swap;
                            current_block = 1608152415753874203;
                        }
                    } else {
                        current_block = 1608152415753874203;
                    }
                    match current_block {
                        16644063200701690463 => {}
                        _ => {
                            if !(flags & 4 as i32 != 0
                                && compute_lba_geom_from_tot_sectors(out_dev) < 0 as i32)
                            {
                                if (*dev).partition != 0 && flags & 2 as i32 == 0 {
                                    let mut Partition: *mut Stream_t = OpenPartition(
                                        Stream,
                                        out_dev,
                                        errmsg,
                                        maxSize,
                                    );
                                    if Partition.is_null() {
                                        current_block = 16644063200701690463;
                                    } else {
                                        Stream = Partition;
                                        current_block = 11913429853522160501;
                                    }
                                } else {
                                    current_block = 11913429853522160501;
                                }
                                match current_block {
                                    16644063200701690463 => {}
                                    _ => return Stream,
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    free_stream(&mut Stream);
    return 0 as *mut Stream_t;
}
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type off_t = __off_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
#[no_mangle]
pub unsafe extern "C" fn compute_lba_geom_from_tot_sectors(mut dev: *mut device) -> i32 {
    let mut sect_per_track: u32 = 0;
    let mut tracks: uint32_t = 0;
    if (*dev).heads as i32 != 0 && (*dev).sectors as i32 != 0 && (*dev).tracks != 0 {
        return 0 as i32;
    }
    if (*dev).tot_sectors == 0 as i32 as u32 {
        return 0 as i32;
    }
    if (*dev).tot_sectors != 0 && (*dev).tot_sectors <= 8640 as i32 as u32
        && ((*dev).tot_sectors).wrapping_rem(40 as i32 as u32) == 0 as i32 as u32
    {
        if (*dev).tot_sectors <= 540 as i32 as u32 {
            (*dev).tracks = 40 as i32 as u32;
            (*dev).heads = 1 as i32 as uint16_t;
        } else if (*dev).tot_sectors <= 1080 as i32 as u32 {
            if (*dev).heads as i32 == 1 as i32 {
                (*dev).tracks = 80 as i32 as u32;
            } else {
                (*dev).tracks = 40 as i32 as u32;
                (*dev).heads = 2 as i32 as uint16_t;
            }
        } else {
            (*dev).tracks = 80 as i32 as u32;
            (*dev).heads = 2 as i32 as uint16_t;
        }
        (*dev).sectors = ((*dev).tot_sectors)
            .wrapping_div((*dev).heads as u32)
            .wrapping_div((*dev).tracks) as uint16_t;
    }
    if (*dev).sectors == 0 || (*dev).heads == 0 {
        (*dev).sectors = 63 as i32 as uint16_t;
        if (*dev).tot_sectors
            < (16 as u32)
                .wrapping_mul((*dev).sectors as u32)
                .wrapping_mul(1024 as i32 as u32)
        {
            (*dev).heads = 16 as i32 as uint16_t;
        } else if (*dev).tot_sectors
            < (32 as u32)
                .wrapping_mul((*dev).sectors as u32)
                .wrapping_mul(1024 as i32 as u32)
        {
            (*dev).heads = 32 as i32 as uint16_t;
        } else if (*dev).tot_sectors
            < (64 as u32)
                .wrapping_mul((*dev).sectors as u32)
                .wrapping_mul(1024 as i32 as u32)
        {
            (*dev).heads = 64 as i32 as uint16_t;
        } else if (*dev).tot_sectors
            < (128 as u32)
                .wrapping_mul((*dev).sectors as u32)
                .wrapping_mul(1024 as i32 as u32)
        {
            (*dev).heads = 128 as i32 as uint16_t;
        } else {
            (*dev).heads = 255 as i32 as uint16_t;
        }
    }
    if (*dev).tracks == 0 {
        sect_per_track = ((*dev).heads as i32 * (*dev).sectors as i32) as u32;
        tracks = ((*dev).tot_sectors)
            .wrapping_add(sect_per_track)
            .wrapping_sub(1 as i32 as u32)
            .wrapping_div(sect_per_track);
        (*dev).tracks = tracks;
    }
    return 0 as i32;
}
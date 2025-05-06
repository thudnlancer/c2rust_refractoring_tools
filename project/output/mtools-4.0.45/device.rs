#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type off_t = __off_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type mt_off_t = off_t;
pub type smt_off_t = mt_off_t;
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
pub unsafe extern "C" fn check_if_sectors_fit(
    mut tot_sectors: uint32_t,
    mut maxBytes: mt_off_t,
    mut sectorSize: uint32_t,
    mut errmsg: *mut i8,
) -> i32 {
    if maxBytes == 0 {
        return 0 as i32;
    }
    if tot_sectors as i64 > maxBytes / sectorSize as smt_off_t {
        sprintf(
            errmsg,
            b"%d sectors too large for this platform\n\0" as *const u8 as *const i8,
            tot_sectors,
        );
        return -(1 as i32);
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn chs_to_totsectors(
    mut dev: *mut device,
    mut errmsg: *mut i8,
) -> i32 {
    let mut sect_per_track: uint32_t = 0;
    let mut tot_sectors: uint32_t = 0;
    if (*dev).tot_sectors != 0 {
        return 0 as i32;
    }
    if (*dev).heads == 0 || (*dev).sectors == 0 || (*dev).tracks == 0 {
        return 0 as i32;
    }
    sect_per_track = ((*dev).heads as i32 * (*dev).sectors as i32) as uint32_t;
    if (*dev).tracks > (4294967295 as u32).wrapping_div(sect_per_track) {
        if !errmsg.is_null() {
            sprintf(
                errmsg,
                b"Number of sectors larger than 2^32\n\0" as *const u8 as *const i8,
            );
        }
        return -(1 as i32);
    }
    tot_sectors = ((*dev).tracks).wrapping_mul(sect_per_track);
    if tot_sectors > ((*dev).hidden).wrapping_rem(sect_per_track) {
        tot_sectors = (tot_sectors as u32)
            .wrapping_sub(((*dev).hidden).wrapping_rem(sect_per_track)) as uint32_t
            as uint32_t;
    }
    (*dev).tot_sectors = tot_sectors;
    return 0 as i32;
}
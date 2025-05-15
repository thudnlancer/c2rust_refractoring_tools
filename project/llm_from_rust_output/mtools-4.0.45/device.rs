use std::ffi::CString;
use std::fmt::Write;

#[derive(Clone)]
pub struct Device {
    pub name: Option<CString>,
    pub drive: char,
    pub fat_bits: i32,
    pub mode: i32,
    pub tracks: u32,
    pub heads: u16,
    pub sectors: u16,
    pub hidden: u32,
    pub offset: i64,
    pub partition: u32,
    pub misc_flags: u32,
    pub ssize: u8,
    pub use_2m: u32,
    pub precmd: Option<CString>,
    pub file_nr: i32,
    pub blocksize: u32,
    pub codepage: u32,
    pub data_map: Option<CString>,
    pub tot_sectors: u32,
    pub sector_size: u16,
    pub postcmd: Option<CString>,
    pub cfg_filename: Option<CString>,
}

pub fn check_if_sectors_fit(
    tot_sectors: u32,
    max_bytes: i64,
    sector_size: u32,
) -> Result<(), String> {
    if max_bytes == 0 {
        return Ok(());
    }
    
    if tot_sectors as i64 > max_bytes / sector_size as i64 {
        return Err(format!("{} sectors too large for this platform", tot_sectors));
    }
    
    Ok(())
}

pub fn chs_to_totsectors(dev: &mut Device) -> Result<(), String> {
    if dev.tot_sectors != 0 {
        return Ok(());
    }
    
    if dev.heads == 0 || dev.sectors == 0 || dev.tracks == 0 {
        return Ok(());
    }
    
    let sect_per_track = dev.heads as u32 * dev.sectors as u32;
    
    if dev.tracks > u32::MAX / sect_per_track {
        return Err("Number of sectors larger than 2^32".to_string());
    }
    
    let mut tot_sectors = dev.tracks.wrapping_mul(sect_per_track);
    
    if tot_sectors > dev.hidden.wrapping_rem(sect_per_track) {
        tot_sectors = tot_sectors.wrapping_sub(dev.hidden.wrapping_rem(sect_per_track));
    }
    
    dev.tot_sectors = tot_sectors;
    Ok(())
}
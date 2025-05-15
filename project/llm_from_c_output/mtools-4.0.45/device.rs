// Copyright 2021 Alain Knaff.
// This file is part of mtools.
//
// Mtools is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Mtools is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Mtools.  If not, see <http://www.gnu.org/licenses/>.

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

bitflags::bitflags! {
    pub struct DeviceFlags: u32 {
        const SCSI = 0x001;
        const PRIV = 0x002;
        const NOLOCK = 0x004;
        const USE_XDF = 0x008;
        const MFORMAT_ONLY = 0x010;
        const VOLD = 0x020;
        const FLOPPYD = 0x040;
        const FILTER = 0x080;
        const SWAP = 0x100;
    }
}

#[derive(Debug)]
pub struct Device {
    pub name: Option<String>,          // full path to device
    pub drive: char,                   // the drive letter
    pub fat_bits: i32,                 // FAT encoding scheme
    pub mode: i32,                     // any special open() flags
    pub tracks: u32,                   // tracks
    pub heads: u16,                    // heads
    pub sectors: u16,                  // sectors
    pub hidden: u32,                   // number of hidden sectors
    pub offset: i64,                   // skip this many bytes
    pub partition: u32,                // partition number
    pub misc_flags: DeviceFlags,       // miscellaneous flags
    pub ssize: u8,                     // Linux only: sector size
    pub use_2m: u32,                   // Linux only: use 2m format
    pub precmd: Option<String>,        // command to execute before opening
    pub file_nr: i32,                  // used during parsing
    pub blocksize: u32,                // size of disk block in bytes
    pub codepage: u32,                 // codepage for shortname encoding
    pub data_map: Option<String>,      // data map
    pub tot_sectors: u32,              // total sectors
    pub sector_size: u16,              // non-default sector size
    pub postcmd: Option<String>,       // command to execute after closing
    pub cfg_filename: Option<String>,  // used for debugging
}

impl Device {
    pub fn is_scsi(&self) -> bool {
        self.misc_flags.contains(DeviceFlags::SCSI)
    }

    pub fn is_privileged(&self) -> bool {
        self.misc_flags.contains(DeviceFlags::PRIV)
    }

    pub fn is_nolock(&self) -> bool {
        self.misc_flags.contains(DeviceFlags::NOLOCK)
    }

    pub fn is_mformat_only(&self) -> bool {
        self.misc_flags.contains(DeviceFlags::MFORMAT_ONLY)
    }

    pub fn should_use_vold(&self) -> bool {
        self.misc_flags.contains(DeviceFlags::VOLD)
    }

    pub fn should_use_xdf(&self) -> bool {
        self.misc_flags.contains(DeviceFlags::USE_XDF)
    }

    pub fn do_swap(&self) -> bool {
        self.misc_flags.contains(DeviceFlags::SWAP)
    }
}

pub static mut DEVICES: Option<Vec<Device>> = None;
pub static mut CONST_DEVICES: Option<Vec<Device>> = None;
pub static mut NR_CONST_DEVICES: u32 = 0;

pub fn lock_dev(fd: i32, mode: i32, dev: &Device) -> i32 {
    // Implementation depends on platform-specific details
    // Placeholder for actual implementation
    0
}

pub fn precmd(dev: &Device) {
    if let Some(cmd) = &dev.precmd {
        // Execute the command
    }
}

pub fn postcmd(cmd: &str) {
    // Execute the command
}

pub fn check_if_sectors_fit(
    tot_sectors: u32,
    max_bytes: i64,
    sector_size: u32,
    errmsg: &mut String,
) -> Result<(), String> {
    if max_bytes == 0 {
        return Ok(());
    }
    if tot_sectors > (max_bytes / sector_size as i64) as u32 {
        *errmsg = format!("{} sectors too large for this platform", tot_sectors);
        return Err(errmsg.clone());
    }
    Ok(())
}

pub fn chs_to_totsectors(dev: &mut Device, errmsg: &mut String) -> Result<(), String> {
    if dev.tot_sectors != 0 {
        return Ok(());
    }
    
    if dev.heads == 0 || dev.sectors == 0 || dev.tracks == 0 {
        return Ok(());
    }
    
    let sect_per_track = dev.heads as u32 * dev.sectors as u32;
    
    if dev.tracks > u32::MAX / sect_per_track {
        *errmsg = "Number of sectors larger than 2^32".to_string();
        return Err(errmsg.clone());
    }
    
    let mut tot_sectors = dev.tracks * sect_per_track;
    if tot_sectors > dev.hidden % sect_per_track {
        tot_sectors -= dev.hidden % sect_per_track;
    }
    dev.tot_sectors = tot_sectors;
    Ok(())
}
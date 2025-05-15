use std::ffi::CStr;

type UInt8 = u8;
type UInt16 = u16;
type UInt32 = u32;
type MtOffT = i64;

#[derive(Debug, Clone)]
pub struct Device {
    pub name: Option<String>,
    pub drive: i8,
    pub fat_bits: i32,
    pub mode: i32,
    pub tracks: u32,
    pub heads: UInt16,
    pub sectors: UInt16,
    pub hidden: u32,
    pub offset: MtOffT,
    pub partition: u32,
    pub misc_flags: u32,
    pub ssize: UInt8,
    pub use_2m: u32,
    pub precmd: Option<String>,
    pub file_nr: i32,
    pub blocksize: u32,
    pub codepage: u32,
    pub data_map: Option<String>,
    pub tot_sectors: UInt32,
    pub sector_size: UInt16,
    pub postcmd: Option<String>,
    pub cfg_filename: Option<String>,
}

impl Device {
    pub fn compute_lba_geom_from_tot_sectors(&mut self) -> i32 {
        if self.heads != 0 && self.sectors != 0 && self.tracks != 0 {
            return 0;
        }

        if self.tot_sectors == 0 {
            return 0;
        }

        if self.tot_sectors != 0
            && self.tot_sectors <= 8640
            && self.tot_sectors % 40 == 0
        {
            if self.tot_sectors <= 540 {
                self.tracks = 40;
                self.heads = 1;
            } else if self.tot_sectors <= 1080 {
                if self.heads == 1 {
                    self.tracks = 80;
                } else {
                    self.tracks = 40;
                    self.heads = 2;
                }
            } else {
                self.tracks = 80;
                self.heads = 2;
            }
            self.sectors = (self.tot_sectors / self.heads as u32 / self.tracks) as UInt16;
        }

        if self.sectors == 0 || self.heads == 0 {
            self.sectors = 63;
            if self.tot_sectors < 16 * self.sectors as u32 * 1024 {
                self.heads = 16;
            } else if self.tot_sectors < 32 * self.sectors as u32 * 1024 {
                self.heads = 32;
            } else if self.tot_sectors < 64 * self.sectors as u32 * 1024 {
                self.heads = 64;
            } else if self.tot_sectors < 128 * self.sectors as u32 * 1024 {
                self.heads = 128;
            } else {
                self.heads = 255;
            }
        }

        if self.tracks == 0 {
            let sect_per_track = self.heads as u32 * self.sectors as u32;
            let tracks = (self.tot_sectors + sect_per_track - 1) / sect_per_track;
            self.tracks = tracks;
        }

        0
    }
}
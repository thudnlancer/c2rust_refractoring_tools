/*  Copyright 2021 Alain Knaff.
 *  This file is part of mtools.
 *
 *  Mtools is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Mtools is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Mtools.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct OldDos {
    pub tracks: u32,
    pub sectors: u16,
    pub heads: u16,
    pub dir_len: u16,
    pub cluster_size: u8,
    pub fat_len: u32,
    pub media: u8,
}

#[derive(Debug)]
pub struct Device {
    pub heads: u16,
    pub tracks: u32,
    pub sectors: u16,
    pub ssize: u8,
    pub use_2m: u32,
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Device {{ heads: {}, tracks: {}, sectors: {}, ssize: {}, use_2m: {} }}",
               self.heads, self.tracks, self.sectors, self.ssize, self.use_2m)
    }
}

const OLD_DOS_TABLE: &[OldDos] = &[
    OldDos { tracks: 40, sectors: 9, heads: 1, dir_len: 4, cluster_size: 1, fat_len: 2, media: 0xfc }, /*  180 KB */
    OldDos { tracks: 40, sectors: 9, heads: 2, dir_len: 7, cluster_size: 2, fat_len: 2, media: 0xfd }, /*  360 KB */
    OldDos { tracks: 40, sectors: 8, heads: 1, dir_len: 4, cluster_size: 1, fat_len: 1, media: 0xfe }, /*  160 KB */
    OldDos { tracks: 40, sectors: 8, heads: 2, dir_len: 7, cluster_size: 2, fat_len: 1, media: 0xff }, /*  320 KB */
    OldDos { tracks: 80, sectors: 9, heads: 2, dir_len: 7, cluster_size: 2, fat_len: 3, media: 0xf9 }, /*  720 KB */
    OldDos { tracks: 80, sectors: 15, heads: 2, dir_len: 14, cluster_size: 1, fat_len: 7, media: 0xf9 }, /* 1200 KB */
    OldDos { tracks: 80, sectors: 18, heads: 2, dir_len: 14, cluster_size: 1, fat_len: 9, media: 0xf0 }, /* 1440 KB */
    OldDos { tracks: 80, sectors: 36, heads: 2, dir_len: 15, cluster_size: 2, fat_len: 9, media: 0xf0 }, /* 2880 KB */
    OldDos { tracks: 80, sectors: 8, heads: 2, dir_len: 7, cluster_size: 2, fat_len: 2, media: 0xfb }, /* 640 KB */
    OldDos { tracks: 80, sectors: 8, heads: 1, dir_len: 7, cluster_size: 2, fat_len: 2, media: 0xfa }, /* 320 KB */
    OldDos { tracks: 80, sectors: 9, heads: 1, dir_len: 7, cluster_size: 2, fat_len: 2, media: 0xf8 }, /* 360 KB */
];

/// Get Old Dos parameters for a filesystem of size KBytes (assuming
/// 512 byte sectors), i.e. number of sectors is double the size
pub fn get_old_dos_by_size(size: usize) -> Option<&'static OldDos> {
    let size = size * 2;
    OLD_DOS_TABLE.iter().find(|&params| {
        params.sectors as usize * params.tracks as usize * params.heads as usize == size
    })
}

pub fn get_old_dos_by_media(media: u8) -> Option<&'static OldDos> {
    let params = OLD_DOS_TABLE.iter().find(|&params| params.media == media);
    if params.is_none() {
        eprintln!("Unknown media type {:02x}", media);
    }
    params
}

pub fn get_old_dos_by_params(
    tracks: u32,
    heads: u16,
    sectors: u16,
    dir_len: u16,
    cluster_size: u8,
) -> Option<&'static OldDos> {
    OLD_DOS_TABLE.iter().find(|&params| {
        sectors == params.sectors &&
        tracks == params.tracks &&
        heads == params.heads &&
        (dir_len == 0 || dir_len == params.dir_len) &&
        (cluster_size == 0 || cluster_size == params.cluster_size)
    })
}

pub fn set_device_from_old_dos(media: u8, dev: &mut Device) -> Result<(), &'static str> {
    let params = get_old_dos_by_media(media).ok_or("Unknown media type")?;
    dev.heads = params.heads;
    dev.tracks = params.tracks;
    dev.sectors = params.sectors;
    dev.ssize = 0x80;
    dev.use_2m = !1u32;
    Ok(())
}
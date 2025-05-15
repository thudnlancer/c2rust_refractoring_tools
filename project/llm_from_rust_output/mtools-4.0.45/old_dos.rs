use std::io::{self, Write};

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

const OLD_DOS_TABLE: [OldDos; 11] = [
    OldDos {
        tracks: 40,
        sectors: 9,
        heads: 1,
        dir_len: 4,
        cluster_size: 1,
        fat_len: 2,
        media: 0xfc,
    },
    OldDos {
        tracks: 40,
        sectors: 9,
        heads: 2,
        dir_len: 7,
        cluster_size: 2,
        fat_len: 2,
        media: 0xfd,
    },
    OldDos {
        tracks: 40,
        sectors: 8,
        heads: 1,
        dir_len: 4,
        cluster_size: 1,
        fat_len: 1,
        media: 0xfe,
    },
    OldDos {
        tracks: 40,
        sectors: 8,
        heads: 2,
        dir_len: 7,
        cluster_size: 2,
        fat_len: 1,
        media: 0xff,
    },
    OldDos {
        tracks: 80,
        sectors: 9,
        heads: 2,
        dir_len: 7,
        cluster_size: 2,
        fat_len: 3,
        media: 0xf9,
    },
    OldDos {
        tracks: 80,
        sectors: 15,
        heads: 2,
        dir_len: 14,
        cluster_size: 1,
        fat_len: 7,
        media: 0xf9,
    },
    OldDos {
        tracks: 80,
        sectors: 18,
        heads: 2,
        dir_len: 14,
        cluster_size: 1,
        fat_len: 9,
        media: 0xf0,
    },
    OldDos {
        tracks: 80,
        sectors: 36,
        heads: 2,
        dir_len: 15,
        cluster_size: 2,
        fat_len: 9,
        media: 0xf0,
    },
    OldDos {
        tracks: 80,
        sectors: 8,
        heads: 2,
        dir_len: 7,
        cluster_size: 2,
        fat_len: 2,
        media: 0xfb,
    },
    OldDos {
        tracks: 80,
        sectors: 8,
        heads: 1,
        dir_len: 7,
        cluster_size: 2,
        fat_len: 2,
        media: 0xfa,
    },
    OldDos {
        tracks: 80,
        sectors: 9,
        heads: 1,
        dir_len: 7,
        cluster_size: 2,
        fat_len: 2,
        media: 0xf8,
    },
];

pub fn get_old_dos_by_size(size: usize) -> Option<&'static OldDos> {
    let size = size * 2;
    OLD_DOS_TABLE.iter().find(|&d| {
        (d.sectors as u32) * d.tracks * (d.heads as u32) == size as u32
    })
}

pub fn get_old_dos_by_media(media: i32) -> Option<&'static OldDos> {
    OLD_DOS_TABLE.iter().find(|&d| d.media as i32 == media).or_else(|| {
        writeln!(io::stderr(), "Unknown media type {:02x}", media).ok();
        None
    })
}

pub fn get_old_dos_by_params(
    tracks: u32,
    heads: u32,
    sectors: u32,
    dir_len: u32,
    cluster_size: u32,
) -> Option<&'static OldDos> {
    OLD_DOS_TABLE.iter().find(|&d| {
        sectors == d.sectors as u32
            && tracks == d.tracks
            && heads == d.heads as u32
            && (dir_len == 0 || dir_len == d.dir_len as u32)
            && (cluster_size == 0 || cluster_size == d.cluster_size as u32)
    })
}

#[derive(Debug, Default)]
pub struct Device {
    pub heads: u16,
    pub tracks: u32,
    pub sectors: u16,
    pub ssize: u8,
    pub use_2m: u32,
}

pub fn set_device_from_old_dos(media: i32, dev: &mut Device) -> Result<(), ()> {
    let params = get_old_dos_by_media(media).ok_or(())?;
    dev.heads = params.heads;
    dev.tracks = params.tracks;
    dev.sectors = params.sectors;
    dev.ssize = 0x80;
    dev.use_2m = !1;
    Ok(())
}
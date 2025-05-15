use std::ffi::{CStr, CString};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::os::unix::prelude::*;
use std::path::Path;
use std::ptr;
use std::mem;
use std::process::exit;
use std::slice;

const SECTOR_SIZE: usize = 512;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Partition {
    start: HSC,
    end: HSC,
    start_sect: [u8; 4],
    nr_sects: [u8; 4],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct HSC {
    byte0: u8,
    head: u8,
    sector: u8,
    cyl: u8,
}

#[derive(Debug)]
struct Device {
    name: String,
    drive: char,
    fat_bits: i32,
    mode: i32,
    tracks: u32,
    heads: u16,
    sectors: u16,
    hidden: u32,
    offset: i64,
    partition: u32,
    misc_flags: u32,
    ssize: u8,
    use_2m: u32,
    precmd: Option<String>,
    file_nr: i32,
    blocksize: u32,
    codepage: u32,
    data_map: Option<String>,
    tot_sectors: u32,
    sector_size: u16,
    postcmd: Option<String>,
    cfg_filename: Option<String>,
}

fn set_dword(data: &mut [u8], value: u32) {
    data[3] = (value >> 24 & 0xff) as u8;
    data[2] = (value >> 16 & 0xff) as u8;
    data[1] = (value >> 8 & 0xff) as u8;
    data[0] = (value & 0xff) as u8;
}

fn set_word(data: &mut [u8], value: u16) {
    data[1] = (value >> 8 & 0xff) as u8;
    data[0] = (value & 0xff) as u8;
}

fn set_offset(h: &mut HSC, offset: u64, heads: u16, sectors: u16) {
    let (cyl, head, sector) = if heads == 0 || sectors == 0 {
        (0, 0, 0)
    } else {
        let sector = (offset % sectors as u64) as u16;
        let offset = offset / sectors as u64;
        let head = (offset % heads as u64) as u16;
        let offset = offset / heads as u64;
        let cyl = if offset > 1023 { 1023 } else { offset as u16 };
        (cyl, head, sector)
    };

    let (head, sector, cyl) = if head > 255 {
        (0, 0, 0)
    } else {
        (head as u8, sector as u8, cyl as u8)
    };

    h.head = head;
    h.sector = ((sector as u32 + 1 & 0x3f) | (cyl as u32 & 0x300) >> 2) as u8;
    h.cyl = (cyl & 0xff) as u8;
}

fn set_begin_end(
    part_table: &mut Partition,
    begin: u32,
    end: u32,
    heads: u16,
    sectors: u16,
    activate: bool,
    part_type: u8,
    fat_bits: u32,
) {
    if heads > 255 {
        eprintln!("Too many heads for partition: {}", heads);
        exit(1);
    }
    if sectors > 255 {
        eprintln!("Too many sectors for partition: {}", sectors);
        exit(1);
    }

    set_offset(&mut part_table.start, begin as u64, heads, sectors);
    set_offset(
        &mut part_table.end,
        (end - 1) as u64,
        heads,
        sectors,
    );

    set_dword(&mut part_table.start_sect, begin);
    set_dword(&mut part_table.nr_sects, end - begin);

    part_table.start.byte0 = if activate { 0x80 } else { 0 };

    let part_type = if part_type == 0 {
        let fat_bits = if fat_bits == 0 {
            if end - begin < 4096 {
                12
            } else {
                16
            }
        } else {
            fat_bits
        };

        if fat_bits == 32 {
            0x0c
        } else if end < 65536 {
            match fat_bits {
                12 => 0x01,
                16 => 0x04,
                _ => 0x06,
            }
        } else if end < (sectors as u32 * heads as u32) * 1024 {
            0x06
        } else {
            0x0e
        }
    } else {
        part_type
    };

    part_table.end.byte0 = part_type;
}

fn main() {
    // Main implementation would go here
    // Note: This is a partial translation focusing on the core data structures and functions
    // A complete translation would require implementing all the C functionality in safe Rust
    // including command line parsing, file I/O, error handling, etc.
}
/*  Copyright 1997-2003,2005-2007,2009 Alain Knaff.
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
 *
 * mformat.c
 */

use std::convert::TryFrom;
use std::io::{self, Write};
use std::process;
use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::mem;

#[derive(Debug, Clone, Copy)]
struct Hsc {
    head: u8,
    sector: u8,
    cyl: u8,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Partition {
    boot_ind: u8,
    head: u8,
    sector: u8,
    cyl: u8,
    sys_ind: u8,
    end_head: u8,
    end_sector: u8,
    end_cyl: u8,
    start_sect: [u8; 4],
    nr_sects: [u8; 4],
}

impl Partition {
    fn new() -> Self {
        Partition {
            boot_ind: 0,
            head: 0,
            sector: 0,
            cyl: 0,
            sys_ind: 0,
            end_head: 0,
            end_sector: 0,
            end_cyl: 0,
            start_sect: [0; 4],
            nr_sects: [0; 4],
        }
    }
}

fn set_offset(h: &mut Hsc, offset: u32, heads: u16, sectors: u16) {
    let (head, sector, cyl) = if heads == 0 || sectors == 0 {
        (0, 0, 0) // linear mode
    } else {
        let sector = (offset % u32::from(sectors)) as u16;
        let offset = offset / u32::from(sectors);
        
        let head = (offset % u32::from(heads)) as u16;
        let offset = offset / u32::from(heads);
        let cyl = if offset > 1023 {
            1023
        } else {
            offset as u16
        };
        
        (head, sector, cyl)
    };
    
    if head > u16::from(u8::MAX) {
        // sector or head out of range => linear mode
        h.head = 0;
        h.sector = 0;
        h.cyl = 0;
    } else {
        h.head = head as u8;
        h.sector = ((sector + 1) & 0x3f) | ((cyl & 0x300) >> 2) as u8;
        h.cyl = (cyl & 0xff) as u8;
    }
}

fn set_dword(dst: &mut [u8; 4], val: u32) {
    dst.copy_from_slice(&val.to_le_bytes());
}

fn set_begin_end(
    part_table: &mut [Partition; 4],
    begin: u32,
    end: u32,
    iheads: u16,
    isectors: u16,
    activate: bool,
    mut ptype: u8,
    fat_bits: u32,
) {
    let heads = if iheads > u16::from(u8::MAX) {
        eprintln!("Too many heads for partition: {}", iheads);
        process::exit(1);
    } else {
        iheads as u8
    };
    
    let sectors = if isectors > u16::from(u8::MAX) {
        eprintln!("Too many sectors for partition: {}", isectors);
        process::exit(1);
    } else {
        isectors as u8
    };
    
    set_offset(
        &mut Hsc {
            head: part_table[0].head,
            sector: part_table[0].sector,
            cyl: part_table[0].cyl,
        },
        begin,
        iheads,
        isectors,
    );
    
    set_offset(
        &mut Hsc {
            head: part_table[0].end_head,
            sector: part_table[0].end_sector,
            cyl: part_table[0].end_cyl,
        },
        end - 1,
        iheads,
        isectors,
    );
    
    set_dword(&mut part_table[0].start_sect, begin);
    set_dword(&mut part_table[0].nr_sects, end - begin);
    
    part_table[0].boot_ind = if activate { 0x80 } else { 0 };
    
    if ptype == 0 {
        if fat_bits == 0 {
            let fat_bits = if end - begin < 4096 { 12 } else { 16 };
            ptype = match fat_bits {
                32 => 0x0C, // Win95 FAT32, LBA
                12 if end < 65536 => 0x01, // DOS FAT12, CHS
                16 if end < 65536 => 0x04, // DOS FAT16, CHS
                12 | 16 if end < u32::from(sectors) * u32::from(heads) * 1024 => 0x06, // DOS BIG FAT16 or FAT12, CHS
                _ => 0x0E, // Win95 BIG FAT16, LBA
            };
        }
    }
    
    part_table[0].sys_ind = ptype;
}

fn setsize(capacity: u32, cyls: &mut u32, hds: &mut u16, secs: &mut u16) -> i32 {
    let mut rv = 0;
    let mut cylinders = 1024;
    let mut sectors = 62;
    
    let mut temp = cylinders * sectors;
    let mut heads = capacity / temp;
    
    if capacity % temp != 0 {
        heads += 1;
        temp = cylinders * heads;
        sectors = capacity / temp;
        
        if capacity % temp != 0 {
            sectors += 1;
            temp = heads * sectors;
            cylinders = capacity / temp;
        }
    }
    
    if cylinders == 0 {
        rv = -1;
    }
    
    *cyls = cylinders;
    *secs = sectors as u16;
    *hds = heads as u16;
    rv
}

fn setsize0(capacity: u32, cyls: &mut u32, hds: &mut u16, secs: &mut u16) {
    if capacity < 1024 * 2048 && capacity % 1024 == 0 {
        *cyls = capacity >> 11;
        *hds = 64;
        *secs = 32;
        return;
    }
    
    let r = setsize(capacity, cyls, hds, secs);
    if r != 0 || *hds > 255 || *secs > 63 {
        *cyls = capacity >> 11;
        *hds = 64;
        *secs = 32;
    }
}

fn usage(ret: i32) -> ! {
    eprintln!("Mtools version {}, dated {}", mversion, mdate);
    eprintln!(
        "Usage: {} [-pradcv] [-I] [-B bootsect-template] [-s sectors] [-t cylinders] \
        [-h heads] [-T type] [-b begin] [-l length] drive",
        progname
    );
    process::exit(ret);
}

fn main() {
    // TODO: Implement the rest of the mpartition functionality
    // This includes:
    // - Command line argument parsing
    // - File I/O operations
    // - Partition table manipulation
    // - Error handling
    
    // Placeholder for now
    usage(1);
}
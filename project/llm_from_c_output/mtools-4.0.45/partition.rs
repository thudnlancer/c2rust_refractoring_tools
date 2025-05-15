/*  Copyright 1997,1998,2001-2003,2006,2009 Alain Knaff.
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

use std::io::{self, Write};
use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Hsc {
    pub byte0: u8,
    pub head: u8,
    pub sector: u8,
    pub cyl: u8,
}

impl Hsc {
    pub fn head(&self) -> u8 {
        self.head
    }

    pub fn sector(&self) -> u8 {
        self.sector & 0x3f
    }

    pub fn cyl(&self) -> u16 {
        self.cyl as u16 | ((self.sector & 0xc0) as u16) << 2
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Partition {
    pub start: Hsc,
    pub end: Hsc,
    pub start_sect: [u8; 4],
    pub nr_sects: [u8; 4],
}

impl Partition {
    pub fn boot_ind(&self) -> u8 {
        self.start.byte0
    }

    pub fn sys_ind(&self) -> u8 {
        self.end.byte0
    }

    pub fn begin(&self) -> u32 {
        let bytes = self.start_sect;
        u32::from_le_bytes(bytes)
    }

    pub fn end(&self) -> u32 {
        self.begin() + self.part_size()
    }

    pub fn part_size(&self) -> u32 {
        let bytes = self.nr_sects;
        u32::from_le_bytes(bytes)
    }
}

pub struct Device {
    pub partition: u8,
    pub tot_sectors: u32,
}

pub struct Stream {
    // Placeholder for Stream implementation
}

pub type MtOff = i64;

pub fn consistency_check(
    part_table: &[Partition; 4],
    do_print: bool,
    verbose: bool,
    has_activated: &mut i32,
    tot_sectors: u32,
    _used_dev: &Device,
    target_partition: u8,
) -> bool {
    let mut inconsistency = false;
    *has_activated = 0;

    for i in 0..4 {
        let partition = &part_table[i];
        if partition.sys_ind() == 0 {
            continue;
        }

        if partition.boot_ind() != 0 {
            *has_activated += 1;
        }

        if partition.end() < partition.begin() {
            eprintln!("End of partition {} before its begin", i + 1);
        }

        if let Some(j) = find_overlap(part_table, i, partition.begin(), partition.end()) {
            eprintln!("Partitions {} and {} overlap", j + 1, i + 1);
            inconsistency = true;
        }

        if tot_sectors != 0 && partition.end() > tot_sectors {
            eprintln!("Partition {} extends beyond end of disk", i + 1);
        }

        if do_print && verbose {
            if (i + 1) as u8 == target_partition {
                print!("*");
            } else {
                print!(" ");
            }
            println!("Partition {}", i + 1);
            println!("  active={:x}", partition.boot_ind());
            println!("  start: h={} s={} c={}", 
                    partition.start.head(), 
                    partition.start.sector(), 
                    partition.start.cyl());
            println!("  type=0x{:x}", partition.sys_ind());
            println!("  end: h={} s={} c={}", 
                    partition.end.head(), 
                    partition.end.sector(), 
                    partition.end.cyl());
            println!("  start={}", partition.begin());
            println!("  nr={}", partition.part_size());
            println!();
        }
    }

    inconsistency
}

pub fn find_overlap(
    part_table: &[Partition; 4],
    until: usize,
    start: u32,
    end: u32,
) -> Option<usize> {
    for i in 0..until {
        if overlap_check(&part_table[i], start, end) {
            return Some(i);
        }
    }
    None
}

fn overlap_check(partition: &Partition, start: u32, end: u32) -> bool {
    if partition.sys_ind() == 0 {
        return false;
    }
    end > partition.begin() && (start < partition.end() || partition.end() < partition.begin())
}

pub struct PartitionStream {
    pub offset: MtOff,
    pub size: MtOff,
    pub nb_sect: u32,
    pub pos: u8,
    pub sectors: u8,
    pub heads: u8,
    pub cylinders: u16,
    pub next: Box<Stream>,
}

impl PartitionStream {
    fn limit_size(&self, start: MtOff, len: &mut usize) -> io::Result<()> {
        if start > self.size {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Start beyond partition size"));
        }
        let remaining = (self.size - start) as usize;
        if *len > remaining {
            *len = remaining;
        }
        Ok(())
    }

    fn pread(&self, buf: &mut [u8], start: MtOff) -> io::Result<usize> {
        let mut len = buf.len();
        self.limit_size(start, &mut len)?;
        // Implement actual pread using self.next
        Ok(len)
    }

    fn pwrite(&mut self, buf: &[u8], start: MtOff) -> io::Result<usize> {
        let mut len = buf.len();
        self.limit_size(start, &mut len)?;
        // Implement actual pwrite using self.next
        Ok(len)
    }
}

pub fn open_partition(
    next: Stream,
    dev: &Device,
    errmsg: &mut String,
    max_size: &mut MtOff,
) -> Option<PartitionStream> {
    if dev.partition == 0 || dev.partition > 4 {
        eprintln!(
            "Invalid partition {} (must be between 1 and 4), ignoring it",
            dev.partition
        );
        return None;
    }

    let mut buf = [0u8; 2048];
    // Read first sector
    // if force_pread(next, &mut buf, 0, 512) != 512 {
    //     return None;
    // }

    if u16::from_le_bytes([buf[510], buf[511]]) != 0xaa55 {
        *errmsg = "Device does not have a BIOS partition table".to_string();
        return None;
    }

    let part_table_ptr = &buf[0x1ae] as *const u8 as *const Partition;
    let part_table = unsafe { &*part_table_ptr };
    let partition = &part_table[(dev.partition - 1) as usize];

    if partition.sys_ind() == 0 {
        *errmsg = format!("Partition {} does not exist", dev.partition);
        return None;
    }

    let part_off = partition.begin() as MtOff;
    if let Some(max) = max_size {
        if part_off > (*max >> 9) {
            *errmsg = "init: Big disks not supported".to_string();
            return None;
        }
        *max -= part_off << 9;
        *max = (*max).min((partition.part_size() as MtOff) << 9);
    }

    let mut has_activated = 0;
    if !false /* mtools_skip_check */ && consistency_check(
        part_table,
        false,
        false,
        &mut has_activated,
        dev.tot_sectors,
        dev,
        0,
    ) {
        eprintln!("Warning: inconsistent partition table");
        eprintln!("Possibly unpartitioned device");
        eprintln!("\n*** Maybe try without partition={} in device definition ***\n", dev.partition);
        eprintln!("If this is a PCMCIA card, or a disk partitioned on another computer, this message may be in error: add mtools_skip_check=1 to your .mtoolsrc file to suppress this warning");
    }

    Some(PartitionStream {
        offset: part_off << 9,
        size: (partition.part_size() as MtOff) << 9,
        nb_sect: partition.part_size(),
        pos: 0,
        sectors: 0,
        heads: 0,
        cylinders: 0,
        next: Box::new(next),
    })
}
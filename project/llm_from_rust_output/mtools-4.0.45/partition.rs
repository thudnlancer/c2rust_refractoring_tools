use std::ffi::{c_void, c_char, c_int, c_uchar, c_ushort, c_uint, c_long, c_ulong};
use std::ptr;
use std::mem;
use std::io::{self, Write};
use std::time::SystemTime;

type uint8_t = c_uchar;
type uint16_t = c_ushort;
type uint32_t = c_uint;
type size_t = c_ulong;
type off_t = c_long;
type ssize_t = c_long;
type time_t = c_long;
type mt_off_t = off_t;

struct DosCp;
struct Class;
struct Stream {
    class: *mut Class,
    refs: c_int,
    next: *mut Stream,
}

struct Device {
    name: *const c_char,
    drive: c_char,
    fat_bits: c_int,
    mode: c_int,
    tracks: c_uint,
    heads: uint16_t,
    sectors: uint16_t,
    hidden: c_uint,
    offset: mt_off_t,
    partition: c_uint,
    misc_flags: c_uint,
    ssize: uint8_t,
    use_2m: c_uint,
    precmd: *mut c_char,
    file_nr: c_int,
    blocksize: c_uint,
    codepage: c_uint,
    data_map: *const c_char,
    tot_sectors: uint32_t,
    sector_size: uint16_t,
    postcmd: *mut c_char,
    cfg_filename: *const c_char,
}

struct Hsc {
    byte0: c_uchar,
    head: c_uchar,
    sector: c_uchar,
    cyl: c_uchar,
}

struct Partition {
    start: Hsc,
    end: Hsc,
    start_sect: [c_uchar; 4],
    nr_sects: [c_uchar; 4],
}

struct PartitionStream {
    head: Stream,
    offset: mt_off_t,
    size: mt_off_t,
    nb_sect: uint32_t,
    pos: uint8_t,
    sectors: uint8_t,
    heads: uint8_t,
    cylinders: uint16_t,
}

impl PartitionStream {
    fn limit_size(&self, start: mt_off_t, len: &mut size_t) -> c_int {
        if start > self.size {
            return -1;
        }
        // Assuming limitSizeToOffT is implemented elsewhere
        unsafe { limitSizeToOffT(len, self.size - start) };
        0
    }

    fn pread(&self, buf: &mut [u8], start: mt_off_t) -> ssize_t {
        let mut len = buf.len() as size_t;
        if self.limit_size(start, &mut len) < 0 {
            return -1;
        }
        // Assuming force_pread is implemented elsewhere
        unsafe { force_pread(self.head.next, buf.as_mut_ptr() as *mut c_char, start + self.offset, len) }
    }

    fn pwrite(&mut self, buf: &[u8], start: mt_off_t) -> ssize_t {
        let mut len = buf.len() as size_t;
        if self.limit_size(start, &mut len) < 0 {
            return -1;
        }
        // Assuming force_pwrite is implemented elsewhere
        unsafe { force_pwrite(self.head.next, buf.as_ptr() as *mut c_char, start + self.offset, len) }
    }

    fn get_data(&self, date: Option<&mut time_t>, size: Option<&mut mt_off_t>, type_: Option<&mut c_int>, address: Option<&mut uint32_t>) -> c_int {
        if date.is_some() || type_.is_some() || address.is_some() {
            // Assuming get_data is implemented elsewhere
            let ret = unsafe { get_data(self.head.next, date.map(|x| x as *mut _).unwrap_or(ptr::null_mut()), ptr::null_mut(), type_.map(|x| x as *mut _).unwrap_or(ptr::null_mut()), address.map(|x| x as *mut _).unwrap_or(ptr::null_mut())) };
            if ret < 0 {
                return ret;
            }
        }
        if let Some(size) = size {
            *size = self.size * 512;
        }
        0
    }

    fn set_geom(&mut self, dev: &mut Device, orig_dev: &mut Device) -> c_int {
        if dev.tot_sectors == 0 {
            dev.tot_sectors = self.nb_sect;
        }
        0
    }
}

fn print_hsc(h: &Hsc) {
    println!(
        " h={} s={} c={}",
        h.head as c_int,
        (h.sector as c_int & 0x3f) as uint8_t as c_int,
        (h.cyl as c_int | ((h.sector as c_int & 0xc0) << 2)) as uint16_t as c_int
    );
}

fn overlap_check(part_table: &[Partition], i: usize, start: uint32_t, end: uint32_t) -> bool {
    if part_table[i].end.byte0 == 0 {
        return false;
    }

    let part_start = u32::from_le_bytes(part_table[i].start_sect);
    let part_end = part_start + u32::from_le_bytes(part_table[i].nr_sects);

    end > part_start && (start < part_end || part_end < part_start)
}

fn find_overlap(part_table: &[Partition], until: usize, start: uint32_t, end: uint32_t) -> Option<usize> {
    (1..=until).find(|&i| overlap_check(part_table, i, start, end))
}

fn consistency_check(
    part_table: &[Partition],
    doprint: bool,
    verbose: bool,
    has_activated: &mut c_int,
    tot_sectors: uint32_t,
    used_dev: &mut Device,
    target_partition: usize,
) -> bool {
    let mut inconsistency = false;
    *has_activated = 0;

    for i in 1..=4 {
        if part_table[i].end.byte0 != 0 {
            if part_table[i].start.byte0 != 0 {
                *has_activated += 1;
            }

            let part_start = u32::from_le_bytes(part_table[i].start_sect);
            let part_end = part_start + u32::from_le_bytes(part_table[i].nr_sects);

            if part_end < part_start {
                eprintln!("End of partition {} before its begin", i);
            }

            if let Some(j) = find_overlap(part_table, i - 1, part_start, part_end) {
                eprintln!("Partitions {} and {} overlap", j, i);
                inconsistency = true;
            }

            if tot_sectors != 0 && part_end > tot_sectors {
                eprintln!("Partition {} extends beyond end of disk", i);
            }

            if doprint && verbose {
                if i == target_partition {
                    print!("*");
                } else {
                    print!(" ");
                }
                println!("Partition {}", i);
                println!("  active={:x}", part_table[i].start.byte0 as c_int);
                print!("  start:");
                print_hsc(&part_table[i].start);
                println!("  type=0x{:x}", part_table[i].end.byte0 as c_int);
                print!("  end:");
                print_hsc(&part_table[i].end);
                println!("  start={}", part_start);
                println!("  nr={}", u32::from_le_bytes(part_table[i].nr_sects));
                println!();
            }
        }
    }

    inconsistency
}

fn open_partition(
    next: *mut Stream,
    dev: &mut Device,
    errmsg: &mut [c_char],
    max_size: Option<&mut mt_off_t>,
) -> Option<*mut Stream> {
    if dev.partition > 4 || dev.partition == 0 {
        eprintln!(
            "Invalid partition {} (must be between 1 and 4), ignoring it",
            dev.partition
        );
        return None;
    }

    let mut this = Box::new(PartitionStream {
        head: Stream {
            class: &PARTITION_CLASS as *const _ as *mut _,
            refs: 1,
            next,
        },
        offset: 0,
        size: 0,
        nb_sect: 0,
        pos: 0,
        sectors: 0,
        heads: 0,
        cylinders: 0,
    });

    let mut buf = [0u8; 2048];
    if unsafe { force_pread(next, buf.as_mut_ptr() as *mut c_char, 0, 512) } != 512 {
        return None;
    }

    if u16::from_le_bytes([buf[510], buf[511]]) != 0xaa55 {
        if !errmsg.is_empty() {
            unsafe {
                sprintf(
                    errmsg.as_mut_ptr(),
                    b"Device does not have a BIOS partition table\n\0".as_ptr() as *const c_char,
                );
            }
        }
        return None;
    }

    let part_table = unsafe {
        &*(buf[0x1ae..].as_ptr() as *const [Partition; 4])
    };

    if part_table[dev.partition as usize].end.byte0 == 0 {
        if !errmsg.is_empty() {
            unsafe {
                sprintf(
                    errmsg.as_mut_ptr(),
                    b"Partition {} does not exist\n\0".as_ptr() as *const c_char,
                    dev.partition,
                );
            }
        }
        return None;
    }

    let part_off = u32::from_le_bytes(part_table[dev.partition as usize].start_sect);
    if let Some(max_size) = max_size {
        if (part_off as mt_off_t) > (*max_size >> 9) {
            if !errmsg.is_empty() {
                unsafe {
                    sprintf(
                        errmsg.as_mut_ptr(),
                        b"init: Big disks not supported\0".as_ptr() as *const c_char,
                    );
                }
            }
            return None;
        }
        *max_size -= (part_off << 9) as mt_off_t;
        let part_size = u32::from_le_bytes(part_table[dev.partition as usize].nr_sects) as mt_off_t << 9;
        if *max_size > part_size {
            *max_size = part_size;
        }
    }

    this.offset = (part_off as mt_off_t) << 9;
    if unsafe { mtools_skip_check == 0 } {
        let mut has_activated = 0;
        if consistency_check(
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
    }

    this.nb_sect = u32::from_le_bytes(part_table[dev.partition as usize].nr_sects);
    dev.tot_sectors = this.nb_sect;
    this.size = (this.nb_sect as mt_off_t) << 9;

    Some(Box::into_raw(this) as *mut _)
}

static PARTITION_CLASS: Class = Class {
    read: None,
    write: None,
    pread: Some(partition_pread),
    pwrite: Some(partition_pwrite),
    flush: None,
    free_func: None,
    set_geom: Some(partition_set_geom),
    get_data: Some(partition_get_data),
    pre_allocate: None,
    get_dos_convert: Some(get_dos_convert_pass_through),
    discard: None,
};

// Placeholder for external functions
extern "C" {
    fn limitSizeToOffT(len: *mut size_t, max_len: mt_off_t);
    fn force_pread(stream: *mut Stream, buf: *mut c_char, start: mt_off_t, len: size_t) -> ssize_t;
    fn force_pwrite(stream: *mut Stream, buf: *mut c_char, start: mt_off_t, len: size_t) -> ssize_t;
    fn get_data(stream: *mut Stream, date: *mut time_t, size: *mut mt_off_t, type_: *mut c_int, address: *mut uint32_t) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    static mut mtools_skip_check: c_uint;
    fn get_dos_convert_pass_through(stream: *mut Stream) -> *mut DosCp;
}

// Wrapper functions for FFI
unsafe fn partition_pread(stream: *mut Stream, buf: *mut c_char, start: mt_off_t, len: size_t) -> ssize_t {
    let this = &*(stream as *const PartitionStream);
    this.pread(std::slice::from_raw_parts_mut(buf as *mut u8, len as usize), start)
}

unsafe fn partition_pwrite(stream: *mut Stream, buf: *mut c_char, start: mt_off_t, len: size_t) -> ssize_t {
    let this = &mut *(stream as *mut PartitionStream);
    this.pwrite(std::slice::from_raw_parts(buf as *const u8, len as usize), start)
}

unsafe fn partition_set_geom(stream: *mut Stream, dev: *mut Device, orig_dev: *mut Device) -> c_int {
    let this = &mut *(stream as *mut PartitionStream);
    this.set_geom(&mut *dev, &mut *orig_dev)
}

unsafe fn partition_get_data(stream: *mut Stream, date: *mut time_t, size: *mut mt_off_t, type_: *mut c_int, address: *mut uint32_t) -> c_int {
    let this = &*(stream as *const PartitionStream);
    this.get_data(
        if date.is_null() { None } else { Some(&mut *date) },
        if size.is_null() { None } else { Some(&mut *size) },
        if type_.is_null() { None } else { Some(&mut *type_) },
        if address.is_null() { None } else { Some(&mut *address) },
    )
}
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};
use std::ptr;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapType {
    Data = 0,
    Zero = 1,
    Skip = 2,
    Pos = 3,
}

#[derive(Debug)]
struct Map {
    orig: i64,
    remapped: i64,
    type_: MapType,
}

#[derive(Debug)]
struct Remap {
    head: Stream,
    map: Vec<Map>,
    net_offset: i64,
}

#[derive(Debug)]
struct Stream {
    class: &'static Class,
    refs: i32,
    next: Box<Stream>,
}

#[derive(Debug)]
struct Class {
    pread: fn(&Stream, &mut [u8], i64) -> Result<usize, i32>,
    pwrite: fn(&Stream, &[u8], i64) -> Result<usize, i32>,
    free_func: fn(&mut Stream) -> i32,
    set_geom: fn(&mut Stream, &mut Device, &mut Device) -> i32,
    get_dos_convert: fn(&Stream) -> *mut DosCp,
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
    data_map: String,
    tot_sectors: u32,
    sector_size: u16,
    postcmd: Option<String>,
    cfg_filename: Option<String>,
}

type DosCp = c_void;

impl Remap {
    fn remap(&self, start: &mut i64, len: &mut usize) -> MapType {
        for i in 0..self.map.len() - 1 {
            if *start < self.map[i + 1].remapped {
                *len = (*len).min((self.map[i + 1].remapped - *start) as usize);
                break;
            }
        }
        let map = &self.map[0];
        *start = *start - map.remapped + map.orig;
        map.type_
    }

    fn pread(&self, buf: &mut [u8], start: i64, len: usize) -> Result<usize, i32> {
        let mut start = start;
        let mut len = len;
        if self.remap(&mut start, &mut len) == MapType::Data {
            (self.head.class.pread)(&self.head.next, &mut buf[..len], start)
        } else {
            buf[..len].fill(0);
            Ok(len)
        }
    }

    fn pwrite(&self, buf: &[u8], start: i64, len: usize) -> Result<usize, i32> {
        let mut start = start;
        let mut len = len;
        if self.remap(&mut start, &mut len) == MapType::Data {
            (self.head.class.pwrite)(&self.head.next, &buf[..len], start)
        } else {
            if buf[..len].iter().any(|&b| b != 0) {
                eprintln!("Bad data written to unmapped sectors");
                Err(14)
            } else {
                Ok(len)
            }
        }
    }

    fn free(&mut self) -> i32 {
        0
    }
}

static REMAP_CLASS: Class = Class {
    pread: |stream, buf, start| {
        let remap = unsafe { &*(stream as *const Stream as *const Remap) };
        remap.pread(buf, start, buf.len())
    },
    pwrite: |stream, buf, start| {
        let remap = unsafe { &*(stream as *const Stream as *const Remap) };
        remap.pwrite(buf, start, buf.len())
    },
    free_func: |stream| {
        let remap = unsafe { &mut *(stream as *mut Stream as *mut Remap) };
        remap.free()
    },
    set_geom: |_, _, _| 0,
    get_dos_convert: |_| ptr::null_mut(),
};

fn process_map(
    remap: &mut Remap,
    map_str: &str,
    count_only: bool,
    errmsg: &mut [u8],
) -> Result<usize, i32> {
    let mut orig = 0i64;
    let mut remapped = 0i64;
    let mut count = 0;
    let mut at_end = false;
    let mut ptr = map_str;

    while !at_end {
        let (type_, rest) = if ptr.is_empty() {
            (MapType::Data, ptr)
        } else if ptr.starts_with("skip") {
            (MapType::Skip, &ptr[4..])
        } else if ptr.starts_with("zero") {
            (MapType::Zero, &ptr[4..])
        } else if ptr.starts_with("pos") {
            (MapType::Pos, &ptr[3..])
        } else {
            (MapType::Data, ptr)
        };

        let (len_str, next) = match rest.find(',') {
            Some(pos) => (&rest[..pos], &rest[pos + 1..]),
            None => (rest, ""),
        };

        let len = len_str.parse::<i64>().map_err(|_| {
            let msg = format!("Bad number {}\n", rest);
            errmsg[..msg.len().min(errmsg.len())].copy_from_slice(msg.as_bytes());
            -1
        })?;

        ptr = next;
        at_end = next.is_empty();

        match type_ {
            MapType::Pos => orig = len,
            _ => {
                if type_ != MapType::Skip {
                    if !count_only {
                        remap.map.push(Map {
                            orig,
                            remapped,
                            type_,
                        });
                    }
                    remapped += len;
                    count += 1;
                }
                if type_ != MapType::Zero {
                    orig += len;
                }
            }
        }
    }

    remap.net_offset = orig - remapped;
    Ok(count)
}

fn remap(
    next: Box<Stream>,
    dev: &mut Device,
    errmsg: &mut [u8],
) -> Result<Box<Stream>, i32> {
    let mut remap = Remap {
        head: Stream {
            class: &REMAP_CLASS,
            refs: 1,
            next,
        },
        map: Vec::new(),
        net_offset: 0,
    };

    let nr_items = process_map(&mut remap, &dev.data_map, true, errmsg)?;
    remap.map.reserve(nr_items);
    process_map(&mut remap, &dev.data_map, false, errmsg)?;

    if adjust_tot_sectors(dev, remap.net_offset, errmsg) < 0 {
        return Err(-1);
    }

    Ok(Box::new(remap))
}

fn adjust_tot_sectors(dev: &mut Device, offset: i64, errmsg: &mut [u8]) -> i32 {
    // Implementation omitted for brevity
    0
}
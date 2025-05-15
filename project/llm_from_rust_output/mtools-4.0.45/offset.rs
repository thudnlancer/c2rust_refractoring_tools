use std::ffi::{CString, CStr};
use std::ptr;
use std::mem;
use std::os::raw::{c_char, c_void, c_int, c_uint, c_uchar, c_ushort, c_ulong, c_long};

type mt_off_t = i64;
type size_t = usize;
type ssize_t = isize;

struct DosCp;
struct Stream {
    class: &'static Class,
    refs: i32,
    next: Option<Box<Stream>>,
}

struct Class {
    read: Option<fn(&mut Stream, &mut [u8]) -> isize>,
    write: Option<fn(&mut Stream, &[u8]) -> isize>,
    pread: Option<fn(&mut Stream, &mut [u8], mt_off_t) -> isize>,
    pwrite: Option<fn(&mut Stream, &[u8], mt_off_t) -> isize>,
    flush: Option<fn(&mut Stream) -> i32>,
    free_func: Option<fn(&mut Stream) -> i32>,
    set_geom: Option<fn(&mut Stream, &mut Device, &mut Device) -> i32>,
    get_data: Option<fn(&mut Stream, &mut i64, &mut mt_off_t, &mut i32, &mut u32) -> i32>,
    pre_allocate: Option<fn(&mut Stream, mt_off_t) -> i32>,
    get_dos_convert: Option<fn(&mut Stream) -> &mut DosCp>,
    discard: Option<fn(&mut Stream) -> i32>,
}

struct Device {
    name: Option<CString>,
    drive: char,
    fat_bits: i32,
    mode: i32,
    tracks: u32,
    heads: u16,
    sectors: u16,
    hidden: u32,
    offset: mt_off_t,
    partition: u32,
    misc_flags: u32,
    ssize: u8,
    use_2m: u32,
    precmd: Option<CString>,
    file_nr: i32,
    blocksize: u32,
    codepage: u32,
    data_map: Option<CString>,
    tot_sectors: u32,
    sector_size: u16,
    postcmd: Option<CString>,
    cfg_filename: Option<CString>,
}

struct Offset {
    head: Stream,
    offset: mt_off_t,
}

fn offset_pread(stream: &mut Stream, buf: &mut [u8], start: mt_off_t) -> isize {
    let this = unsafe { &mut *(stream as *mut _ as *mut Offset) };
    if let Some(ref mut next) = this.head.next {
        if let Some(pread) = next.class.pread {
            return pread(next, buf, start + this.offset);
        }
    }
    -1
}

fn offset_pwrite(stream: &mut Stream, buf: &[u8], start: mt_off_t) -> isize {
    let this = unsafe { &mut *(stream as *mut _ as *mut Offset) };
    if let Some(ref mut next) = this.head.next {
        if let Some(pwrite) = next.class.pwrite {
            return pwrite(next, buf, start + this.offset);
        }
    }
    -1
}

static OFFSET_CLASS: Class = Class {
    read: None,
    write: None,
    pread: Some(offset_pread),
    pwrite: Some(offset_pwrite),
    flush: None,
    free_func: None,
    set_geom: Some(set_geom_pass_through),
    get_data: None,
    pre_allocate: None,
    get_dos_convert: Some(get_dos_convert_pass_through),
    discard: None,
};

fn set_geom_pass_through(_stream: &mut Stream, _dev: &mut Device, _orig_dev: &mut Device) -> i32 {
    0
}

fn get_dos_convert_pass_through(_stream: &mut Stream) -> &mut DosCp {
    unsafe { &mut *(ptr::null_mut() as *mut DosCp) }
}

fn print_oom() {
    eprintln!("Out of memory");
}

fn open_offset(
    next: Option<Box<Stream>>,
    dev: &mut Device,
    offset: mt_off_t,
    errmsg: Option<&mut CString>,
    max_size: Option<&mut mt_off_t>,
) -> Option<Box<Stream>> {
    let mut this = Box::new(Offset {
        head: Stream {
            class: &OFFSET_CLASS,
            refs: 1,
            next,
        },
        offset,
    });

    if let Some(max_size) = max_size {
        if this.offset > *max_size {
            if let Some(errmsg) = errmsg {
                *errmsg = CString::new("init: Big disks not supported").unwrap();
            }
            return None;
        } else {
            *max_size -= this.offset;
        }
    }

    if adjust_tot_sectors(dev, this.offset, errmsg) < 0 {
        return None;
    }

    Some(Box::new(Stream {
        class: &OFFSET_CLASS,
        refs: 1,
        next: Some(this),
    }))
}

fn adjust_tot_sectors(_dev: &mut Device, _offset: mt_off_t, _errmsg: Option<&mut CString>) -> i32 {
    0
}
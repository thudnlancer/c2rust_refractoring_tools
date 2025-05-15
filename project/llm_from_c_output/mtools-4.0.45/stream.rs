// Copyright 1996-1999,2001,2002,2005,2006,2008,2009,2011 Alain Knaff.
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

use std::sync::Arc;
use std::time::SystemTime;
use std::io::{self, Read, Write, Seek, SeekFrom};

type MtOff = i64;
type DosCp = (); // Placeholder for doscp_t
type Device = (); // Placeholder for device_t

struct Stream {
    class: Arc<Class>,
    refs: std::sync::atomic::AtomicUsize,
    next: Option<Arc<Stream>>,
}

impl Stream {
    fn new(class: Arc<Class>, next: Option<Arc<Stream>>) -> Self {
        Stream {
            class,
            refs: std::sync::atomic::AtomicUsize::new(1),
            next,
        }
    }

    fn copy(&self) -> Arc<Self> {
        self.refs.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Arc::new(Self {
            class: self.class.clone(),
            refs: self.refs.clone(),
            next: self.next.clone(),
        })
    }
}

struct Class {
    read: fn(&Stream, &mut [u8]) -> io::Result<usize>,
    write: fn(&Stream, &[u8]) -> io::Result<usize>,
    pread: fn(&Stream, &mut [u8], MtOff) -> io::Result<usize>,
    pwrite: fn(&Stream, &[u8], MtOff) -> io::Result<usize>,
    flush: fn(&Stream) -> io::Result<()>,
    free_func: fn(&Stream) -> io::Result<()>,
    set_geom: fn(&Stream, &Device, &Device) -> io::Result<()>,
    get_data: fn(&Stream, &mut SystemTime, &mut MtOff, &mut i32, &mut u32) -> io::Result<()>,
    pre_allocate: fn(&Stream, MtOff) -> io::Result<()>,
    get_dos_convert: fn(&Stream) -> io::Result<DosCp>,
    discard: fn(&Stream) -> io::Result<()>,
}

fn limit_size_to_off_t(len: &mut usize, max_len: MtOff) {
    if *len as MtOff > max_len {
        *len = max_len as usize;
    }
}

fn init_head(stream: &mut Stream, class: Arc<Class>, next: Option<Arc<Stream>>) {
    stream.class = class;
    stream.refs.store(1, std::sync::atomic::Ordering::Relaxed);
    stream.next = next;
}

fn flush_stream(stream: &Stream) -> io::Result<()> {
    if !BATCHMODE.load(std::sync::atomic::Ordering::Relaxed) {
        if let Some(flush) = stream.class.flush {
            flush(stream)?;
        }
        if let Some(next) = &stream.next {
            flush_stream(next)?;
        }
    }
    Ok(())
}

fn free_stream(stream: Arc<Stream>) -> io::Result<()> {
    let refs = stream.refs.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    if refs == 1 {
        if let Some(flush) = stream.class.flush {
            flush(&stream)?;
        }
        if let Some(free_func) = stream.class.free_func {
            free_func(&stream)?;
        }
        if let Some(next) = &stream.next {
            free_stream(next.clone())?;
        }
    }
    Ok(())
}

static BATCHMODE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn set_geom_pass_through(stream: &Stream, dev: &Device, orig_dev: &Device) -> io::Result<()> {
    if let Some(next) = &stream.next {
        if let Some(set_geom) = next.class.set_geom {
            set_geom(next, dev, orig_dev)?;
        }
    }
    Ok(())
}

fn set_geom_noop(_stream: &Stream, _dev: &Device, _orig_dev: &Device) -> io::Result<()> {
    Ok(())
}

fn get_data_pass_through(
    stream: &Stream,
    date: &mut SystemTime,
    size: &mut MtOff,
    type_: &mut i32,
    address: &mut u32,
) -> io::Result<()> {
    if let Some(next) = &stream.next {
        if let Some(get_data) = next.class.get_data {
            get_data(next, date, size, type_, address)?;
        }
    }
    Ok(())
}

fn pread_pass_through(stream: &Stream, buf: &mut [u8], start: MtOff) -> io::Result<usize> {
    if let Some(next) = &stream.next {
        if let Some(pread) = next.class.pread {
            return pread(next, buf, start);
        }
    }
    Ok(0)
}

fn pwrite_pass_through(stream: &Stream, buf: &[u8], start: MtOff) -> io::Result<usize> {
    if let Some(next) = &stream.next {
        if let Some(pwrite) = next.class.pwrite {
            return pwrite(next, buf, start);
        }
    }
    Ok(0)
}

fn get_dos_convert_pass_through(stream: &Stream) -> io::Result<DosCp> {
    if let Some(next) = &stream.next {
        if let Some(get_dos_convert) = next.class.get_dos_convert {
            return get_dos_convert(next);
        }
    }
    Ok(())
}

fn adjust_tot_sectors(dev: &mut Device, offset: MtOff, errmsg: &mut String) -> io::Result<()> {
    if dev.tot_sectors == 0 {
        return Ok(());
    }

    let offs_sectors = offset / (if dev.sector_size != 0 { dev.sector_size } else { 512 });
    if offs_sectors > 0 && (dev.tot_sectors as MtOff) < offs_sectors {
        if !errmsg.is_empty() {
            *errmsg = "init: Offset bigger than base image".to_string();
        }
        return Err(io::Error::new(io::ErrorKind::InvalidInput, errmsg));
    }
    dev.tot_sectors -= offs_sectors as u32;
    Ok(())
}

fn open_root_dir(drivename: char, flags: i32, is_rop: &mut bool) -> io::Result<Option<Arc<Stream>>> {
    // Implementation placeholder
    Ok(None)
}
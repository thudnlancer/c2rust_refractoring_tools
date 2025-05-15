/* zio.rs (simulation of non-standard low-level i/o functions) */

/* Written by Andrew Makhorin <mao@gnu.org>, April 2011
 * For conditions of distribution and use, see copyright notice in
 * zlib.h */

/* WARNING: this file should *not* be used by applications. It is
   part of the implementation of the compression library and is
   subject to change. Applications should only use zlib.h. */

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::sync::Once;

pub const O_RDONLY: i32 = 0x00;
pub const O_WRONLY: i32 = 0x01;
pub const O_CREAT: i32 = 0x10;
pub const O_TRUNC: i32 = 0x20;
pub const O_APPEND: i32 = 0x30;

static INIT: Once = Once::new();
static mut FILES: Vec<Option<File>> = Vec::new();

fn initialize() {
    unsafe {
        FILES = Vec::with_capacity(std::os::raw::c_int::MAX as usize);
        FILES.push(Some(io::stdin()));
        FILES.push(Some(io::stdout()));
        FILES.push(Some(io::stderr()));
        
        for _ in 3..FILES.capacity() {
            FILES.push(None);
        }
    }
}

pub fn open(path: &str, oflag: i32) -> io::Result<i32> {
    INIT.call_once(initialize);
    
    let file = unsafe {
        match oflag {
            O_RDONLY => File::open(path),
            _ if oflag == (O_WRONLY | O_CREAT | O_TRUNC) => {
                OpenOptions::new().write(true).create(true).truncate(true).open(path)
            }
            _ if oflag == (O_WRONLY | O_CREAT | O_APPEND) => {
                OpenOptions::new().write(true).create(true).append(true).open(path)
            }
            _ => panic!("invalid open flag"),
        }
    }?;

    unsafe {
        for (fd, slot) in FILES.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(file);
                return Ok(fd as i32);
            }
        }
    }
    
    Err(io::Error::new(io::ErrorKind::Other, "no available file descriptors"))
}

pub fn read(fd: i32, buf: &mut [u8]) -> io::Result<usize> {
    INIT.call_once(initialize);
    
    unsafe {
        if fd < 0 || fd >= FILES.len() as i32 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid file descriptor"));
        }
        
        if let Some(file) = &mut FILES[fd as usize] {
            file.read(buf)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "file descriptor not open"))
        }
    }
}

pub fn write(fd: i32, buf: &[u8]) -> io::Result<usize> {
    INIT.call_once(initialize);
    
    unsafe {
        if fd < 0 || fd >= FILES.len() as i32 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid file descriptor"));
        }
        
        if let Some(file) = &mut FILES[fd as usize] {
            let count = file.write(buf)?;
            file.flush()?;
            Ok(count)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "file descriptor not open"))
        }
    }
}

pub fn lseek(fd: i32, offset: i64, whence: i32) -> io::Result<i64> {
    INIT.call_once(initialize);
    
    unsafe {
        if fd < 0 || fd >= FILES.len() as i32 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid file descriptor"));
        }
        
        if let Some(file) = &mut FILES[fd as usize] {
            let pos = match whence {
                0 => SeekFrom::Start(offset as u64),
                1 => SeekFrom::Current(offset),
                2 => SeekFrom::End(offset),
                _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid whence")),
            };
            file.seek(pos)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "file descriptor not open"))
        }
    }
}

pub fn close(fd: i32) -> io::Result<()> {
    INIT.call_once(initialize);
    
    unsafe {
        if fd < 0 || fd >= FILES.len() as i32 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid file descriptor"));
        }
        
        if FILES[fd as usize].take().is_some() {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "file descriptor not open"))
        }
    }
}

/* eof */
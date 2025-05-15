use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::sync::Once;

const FOPEN_MAX: usize = 16;

static INIT: Once = Once::new();
static mut FILES: [Option<File>; FOPEN_MAX] = [None; FOPEN_MAX];

fn initialize() {
    INIT.call_once(|| {
        unsafe {
            FILES[0] = Some(File::from_raw_fd(0));
            FILES[1] = Some(File::from_raw_fd(1));
            FILES[2] = Some(File::from_raw_fd(2));
        }
    });
}

pub fn _glp_zlib_open(path: &str, oflag: i32) -> io::Result<i32> {
    initialize();
    
    let file = match oflag {
        0 => File::open(path),
        0x1 | 0x10 | 0x20 => OpenOptions::new().write(true).create(true).truncate(true).open(path),
        0x1 | 0x10 | 0x30 => OpenOptions::new().write(true).create(true).append(true).open(path),
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid open flag")),
    }?;

    unsafe {
        for (fd, slot) in FILES.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(file);
                return Ok(fd as i32);
            }
        }
    }
    
    Err(io::Error::new(io::ErrorKind::Other, "No available file descriptor"))
}

pub fn _glp_zlib_read(fd: i32, buf: &mut [u8]) -> io::Result<usize> {
    initialize();
    
    if fd < 0 || fd >= FOPEN_MAX as i32 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid file descriptor"));
    }
    
    unsafe {
        if let Some(file) = FILES.get_mut(fd as usize).and_then(|f| f.as_mut()) {
            file.read(buf)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "File not open"))
        }
    }
}

pub fn _glp_zlib_write(fd: i32, buf: &[u8]) -> io::Result<usize> {
    initialize();
    
    if fd < 0 || fd >= FOPEN_MAX as i32 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid file descriptor"));
    }
    
    unsafe {
        if let Some(file) = FILES.get_mut(fd as usize).and_then(|f| f.as_mut()) {
            let count = file.write(buf)?;
            file.flush()?;
            Ok(count)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "File not open"))
        }
    }
}

pub fn _glp_zlib_lseek(fd: i32, offset: i64, whence: i32) -> io::Result<i64> {
    initialize();
    
    if fd < 0 || fd >= FOPEN_MAX as i32 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid file descriptor"));
    }
    
    let seek_from = match whence {
        0 => SeekFrom::Start(offset as u64),
        1 => SeekFrom::Current(offset),
        2 => SeekFrom::End(offset),
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid whence")),
    };
    
    unsafe {
        if let Some(file) = FILES.get_mut(fd as usize).and_then(|f| f.as_mut()) {
            file.seek(seek_from)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "File not open"))
        }
    }
}

pub fn _glp_zlib_close(fd: i32) -> io::Result<()> {
    initialize();
    
    if fd < 0 || fd >= FOPEN_MAX as i32 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid file descriptor"));
    }
    
    unsafe {
        if FILES[fd as usize].take().is_some() {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "File not open"))
        }
    }
}
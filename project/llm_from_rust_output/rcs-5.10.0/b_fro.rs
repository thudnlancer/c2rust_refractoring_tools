use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::path::Path;
use std::ptr;
use libc::{c_char, c_int, c_long, c_void, off_t, size_t, ssize_t};
use nix::sys::mman::{mmap, munmap, ProtFlags, MapFlags, madvise, Advice};
use nix::fcntl::{open, OFlag};
use nix::sys::stat::{fstat, FileStat};
use nix::unistd::{close, lseek, Whence};

#[derive(Debug)]
pub struct Fro {
    fd: Option<i32>,
    end: off_t,
    rm: ReadMethod,
    ptr: *mut c_char,
    lim: *mut c_char,
    base: *mut c_char,
    stream: Option<File>,
    verbatim: off_t,
}

#[derive(Debug, Clone, Copy)]
enum ReadMethod {
    Mmap,
    Mem,
    Stdio,
}

impl Fro {
    pub fn open(name: &Path, file_type: &str, status: Option<&mut FileStat>) -> io::Result<Self> {
        let fd = open(name, OFlag::O_RDONLY, nix::sys::stat::Mode::empty())?;
        let fd = fd_safer(fd);

        let mut st = FileStat::default();
        let status = status.unwrap_or(&mut st);
        fstat(fd, status)?;

        if !status.st_mode.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("`{}` is not a regular file", name.display()),
            ));
        }

        let s = status.st_size;
        let rm = if s == 0 {
            ReadMethod::Mem
        } else {
            ReadMethod::Mmap
        };

        let mut f = Fro {
            fd: Some(fd),
            end: s,
            rm,
            ptr: ptr::null_mut(),
            lim: ptr::null_mut(),
            base: ptr::null_mut(),
            stream: None,
            verbatim: 0,
        };

        match f.rm {
            ReadMethod::Mmap => {
                f.base = mmap(
                    ptr::null_mut(),
                    s as size_t,
                    ProtFlags::PROT_READ,
                    MapFlags::MAP_PRIVATE,
                    fd,
                    0,
                )? as *mut c_char;
                f.ptr = f.base;
                f.lim = unsafe { f.base.offset(s as isize) };
                madvise(f.base as *mut c_void, s as size_t, Advice::MADV_SEQUENTIAL)?;
            }
            ReadMethod::Mem => {
                if s == 0 {
                    f.base = ptr::null_mut();
                } else {
                    let mut buf = vec![0; s as usize];
                    let mut bytes_read = 0;
                    while bytes_read < s as usize {
                        let r = unsafe {
                            read(
                                fd,
                                buf[bytes_read..].as_mut_ptr() as *mut c_void,
                                (s as usize - bytes_read) as size_t,
                            )
                        };
                        if r < 0 {
                            return Err(io::Error::last_os_error());
                        }
                        bytes_read += r as usize;
                    }
                    f.base = buf.as_mut_ptr() as *mut c_char;
                    f.ptr = f.base;
                    f.lim = unsafe { f.base.offset(s as isize) };
                }
            }
            ReadMethod::Stdio => {
                let file = unsafe { File::from_raw_fd(fd) };
                f.stream = Some(file);
            }
        }

        Ok(f)
    }

    pub fn close(&mut self) -> io::Result<()> {
        match self.rm {
            ReadMethod::Mmap => {
                unsafe {
                    munmap(self.base as *mut c_void, (self.lim as usize - self.base as usize) as size_t)?;
                }
                if let Some(fd) = self.fd.take() {
                    close(fd)?;
                }
            }
            ReadMethod::Mem => {
                if let Some(fd) = self.fd.take() {
                    close(fd)?;
                }
            }
            ReadMethod::Stdio => {
                if let Some(mut file) = self.stream.take() {
                    file.flush()?;
                }
            }
        }
        Ok(())
    }

    pub fn tello(&self) -> io::Result<off_t> {
        match self.rm {
            ReadMethod::Mmap | ReadMethod::Mem => {
                Ok(unsafe { self.ptr.offset_from(self.base) } as off_t)
            }
            ReadMethod::Stdio => {
                if let Some(ref file) = self.stream {
                    Ok(file.seek(SeekFrom::Current(0))?)
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "No file stream"))
                }
            }
        }
    }

    pub fn move_cursor(&mut self, change: off_t) -> io::Result<()> {
        match self.rm {
            ReadMethod::Mmap | ReadMethod::Mem => {
                if change >= 0 {
                    self.ptr = unsafe { self.base.offset(change as isize) };
                }
                Ok(())
            }
            ReadMethod::Stdio => {
                if let Some(ref mut file) = self.stream {
                    file.seek(SeekFrom::Start(change as u64))?;
                    Ok(())
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "No file stream"))
                }
            }
        }
    }

    pub fn try_getbyte(&mut self) -> io::Result<Option<u8>> {
        match self.rm {
            ReadMethod::Mmap | ReadMethod::Mem => {
                if self.ptr == self.lim {
                    Ok(None)
                } else {
                    let byte = unsafe { *self.ptr };
                    self.ptr = unsafe { self.ptr.offset(1) };
                    Ok(Some(byte as u8))
                }
            }
            ReadMethod::Stdio => {
                if let Some(ref mut file) = self.stream {
                    let mut buf = [0u8; 1];
                    match file.read(&mut buf) {
                        Ok(0) => Ok(None),
                        Ok(_) => Ok(Some(buf[0])),
                        Err(e) => Err(e),
                    }
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "No file stream"))
                }
            }
        }
    }

    pub fn must_getbyte(&mut self) -> io::Result<u8> {
        match self.try_getbyte() {
            Ok(Some(byte)) => Ok(byte),
            Ok(None) => Err(io::Error::new(io::ErrorKind::UnexpectedEof, "unexpected end of file")),
            Err(e) => Err(e),
        }
    }

    pub fn spew_partial(&mut self, to: &mut File, r: &Range) -> io::Result<()> {
        match self.rm {
            ReadMethod::Mmap | ReadMethod::Mem => {
                let len = (r.end - r.beg) as usize;
                let buf = unsafe { std::slice::from_raw_parts(self.base.offset(r.beg as isize), len) };
                to.write_all(buf)?;
                if self.end == r.end {
                    self.ptr = self.lim;
                }
                Ok(())
            }
            ReadMethod::Stdio => {
                if let Some(ref mut file) = self.stream {
                    file.seek(SeekFrom::Start(r.beg as u64))?;
                    let mut buf = [0u8; 65536];
                    let mut pos = r.beg;
                    while pos < r.end {
                        let chunk_size = std::cmp::min(65536, (r.end - pos) as usize);
                        let bytes_read = file.read(&mut buf[..chunk_size])?;
                        if bytes_read == 0 {
                            break;
                        }
                        to.write_all(&buf[..bytes_read])?;
                        pos += bytes_read as off_t;
                    }
                    Ok(())
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "No file stream"))
                }
            }
        }
    }
}

fn fd_safer(fd: i32) -> i32 {
    // Implementation of fd_safer would go here
    fd
}

#[derive(Debug)]
struct Range {
    beg: off_t,
    end: off_t,
}
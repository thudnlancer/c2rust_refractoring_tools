use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::Path;
use std::slice;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use memmap2::Mmap;
use libc::{c_int, off_t};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReadMethod {
    Mmap,
    Mem,
    Stdio,
}

struct Range {
    beg: off_t,
    end: off_t,
}

struct Fro {
    fd: Option<RawFd>,
    end: off_t,
    rm: ReadMethod,
    ptr: *const u8,
    lim: *const u8,
    base: *const u8,
    deallocate: Option<fn(&mut Fro)>,
    stream: Option<File>,
    verbatim: off_t,
}

impl Fro {
    fn new() -> Self {
        Fro {
            fd: None,
            end: 0,
            rm: ReadMethod::Mem,
            ptr: std::ptr::null(),
            lim: std::ptr::null(),
            base: std::ptr::null(),
            deallocate: None,
            stream: None,
            verbatim: 0,
        }
    }

    fn open(name: &Path, ty: &str) -> io::Result<(Self, std::fs::Metadata)> {
        let mut f = Fro::new();
        let file = OpenOptions::new().read(true).open(name)?;
        let metadata = file.metadata()?;
        
        if !metadata.is_file() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Not a regular file"));
        }

        f.end = metadata.len() as off_t;
        f.fd = Some(file.as_raw_fd());

        // Determine read method
        f.rm = if metadata.len() < 1024 * 1024 { // Simplified condition
            ReadMethod::Mmap
        } else {
            ReadMethod::Stdio
        };

        match f.rm {
            ReadMethod::Mmap => {
                let mmap = unsafe { Mmap::map(&file)? };
                f.base = mmap.as_ptr();
                f.ptr = f.base;
                f.lim = unsafe { f.base.add(metadata.len()) };
                f.deallocate = Some(|f: &mut Fro| {
                    unsafe {
                        Box::from_raw(f.base as *mut u8);
                    }
                });
            }
            ReadMethod::Mem => {
                let mut buf = Vec::with_capacity(metadata.len() as usize);
                file.take(metadata.len()).read_to_end(&mut buf)?;
                f.base = buf.as_ptr();
                f.ptr = f.base;
                f.lim = unsafe { f.base.add(buf.len()) };
                // Leak the buffer to keep it alive
                std::mem::forget(buf);
            }
            ReadMethod::Stdio => {
                f.stream = Some(file);
            }
        }

        Ok((f, metadata))
    }

    fn close(&mut self) -> io::Result<()> {
        match self.rm {
            ReadMethod::Mmap | ReadMethod::Mem => {
                if let Some(dealloc) = self.deallocate.take() {
                    dealloc(self);
                }
                if let Some(fd) = self.fd.take() {
                    unsafe {
                        libc::close(fd);
                    }
                }
            }
            ReadMethod::Stdio => {
                if let Some(stream) = self.stream.take() {
                    drop(stream);
                }
            }
        }
        Ok(())
    }

    fn tello(&self) -> io::Result<off_t> {
        match self.rm {
            ReadMethod::Mmap | ReadMethod::Mem => {
                Ok(unsafe { self.ptr.offset_from(self.base) } as off_t)
            }
            ReadMethod::Stdio => {
                self.stream.as_ref().unwrap().seek(SeekFrom::Current(0))
            }
        }
    }

    fn move_ptr(&mut self, change: off_t) -> io::Result<()> {
        match self.rm {
            ReadMethod::Mmap | ReadMethod::Mem => {
                if change < 0 {
                    self.ptr = unsafe { self.ptr.offset(change as isize) };
                } else {
                    self.ptr = unsafe { self.base.offset(change as isize) };
                }
                Ok(())
            }
            ReadMethod::Stdio => {
                let pos = if change < 0 {
                    SeekFrom::Current(change)
                } else {
                    SeekFrom::Start(change as u64)
                };
                self.stream.as_ref().unwrap().seek(pos).map(|_| ())
            }
        }
    }

    fn try_getbyte(&mut self) -> io::Result<Option<u8>> {
        match self.rm {
            ReadMethod::Mmap | ReadMethod::Mem => {
                if self.ptr == self.lim {
                    Ok(None)
                } else {
                    let byte = unsafe { *self.ptr };
                    self.ptr = unsafe { self.ptr.add(1) };
                    Ok(Some(byte))
                }
            }
            ReadMethod::Stdio => {
                let mut buf = [0u8; 1];
                match self.stream.as_ref().unwrap().read_exact(&mut buf) {
                    Ok(_) => Ok(Some(buf[0])),
                    Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
                    Err(e) => Err(e),
                }
            }
        }
    }

    fn must_getbyte(&mut self) -> io::Result<u8> {
        self.try_getbyte()?.ok_or(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "unexpected end of file",
        ))
    }

    fn spew_partial(&mut self, to: &mut impl Write, r: &Range) -> io::Result<()> {
        match self.rm {
            ReadMethod::Mmap | ReadMethod::Mem => {
                let len = (r.end - r.beg) as usize;
                let data = unsafe { slice::from_raw_parts(self.base.add(r.beg as usize), len) };
                to.write_all(data)?;
                if self.end == r.end {
                    self.ptr = self.lim;
                }
            }
            ReadMethod::Stdio => {
                const MEMBUFSIZ: usize = 8 * 1024;
                let mut buf = [0u8; MEMBUFSIZ];
                let mut pos = r.beg;

                self.stream.as_ref().unwrap().seek(SeekFrom::Start(r.beg as u64))?;
                while pos < r.end {
                    let chunk_size = std::cmp::min(MEMBUFSIZ, (r.end - pos) as usize);
                    let bytes_read = self.stream.as_ref().unwrap().read(&mut buf[..chunk_size])?;
                    if bytes_read == 0 {
                        return Err(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "unexpected end of file",
                        ));
                    }
                    to.write_all(&buf[..bytes_read])?;
                    pos += bytes_read as off_t;
                }
            }
        }
        Ok(())
    }

    fn spew(&mut self, to: &mut impl Write) -> io::Result<()> {
        let range = Range {
            beg: self.verbatim,
            end: self.end,
        };
        self.spew_partial(to, &range)?;
        self.verbatim = self.end;
        Ok(())
    }
}

impl Drop for Fro {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

struct AtAt {
    count: usize,
    lno: usize,
    line_count: usize,
    from: Arc<Fro>,
    beg: off_t,
    holes: Vec<off_t>,
}

impl AtAt {
    fn string_from_atat(&self, space: &mut Vec<u8>) -> io::Result<Vec<u8>> {
        let mut result = Vec::new();
        for i in 0..self.count {
            let rbeg = if i == 0 {
                self.beg + 1
            } else {
                self.holes[i - 1] + 1
            };
            let rend = self.holes[i];
            
            match self.from.rm {
                ReadMethod::Mmap | ReadMethod::Mem => {
                    let len = (rend - rbeg) as usize;
                    let data = unsafe {
                        slice::from_raw_parts(self.from.base.add(rbeg as usize), len)
                    };
                    result.extend_from_slice(data);
                }
                ReadMethod::Stdio => {
                    let mut stream = self.from.stream.as_ref().unwrap();
                    let pos = stream.seek(SeekFrom::Current(0))?;
                    stream.seek(SeekFrom::Start(rbeg as u64))?;
                    
                    let mut buf = vec![0u8; (rend - rbeg) as usize];
                    stream.read_exact(&mut buf)?;
                    result.extend_from_slice(&buf);
                    
                    stream.seek(SeekFrom::Start(pos))?;
                }
            }
        }
        Ok(result)
    }

    fn put(&self, to: &mut impl Write) -> io::Result<()> {
        let range = Range {
            beg: self.beg,
            end: self.holes.last().map(|&h| h + 2).unwrap_or(self.beg + 2),
        };
        self.from.spew_partial(to, &range)
    }

    fn display(&self, to: &mut impl Write, ensure_end_nl: bool) -> io::Result<()> {
        for i in 0..self.count {
            let range = Range {
                beg: if i == 0 {
                    self.beg + 1
                } else {
                    self.holes[i - 1] + 1
                },
                end: self.holes[i],
            };
            self.from.spew_partial(to, &range)?;
        }

        if !ensure_end_nl || (self.count == 1 && self.beg + 1 == self.holes[0]) {
            return Ok(());
        }

        let pos = self.holes[self.count - 1] - 1;
        let lc = match self.from.rm {
            ReadMethod::Mmap | ReadMethod::Mem => unsafe {
                *self.from.base.add(pos as usize)
            },
            ReadMethod::Stdio => {
                let mut stream = self.from.stream.as_ref().unwrap();
                let current = stream.seek(SeekFrom::Current(0))?;
                stream.seek(SeekFrom::Start(pos as u64))?;
                let mut buf = [0u8; 1];
                stream.read_exact(&mut buf)?;
                stream.seek(SeekFrom::Start(current))?;
                buf[0]
            }
        };

        if lc != b'\n' {
            writeln!(to)?;
        }
        Ok(())
    }
}
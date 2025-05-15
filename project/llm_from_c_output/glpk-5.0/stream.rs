use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom, Error, ErrorKind};
use std::path::Path;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::ptr;
use libc::{fopen, fclose, fread, fwrite, fflush, ferror, feof, stdin, stdout, stderr, FILE};
use libz_sys::{gzFile, gzopen, gzclose, gzread, gzwrite, gzerror, Z_OK, Z_ERRNO};
use std::mem;
use std::slice;
use std::fmt;

const BUFSIZ: usize = 8192;

bitflags! {
    struct StreamFlags: u32 {
        const IONULL = 0x01;
        const IOSTD = 0x02;
        const IOGZIP = 0x04;
        const IOWRT = 0x08;
        const IOEOF = 0x10;
        const IOERR = 0x20;
    }
}

struct GlpFile {
    base: Vec<u8>,
    size: usize,
    ptr: usize,
    cnt: usize,
    flag: StreamFlags,
    file: FileType,
}

enum FileType {
    Null,
    StdIn,
    StdOut,
    StdErr,
    Regular(File),
    Gzip(gzFile),
}

impl GlpFile {
    fn new(flag: StreamFlags, file: FileType) -> Self {
        GlpFile {
            base: vec![0; BUFSIZ],
            size: BUFSIZ,
            ptr: 0,
            cnt: 0,
            flag,
            file,
        }
    }

    fn eof(&self) -> bool {
        self.flag.contains(IOEOF)
    }

    fn ioerr(&self) -> bool {
        self.flag.contains(IOERR)
    }

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.flag.contains(IOWRT) {
            return Err(Error::new(ErrorKind::Other, "attempt to read from output stream"));
        }

        if buf.len() < 1 {
            return Err(Error::new(ErrorKind::InvalidInput, "invalid buffer size"));
        }

        let mut nrd = 0;
        while nrd < buf.len() {
            if self.cnt == 0 {
                let cnt = match self.file {
                    FileType::Null => 0,
                    FileType::StdIn => {
                        let file = unsafe { stdin };
                        let cnt = unsafe { fread(self.base.as_mut_ptr() as *mut c_char, 1, self.size, file) };
                        if unsafe { ferror(file) } != 0 {
                            self.flag.insert(IOERR);
                            return Err(Error::last_os_error());
                        }
                        cnt
                    },
                    FileType::Regular(ref mut file) => {
                        file.read(&mut self.base)?
                    },
                    FileType::Gzip(file) => {
                        let cnt = unsafe { gzread(file, self.base.as_mut_ptr() as *mut c_char, self.size as i32) };
                        if cnt < 0 {
                            self.flag.insert(IOERR);
                            let mut errnum = 0;
                            let msg = unsafe { gzerror(file, &mut errnum) };
                            if errnum == Z_ERRNO {
                                return Err(Error::last_os_error());
                            } else {
                                return Err(Error::new(ErrorKind::Other, unsafe { CStr::from_ptr(msg) }.to_string_lossy().into_owned()));
                            }
                        }
                        cnt as usize
                    },
                    _ => return Err(Error::new(ErrorKind::Other, "invalid stream type for reading")),
                };

                if cnt == 0 {
                    if nrd == 0 {
                        self.flag.insert(IOEOF);
                    }
                    break;
                }

                self.ptr = 0;
                self.cnt = cnt;
            }

            let cnt = std::cmp::min(buf.len() - nrd, self.cnt);
            buf[nrd..nrd + cnt].copy_from_slice(&self.base[self.ptr..self.ptr + cnt]);
            self.ptr += cnt;
            self.cnt -= cnt;
            nrd += cnt;
        }

        Ok(nrd)
    }

    fn getc(&mut self) -> io::Result<Option<u8>> {
        let mut buf = [0u8; 1];
        match self.read(&mut buf)? {
            0 => Ok(None),
            1 => Ok(Some(buf[0])),
            _ => unreachable!(),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.cnt > 0 {
            match self.file {
                FileType::Null => (),
                FileType::StdOut => {
                    let file = unsafe { stdout };
                    if unsafe { fwrite(self.base.as_ptr() as *const c_char, 1, self.cnt, file) } != self.cnt {
                        self.flag.insert(IOERR);
                        return Err(Error::last_os_error());
                    }
                },
                FileType::StdErr => {
                    let file = unsafe { stderr };
                    if unsafe { fwrite(self.base.as_ptr() as *const c_char, 1, self.cnt, file) } != self.cnt {
                        self.flag.insert(IOERR);
                        return Err(Error::last_os_error());
                    }
                },
                FileType::Regular(ref mut file) => {
                    file.write_all(&self.base[..self.cnt])?;
                },
                FileType::Gzip(file) => {
                    if unsafe { gzwrite(file, self.base.as_ptr() as *const c_char, self.cnt as i32) } != self.cnt as i32 {
                        self.flag.insert(IOERR);
                        let mut errnum = 0;
                        let msg = unsafe { gzerror(file, &mut errnum) };
                        if errnum == Z_ERRNO {
                            return Err(Error::last_os_error());
                        } else {
                            return Err(Error::new(ErrorKind::Other, unsafe { CStr::from_ptr(msg) }.to_string_lossy().into_owned()));
                        }
                    }
                },
                _ => return Err(Error::new(ErrorKind::Other, "invalid stream type for writing")),
            }
        }

        self.ptr = 0;
        self.cnt = 0;
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if !self.flag.contains(IOWRT) {
            return Err(Error::new(ErrorKind::Other, "attempt to write to input stream"));
        }

        if buf.len() < 1 {
            return Err(Error::new(ErrorKind::InvalidInput, "invalid buffer size"));
        }

        let mut nwr = 0;
        while nwr < buf.len() {
            let cnt = std::cmp::min(buf.len() - nwr, self.size - self.cnt);
            self.base[self.ptr..self.ptr + cnt].copy_from_slice(&buf[nwr..nwr + cnt]);
            self.ptr += cnt;
            self.cnt += cnt;
            nwr += cnt;

            if self.cnt == self.size {
                self.flush()?;
            }
        }

        Ok(nwr)
    }

    fn close(mut self) -> io::Result<()> {
        if self.flag.contains(IOWRT) {
            self.flush()?;
        }

        match self.file {
            FileType::Null | FileType::StdIn | FileType::StdOut | FileType::StdErr => (),
            FileType::Regular(file) => {
                file.sync_all()?;
            },
            FileType::Gzip(file) => {
                let errnum = unsafe { gzclose(file) };
                if errnum != Z_OK {
                    if errnum == Z_ERRNO {
                        return Err(Error::last_os_error());
                    } else {
                        return Err(Error::new(ErrorKind::Other, format!("gzclose returned {}", errnum)));
                    }
                }
            },
        }

        Ok(())
    }
}

fn glp_open(name: &str, mode: &str) -> io::Result<GlpFile> {
    let flag = match mode {
        "r" | "rb" => StreamFlags::empty(),
        "w" | "wb" | "a" | "ab" => IOWRT,
        _ => return Err(Error::new(ErrorKind::InvalidInput, "invalid mode string")),
    };

    let file = match name {
        "/dev/null" => FileType::Null,
        "/dev/stdin" => FileType::StdIn,
        "/dev/stdout" => FileType::StdOut,
        "/dev/stderr" => FileType::StdErr,
        _ => {
            if name.ends_with(".gz") {
                let mode_c = CString::new(mode).unwrap();
                let name_c = CString::new(name).unwrap();
                let file = unsafe { gzopen(name_c.as_ptr(), mode_c.as_ptr()) };
                if file.is_null() {
                    return Err(Error::last_os_error());
                }
                FileType::Gzip(file)
            } else {
                let file = OpenOptions::new()
                    .read(mode.contains('r'))
                    .write(mode.contains('w') || mode.contains('a'))
                    .create(mode.contains('w') || mode.contains('a'))
                    .append(mode.contains('a'))
                    .truncate(mode.contains('w'))
                    .open(name)?;
                FileType::Regular(file)
            }
        }
    };

    Ok(GlpFile::new(flag, file))
}

fn glp_eof(f: &GlpFile) -> bool {
    f.eof()
}

fn glp_ioerr(f: &GlpFile) -> bool {
    f.ioerr()
}

fn glp_read(f: &mut GlpFile, buf: &mut [u8]) -> io::Result<usize> {
    f.read(buf)
}

fn glp_getc(f: &mut GlpFile) -> io::Result<Option<u8>> {
    f.getc()
}

fn glp_write(f: &mut GlpFile, buf: &[u8]) -> io::Result<usize> {
    f.write(buf)
}

fn glp_close(f: GlpFile) -> io::Result<()> {
    f.close()
}
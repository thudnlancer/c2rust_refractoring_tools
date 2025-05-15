use std::fs::{File, OpenOptions};
use std::io::{self, Write, Error, ErrorKind};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::Path;
use libc::mode_t;

pub fn change_mode(fd: RawFd, mode: mode_t) -> io::Result<()> {
    unsafe {
        if libc::fchmod(fd, mode) == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

pub fn ierror() -> ! {
    panic!("input error");
}

pub fn test_ierror(f: &mut File) {
    if f.metadata().is_err() {
        ierror();
    }
}

pub fn oerror() -> ! {
    panic!("output error");
}

pub fn test_oerror(o: &mut File) {
    if o.metadata().is_err() {
        oerror();
    }
}

pub fn fopen_safer(filename: &Path, mode: &str) -> io::Result<File> {
    let file = OpenOptions::new()
        .read(mode.contains('r'))
        .write(mode.contains('w'))
        .create(mode.contains('w'))
        .truncate(mode.contains('w'))
        .append(mode.contains('a'))
        .open(filename)?;

    let fd = file.as_raw_fd();
    if fd >= 0 && fd <= 2 {
        let new_fd = unsafe { libc::dup(fd) };
        if new_fd == -1 {
            return Err(Error::last_os_error());
        }
        let new_file = unsafe { File::from_raw_fd(new_fd) };
        Ok(new_file)
    } else {
        Ok(file)
    }
}

pub fn ozclose(file: Option<File>) -> io::Result<()> {
    if let Some(mut f) = file {
        f.flush()?;
    }
    Ok(())
}

pub fn aflush(f: &mut File) -> io::Result<()> {
    f.flush()
}

pub fn oflush(stdout: Option<&mut File>) -> io::Result<()> {
    if let Some(f) = stdout {
        f.flush()
    } else {
        io::stdout().flush()
    }
}

pub fn afputc(c: u8, f: &mut File) -> io::Result<()> {
    f.write_all(&[c])
}

pub fn newline(f: &mut File) -> io::Result<()> {
    afputc(b'\n', f)
}

pub fn aputs(s: &str, iop: &mut File) -> io::Result<()> {
    write!(iop, "{}", s)
}

pub fn aprintf(iop: &mut File, fmt: &str, args: std::fmt::Arguments) -> io::Result<()> {
    write!(iop, "{}", args)
}
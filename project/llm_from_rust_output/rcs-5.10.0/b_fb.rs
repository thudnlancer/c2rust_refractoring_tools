use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufWriter, ErrorKind};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::path::Path;
use std::ffi::CString;
use libc::{mode_t, c_int};

struct Top {
    // Placeholder for top-level context
}

struct Behavior {
    Oerrloop: bool,
    // Other behavior fields
}

struct Manifestation {
    standard_output: Option<BufWriter<File>>,
    // Other manifestation fields
}

impl Top {
    fn new() -> Self {
        Top {
            // Initialize fields
        }
    }

    fn behavior(&self) -> &Behavior {
        // Return reference to behavior
        &Behavior { Oerrloop: false }
    }

    fn behavior_mut(&mut self) -> &mut Behavior {
        // Return mutable reference to behavior
        &mut Behavior { Oerrloop: false }
    }

    fn manifestation(&mut self) -> &mut Manifestation {
        // Return mutable reference to manifestation
        &mut Manifestation { standard_output: None }
    }
}

fn change_mode(fd: c_int, mode: mode_t) -> io::Result<()> {
    unsafe {
        if libc::fchmod(fd, mode) != 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

fn ierror() -> ! {
    panic!("input error");
}

fn test_ierror(file: &File) {
    if let Err(e) = file.sync_all() {
        if e.kind() == ErrorKind::Other {
            ierror();
        }
    }
}

fn oerror(top: &mut Top) -> ! {
    if top.behavior().Oerrloop {
        // thank_you_and_goodnight
        std::process::exit(1);
    }
    top.behavior_mut().Oerrloop = true;
    panic!("output error");
}

fn test_oerror(writer: &mut impl Write, top: &mut Top) {
    if let Err(e) = writer.flush() {
        if e.kind() == ErrorKind::Other {
            oerror(top);
        }
    }
}

fn fopen_safer(filename: &Path, mode: &str) -> io::Result<File> {
    let file = OpenOptions::new()
        .read(mode.contains('r'))
        .write(mode.contains('w'))
        .create(mode.contains('w'))
        .truncate(mode.contains('w'))
        .open(filename)?;

    let fd = file.as_raw_fd();
    if fd <= 2 {
        let new_fd = unsafe { libc::dup(fd) };
        if new_fd == -1 {
            return Err(io::Error::last_os_error());
        }
        unsafe { File::from_raw_fd(new_fd) }
    } else {
        Ok(file)
    }
}

fn ozclose(file: &mut Option<File>) {
    if let Some(f) = file.take() {
        if let Err(e) = f.sync_all() {
            if e.kind() == ErrorKind::Other {
                panic!("output error");
            }
        }
    }
}

fn aflush(writer: &mut impl Write) -> io::Result<()> {
    writer.flush()
}

fn oflush(top: &mut Top) {
    let stdout = top.manifestation().standard_output
        .as_mut()
        .map(|w| w as &mut dyn Write)
        .unwrap_or(&mut io::stdout());

    if let Err(e) = stdout.flush() {
        if e.kind() == ErrorKind::Other && !top.behavior().Oerrloop {
            oerror(top);
        }
    }
}

fn afputc(c: u8, writer: &mut impl Write) -> io::Result<()> {
    writer.write_all(&[c])
}

fn newline(writer: &mut impl Write) -> io::Result<()> {
    writer.write_all(b"\n")
}

fn aputs(s: &str, writer: &mut impl Write) -> io::Result<()> {
    writer.write_all(s.as_bytes())
}

fn aprintf(writer: &mut impl Write, fmt: &str, args: std::fmt::Arguments) -> io::Result<()> {
    write!(writer, "{}", args)
}
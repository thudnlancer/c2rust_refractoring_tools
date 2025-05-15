use std::os::unix::io::{AsRawFd, RawFd};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use nix::fcntl::{fcntl, FcntlArg, OFlag};
use nix::sys::stat::fstat;
use nix::sys::select::{pselect, FdSet};
use nix::sys::signal::{sigemptyset, SigSet};
use nix::unistd::{close, dup, pipe, write};
use libc::{EBADF, EINTR, EAGAIN, O_NONBLOCK, O_RDONLY, O_WRONLY, O_APPEND, O_CLOEXEC};

const IO_UNKNOWN: u32 = 0;
const IO_STDIN_OK: u32 = 1 << 0;
const IO_STDOUT_OK: u32 = 1 << 1;
const IO_STDERR_OK: u32 = 1 << 2;
const IO_COMBINED_OUTERR: u32 = 1 << 3;

fn check_io_state() -> u32 {
    static STATE: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    
    *STATE.get_or_init(|| {
        let mut state = IO_UNKNOWN;
        
        if stream_ok(&io::stdin()) {
            state |= IO_STDIN_OK;
        }
        if stream_ok(&io::stdout()) {
            state |= IO_STDOUT_OK;
        }
        if stream_ok(&io::stderr()) {
            state |= IO_STDERR_OK;
        }
        
        if (state & (IO_STDOUT_OK | IO_STDERR_OK)) == (IO_STDOUT_OK | IO_STDERR_OK) {
            if let (Ok(stat_o), Ok(stat_e)) = (
                fstat(io::stdout().as_raw_fd()),
                fstat(io::stderr().as_raw_fd()),
            ) {
                if stat_o.st_dev == stat_e.st_dev && stat_o.st_ino == stat_e.st_ino {
                    state |= IO_COMBINED_OUTERR;
                }
            }
        }
        
        state
    })
}

fn stream_ok(stream: &impl AsRawFd) -> bool {
    match fcntl(stream.as_raw_fd(), FcntlArg::F_GETFD) {
        Ok(_) => true,
        Err(e) => e != nix::errno::Errno::EBADF,
    }
}

#[derive(Debug)]
enum JobServerType {
    None,
    Pipe,
    Fifo(PathBuf),
}

struct JobServer {
    job_type: JobServerType,
    read_fd: RawFd,
    write_fd: RawFd,
    notify_fd: RawFd,
    is_root: bool,
    token: u8,
}

impl JobServer {
    fn new() -> Self {
        Self {
            job_type: JobServerType::None,
            read_fd: -1,
            write_fd: -1,
            notify_fd: -1,
            is_root: false,
            token: b'+',
        }
    }

    fn setup(&mut self, slots: usize, style: Option<&str>) -> io::Result<()> {
        if let Some("fifo") = style {
            let tmpdir = std::env::temp_dir();
            let fifo_name = tmpdir.join(format!("GMfifo{}", std::process::id()));
            
            nix::unistd::mkfifo(&fifo_name, nix::sys::stat::Mode::S_IRUSR | nix::sys::stat::Mode::S_IWUSR)?;
            
            self.read_fd = nix::fcntl::open(
                &fifo_name,
                OFlag::O_RDONLY | OFlag::O_NONBLOCK,
                nix::sys::stat::Mode::empty(),
            )?;
            
            self.write_fd = nix::fcntl::open(
                &fifo_name,
                OFlag::O_WRONLY,
                nix::sys::stat::Mode::empty(),
            )?;
            
            self.job_type = JobServerType::Fifo(fifo_name);
        } else {
            let (read_fd, write_fd) = pipe()?;
            self.read_fd = read_fd;
            self.write_fd = write_fd;
            self.job_type = JobServerType::Pipe;
        }
        
        self.set_nonblocking(self.read_fd, false)?;
        
        for _ in 0..slots {
            nix::unistd::write(self.write_fd, &[self.token])?;
        }
        
        self.is_root = true;
        Ok(())
    }

    fn set_nonblocking(&self, fd: RawFd, blocking: bool) -> io::Result<()> {
        let mut flags = OFlag::from_bits_truncate(fcntl(fd, FcntlArg::F_GETFL)?);
        flags.set(OFlag::O_NONBLOCK, !blocking);
        fcntl(fd, FcntlArg::F_SETFL(flags))?;
        Ok(())
    }

    fn acquire(&self, timeout: Option<Duration>) -> io::Result<bool> {
        let mut fdset = FdSet::new();
        fdset.insert(self.read_fd);
        
        let mut sigset = SigSet::empty();
        sigemptyset(&mut sigset);
        
        let timeout = timeout.map(|d| nix::sys::time::TimeSpec::from(d));
        
        match pselect(
            self.read_fd + 1,
            Some(&mut fdset),
            None,
            None,
            timeout.as_ref(),
            Some(&sigset),
        ) {
            Ok(1) => {
                let mut buf = [0];
                match nix::unistd::read(self.read_fd, &mut buf) {
                    Ok(1) => Ok(true),
                    Ok(_) => Ok(false),
                    Err(e) if e == nix::errno::Errno::EAGAIN => Ok(false),
                    Err(e) => Err(e.into()),
                }
            }
            Ok(0) => Ok(false),
            Ok(_) => unreachable!(),
            Err(e) if e == nix::errno::Errno::EINTR => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    fn release(&self) -> io::Result<()> {
        nix::unistd::write(self.write_fd, &[self.token])?;
        Ok(())
    }
}

impl Drop for JobServer {
    fn drop(&mut self) {
        if self.read_fd != -1 {
            let _ = close(self.read_fd);
        }
        if self.write_fd != -1 {
            let _ = close(self.write_fd);
        }
        if self.notify_fd != -1 {
            let _ = close(self.notify_fd);
        }
        
        if let JobServerType::Fifo(ref path) = self.job_type {
            if self.is_root {
                let _ = std::fs::remove_file(path);
            }
        }
    }
}

struct OutputSync {
    handle: Option<File>,
    temp_path: Option<PathBuf>,
    is_root: bool,
}

impl OutputSync {
    fn new() -> Self {
        Self {
            handle: None,
            temp_path: None,
            is_root: false,
        }
    }

    fn setup(&mut self) -> io::Result<()> {
        let temp_file = tempfile::NamedTempFile::new()?;
        let path = temp_file.path().to_path_buf();
        self.handle = Some(temp_file.reopen()?);
        self.temp_path = Some(path);
        self.is_root = true;
        Ok(())
    }

    fn acquire(&self) -> io::Result<()> {
        if let Some(file) = &self.handle {
            let lock = nix::fcntl::flock(file.as_raw_fd(), nix::fcntl::FlockArg::LockExclusive)?;
            Ok(())
        } else {
            Ok(())
        }
    }

    fn release(&self) -> io::Result<()> {
        if let Some(file) = &self.handle {
            let _ = nix::fcntl::flock(file.as_raw_fd(), nix::fcntl::FlockArg::Unlock)?;
        }
        Ok(())
    }
}

impl Drop for OutputSync {
    fn drop(&mut self) {
        if self.is_root {
            if let Some(path) = &self.temp_path {
                let _ = std::fs::remove_file(path);
            }
        }
    }
}

fn get_bad_stdin() -> io::Result<RawFd> {
    static BAD_STDIN: std::sync::OnceLock<io::Result<RawFd>> = std::sync::OnceLock::new();
    
    BAD_STDIN.get_or_init(|| {
        let (read_fd, write_fd) = pipe()?;
        close(write_fd)?;
        Ok(read_fd)
    }).clone()
}

fn set_fd_cloexec(fd: RawFd, cloexec: bool) -> io::Result<()> {
    let mut flags = OFlag::from_bits_truncate(fcntl(fd, FcntlArg::F_GETFD)?);
    flags.set(OFlag::FD_CLOEXEC, cloexec);
    fcntl(fd, FcntlArg::F_SETFD(flags))?;
    Ok(())
}

fn fd_inherit(fd: RawFd) -> io::Result<()> {
    set_fd_cloexec(fd, false)
}

fn fd_noinherit(fd: RawFd) -> io::Result<()> {
    set_fd_cloexec(fd, true)
}

fn fd_set_append(fd: RawFd) -> io::Result<()> {
    if let Ok(stat) = nix::sys::stat::fstat(fd) {
        if nix::sys::stat::SFlag::from_bits_truncate(stat.st_mode).contains(nix::sys::stat::SFlag::S_IFREG) {
            let mut flags = OFlag::from_bits_truncate(fcntl(fd, FcntlArg::F_GETFL)?);
            flags.insert(OFlag::O_APPEND);
            fcntl(fd, FcntlArg::F_SETFL(flags))?;
        }
    }
    Ok(())
}

fn os_anontmp() -> io::Result<File> {
    let tmpdir = std::env::temp_dir();
    
    #[cfg(target_os = "linux")]
    {
        match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&tmpdir)
            .and_then(|f| {
                f.set_permissions(std::fs::Permissions::from_mode(0o600))?;
                Ok(f)
            }) {
            Ok(f) => return Ok(f),
            Err(e) => eprintln!("Cannot open temp file with O_TMPFILE: {}", e),
        }
    }
    
    tempfile::tempfile()
}
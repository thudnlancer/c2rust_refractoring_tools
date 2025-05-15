use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use nix::fcntl::{fcntl, FcntlArg, OFlag};
use nix::sys::stat::{fstat, FileStat};
use nix::unistd::{close, dup, pipe, read, write};
use libc::{c_char, c_int, c_uint, mode_t};

const JS_NONE: u32 = 0;
const JS_PIPE: u32 = 1;
const JS_FIFO: u32 = 2;

struct JobServer {
    job_fds: [RawFd; 2],
    job_rfd: RawFd,
    token: char,
    js_type: u32,
    fifo_name: Option<String>,
    job_root: bool,
}

impl JobServer {
    fn new() -> Self {
        JobServer {
            job_fds: [-1, -1],
            job_rfd: -1,
            token: '+',
            js_type: JS_NONE,
            fifo_name: None,
            job_root: false,
        }
    }

    fn setup(&mut self, slots: i32, style: Option<&str>) -> Result<(), io::Error> {
        if let Some("fifo") = style {
            let tmpdir = get_tmpdir();
            let fifo_name = format!("{}/GMfifo{}", tmpdir, make_pid());
            
            nix::sys::stat::mkfifo(&fifo_name, nix::sys::stat::Mode::S_IRUSR | nix::sys::stat::Mode::S_IWUSR)?;
            
            self.job_fds[0] = OpenOptions::new()
                .read(true)
                .open(&fifo_name)?
                .as_raw_fd();
            
            self.job_fds[1] = OpenOptions::new()
                .write(true)
                .open(&fifo_name)?
                .as_raw_fd();
            
            self.js_type = JS_FIFO;
            self.fifo_name = Some(fifo_name);
        } else {
            let (rfd, wfd) = pipe()?;
            self.job_fds = [rfd, wfd];
            self.js_type = JS_PIPE;
        }

        self.fd_noinherit(self.job_fds[0])?;
        self.fd_noinherit(self.job_fds[1])?;

        if self.make_job_rfd()? < 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to create job rfd"));
        }

        for _ in 0..slots {
            write(self.job_fds[1], &[self.token as u8])?;
        }

        self.set_blocking(self.job_fds[0], false)?;
        self.job_root = true;
        Ok(())
    }

    fn make_job_rfd(&mut self) -> Result<RawFd, io::Error> {
        if self.job_fds[0] >= 0 {
            self.job_rfd = dup(self.job_fds[0])?;
            Ok(self.job_rfd)
        } else {
            Ok(-1)
        }
    }

    fn set_blocking(&self, fd: RawFd, blocking: bool) -> Result<(), io::Error> {
        let flags = fcntl(fd, FcntlArg::F_GETFL)?;
        let new_flags = if blocking {
            flags & !OFlag::O_NONBLOCK.bits()
        } else {
            flags | OFlag::O_NONBLOCK.bits()
        };
        fcntl(fd, FcntlArg::F_SETFL(new_flags))?;
        Ok(())
    }

    fn fd_noinherit(&self, fd: RawFd) -> Result<(), io::Error> {
        let flags = fcntl(fd, FcntlArg::F_GETFD)?;
        fcntl(fd, FcntlArg::F_SETFD(flags | nix::fcntl::FdFlag::FD_CLOEXEC.bits()))?;
        Ok(())
    }

    fn clear(&mut self) {
        if self.job_fds[0] >= 0 {
            let _ = close(self.job_fds[0]);
        }
        if self.job_fds[1] >= 0 {
            let _ = close(self.job_fds[1]);
        }
        if self.job_rfd >= 0 {
            let _ = close(self.job_rfd);
        }
        
        self.job_rfd = -1;
        self.job_fds = [-1, -1];
        
        if let Some(ref name) = self.fifo_name {
            if self.job_root {
                let _ = std::fs::remove_file(name);
            }
            self.fifo_name = None;
        }
        
        self.js_type = JS_NONE;
    }

    fn release(&self, is_fatal: bool) -> Result<(), io::Error> {
        write(self.job_fds[1], &[self.token as u8])?;
        Ok(())
    }

    fn acquire(&self, timeout: i32) -> Result<bool, io::Error> {
        let mut fds = nix::poll::PollFd::new(self.job_fds[0], nix::poll::PollFlags::POLLIN);
        let timeout_ms = if timeout != 0 { 1000 } else { -1 };
        
        match nix::poll::poll(&mut [fds], timeout_ms)? {
            0 => Ok(false),
            _ => {
                let mut buf = [0u8; 1];
                match read(self.job_fds[0], &mut buf)? {
                    1 => Ok(true),
                    _ => Ok(false),
                }
            }
        }
    }
}

fn get_tmpdir() -> String {
    std::env::temp_dir().to_string_lossy().into_owned()
}

fn make_pid() -> i32 {
    unsafe { libc::getpid() }
}

struct OutputSync {
    handle: RawFd,
    tmpfile: Option<String>,
    sync_root: bool,
}

impl OutputSync {
    fn new() -> Self {
        OutputSync {
            handle: -1,
            tmpfile: None,
            sync_root: false,
        }
    }

    fn setup(&mut self) -> Result<(), io::Error> {
        let tmpfile = format!("{}/osync{}", get_tmpdir(), make_pid());
        self.handle = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&tmpfile)?
            .as_raw_fd();
        self.tmpfile = Some(tmpfile);
        self.sync_root = true;
        Ok(())
    }

    fn clear(&mut self) {
        if self.handle >= 0 {
            let _ = close(self.handle);
            self.handle = -1;
        }
        if self.sync_root {
            if let Some(ref name) = self.tmpfile {
                let _ = std::fs::remove_file(name);
            }
            self.tmpfile = None;
        }
    }

    fn acquire(&self) -> Result<bool, io::Error> {
        let lock = nix::fcntl::flock(self.handle, nix::fcntl::FlockArg::LockExclusive)?;
        Ok(lock.is_some())
    }

    fn release(&self) -> Result<(), io::Error> {
        nix::fcntl::flock(self.handle, nix::fcntl::FlockArg::Unlock)?;
        Ok(())
    }
}
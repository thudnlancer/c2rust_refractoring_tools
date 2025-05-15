use std::env;
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::sync::Once;
use libc::{self, c_int, mode_t, O_CLOEXEC, F_GETFD, FD_CLOEXEC, O_RDONLY, O_CREAT, pollfd, POLLIN, POLLOUT, POLLNVAL};
use nix::dir::{Dir, Type};
use nix::fcntl::{fcntl, FdFlag};
use nix::sys::resource::{getrlimit, Resource};
use nix::errno::Errno;

static mut NON_CLOEXEC_FDS: Option<Vec<RawFd>> = None;
static mut NUM_CLOEXEC_FDS: usize = 0;
static CLOEXEC_STATUS: Once = Once::new();
static mut CLOEXEC_WORKS: bool = false;

fn get_proc_max_fd() -> io::Result<RawFd> {
    let path = "/proc/self/fd";
    let mut maxfd = -1;
    
    if let Ok(dir) = Dir::open(path) {
        let mut good = false;
        
        for entry in dir.iter() {
            if let Ok(entry) = entry {
                let name = entry.file_name();
                if name != "." && name != ".." {
                    if let Ok(fd) = name.to_str().unwrap().parse::<RawFd>() {
                        if fd > maxfd {
                            maxfd = fd;
                        }
                        good = true;
                    }
                }
            }
        }
        
        if good {
            return Ok(maxfd);
        }
    }
    
    Err(io::Error::last_os_error())
}

fn get_max_fd() -> RawFd {
    if let Ok(maxfd) = get_proc_max_fd() {
        return maxfd;
    }

    let open_max = unsafe { libc::sysconf(libc::_SC_OPEN_MAX) };
    if open_max == -1 {
        return libc::_POSIX_OPEN_MAX as RawFd;
    }

    if let Ok(limit) = getrlimit(Resource::RLIMIT_NOFILE) {
        if limit.rlim_cur == libc::RLIM_INFINITY {
            return open_max as RawFd;
        } else {
            return limit.rlim_cur as RawFd;
        }
    }

    open_max as RawFd
}

fn visit_open_fds<F>(fd_min: RawFd, fd_max: RawFd, mut callback: F) -> io::Result<()>
where
    F: FnMut(RawFd) -> io::Result<()>,
{
    const MAX_POLL: usize = 64;
    let mut pf: [pollfd; MAX_POLL] = unsafe { std::mem::zeroed() };
    let mut current_fd = fd_min;

    while current_fd < fd_max {
        let limit = std::cmp::min(MAX_POLL as RawFd, fd_max - current_fd) as usize;

        for i in 0..limit {
            pf[i].fd = current_fd + i as RawFd;
            pf[i].events = POLLIN | POLLOUT;
            pf[i].revents = 0;
        }

        let rv = unsafe { libc::poll(pf.as_mut_ptr(), limit as libc::nfds_t, 0) };
        if rv == -1 {
            return Err(io::Error::last_os_error());
        }

        for i in 0..limit {
            if pf[i].revents != POLLNVAL {
                callback(pf[i].fd)?;
            }
        }

        current_fd += limit as RawFd;
    }

    Ok(())
}

fn fd_is_cloexec(fd: RawFd) -> bool {
    match fcntl(fd, FdFlag::empty()) {
        Ok(flags) => flags.contains(FdFlag::FD_CLOEXEC),
        Err(_) => false,
    }
}

pub fn remember_non_cloexec_fds() -> io::Result<()> {
    let max_fd = get_max_fd();
    let mut fds = Vec::new();

    let visit_result = visit_open_fds(0, max_fd + 1, |fd| {
        if !fd_is_cloexec(fd) {
            fds.push(fd);
        }
        Ok(())
    });

    unsafe {
        NON_CLOEXEC_FDS = Some(fds);
        NUM_CLOEXEC_FDS = NON_CLOEXEC_FDS.as_ref().unwrap().len();
    }

    visit_result
}

fn find_first_leaked_fd(prev_non_cloexec_fds: &[RawFd]) -> Option<RawFd> {
    let max_fd = get_max_fd();
    let mut leaked_fd = None;
    let mut lookup_pos = 0;

    let _ = visit_open_fds(0, max_fd + 1, |fd| {
        if !fd_is_cloexec(fd) {
            while lookup_pos < prev_non_cloexec_fds.len() {
                if prev_non_cloexec_fds[lookup_pos] < fd {
                    lookup_pos += 1;
                } else if prev_non_cloexec_fds[lookup_pos] == fd {
                    return Ok(());
                } else {
                    break;
                }
            }
            leaked_fd = Some(fd);
            return Err(io::Error::new(io::ErrorKind::Other, "Leak found"));
        }
        Ok(())
    });

    leaked_fd
}

fn o_cloexec_works() -> bool {
    match File::open("/").map(|f| f.as_raw_fd()) {
        Ok(fd) => {
            let result = fd_is_cloexec(fd);
            unsafe { libc::close(fd) };
            result
        }
        Err(_) => false,
    }
}

pub fn open_cloexec(path: &Path, flags: c_int, mode: mode_t) -> io::Result<File> {
    CLOEXEC_STATUS.call_once(|| {
        unsafe { CLOEXEC_WORKS = o_cloexec_works() };
    });

    let mut open_options = OpenOptions::new();
    open_options.custom_flags(flags | O_CLOEXEC);

    if flags & O_CREAT != 0 {
        open_options.mode(mode);
    }

    let file = open_options.open(path)?;
    
    if !(O_CLOEXEC != 0 && unsafe { CLOEXEC_WORKS }) {
        if let Err(e) = fcntl(file.as_raw_fd(), FdFlag::FD_CLOEXEC) {
            return Err(io::Error::new(io::ErrorKind::Other, e));
        }
    }

    Ok(file)
}

pub fn forget_non_cloexec_fds() {
    unsafe {
        NON_CLOEXEC_FDS = None;
        NUM_CLOEXEC_FDS = 0;
    }
}

pub fn fd_leak_check_is_enabled() -> bool {
    env::var("GNU_FINDUTILS_FD_LEAK_CHECK").is_ok()
}

pub fn complain_about_leaky_fds() {
    let non_cloexec_fds = unsafe { NON_CLOEXEC_FDS.as_ref().unwrap_or(&Vec::new()) };
    if let Some(leaking_fd) = find_first_leaked_fd(non_cloexec_fds) {
        eprintln!(
            "File descriptor {} will leak; please report this as a bug, \
            remembering to include a detailed description of the simplest \
            way to reproduce this problem.",
            leaking_fd
        );
        panic!("File descriptor leak detected");
    }
}
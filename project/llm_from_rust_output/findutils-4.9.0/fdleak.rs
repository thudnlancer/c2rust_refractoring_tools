use std::env;
use std::fs;
use std::io;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::Path;
use libc::{self, c_int, c_void, mode_t, pollfd, nfds_t, rlimit, RLIMIT_NOFILE};
use nix::fcntl::{fcntl, FcntlArg, OFlag};
use nix::sys::resource::{getrlimit, Resource};
use nix::unistd::close;

const MAX_POLL: usize = 64;

struct RememberFdContext {
    buf: Vec<c_int>,
}

struct FdLeakContext<'a> {
    prev_buf: &'a [c_int],
    lookup_pos: usize,
    leaked_fd: c_int,
}

fn get_proc_max_fd() -> io::Result<c_int> {
    let path = Path::new("/proc/self/fd");
    let mut maxfd = -1;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let name = entry.file_name();
        if name.len() == 0 || name[0] == b'.' {
            continue;
        }

        if let Ok(fd) = name.to_string_lossy().parse::<c_int>() {
            if fd > maxfd {
                maxfd = fd;
            }
        }
    }

    Ok(maxfd)
}

fn get_max_fd() -> c_int {
    let open_max = match get_proc_max_fd() {
        Ok(fd) => fd as libc::c_long,
        Err(_) => -1,
    };

    if open_max >= 0 {
        return open_max as c_int;
    }

    let open_max = unsafe { libc::sysconf(libc::_SC_OPEN_MAX) };
    if open_max == -1 {
        20
    } else {
        match getrlimit(Resource::RLIMIT_NOFILE) {
            Ok(limit) => {
                if limit.soft == libc::RLIM_INFINITY {
                    open_max as c_int
                } else {
                    limit.soft as c_int
                }
            }
            Err(_) => open_max as c_int,
        }
    }
}

fn visit_open_fds<F>(fd_min: c_int, fd_max: c_int, mut callback: F) -> io::Result<()>
where
    F: FnMut(c_int) -> io::Result<()>,
{
    let mut pf = [pollfd {
        fd: 0,
        events: libc::POLLIN | libc::POLLPRI,
        revents: 0,
    }; MAX_POLL];

    let mut fd_min = fd_min;
    while fd_min < fd_max {
        let limit = (fd_max - fd_min).min(MAX_POLL as c_int);
        
        for i in 0..limit {
            pf[i as usize].fd = fd_min + i;
            pf[i as usize].revents = 0;
        }

        let rv = unsafe { libc::poll(pf.as_mut_ptr(), limit as nfds_t, 0) };
        if rv == -1 {
            return Err(io::Error::last_os_error());
        }

        for j in 0..limit {
            if pf[j as usize].revents != libc::POLLNVAL {
                callback(pf[j as usize].fd)?;
            }
        }

        fd_min += limit;
    }

    Ok(())
}

fn fd_is_cloexec(fd: c_int) -> bool {
    match fcntl(fd, FcntlArg::F_GETFD) {
        Ok(flags) => (flags & libc::FD_CLOEXEC) != 0,
        Err(_) => false,
    }
}

fn remember_non_cloexec_fds() -> Vec<c_int> {
    let max_fd = get_max_fd();
    let mut context = RememberFdContext { buf: Vec::new() };

    if max_fd < i32::MAX {
        let _ = visit_open_fds(0, max_fd + 1, |fd| {
            if !fd_is_cloexec(fd) {
                context.buf.push(fd);
            }
            Ok(())
        });
    }

    context.buf
}

fn find_first_leaked_fd(prev_non_cloexec_fds: &[c_int]) -> Option<c_int> {
    let max_fd = get_max_fd();
    let mut context = FdLeakContext {
        prev_buf: prev_non_cloexec_fds,
        lookup_pos: 0,
        leaked_fd: -1,
    };

    if max_fd < i32::MAX {
        let _ = visit_open_fds(0, max_fd + 1, |fd| {
            if !fd_is_cloexec(fd) {
                while context.lookup_pos < context.prev_buf.len() {
                    if context.prev_buf[context.lookup_pos] < fd {
                        context.lookup_pos += 1;
                    } else {
                        if context.prev_buf[context.lookup_pos] != fd {
                            context.leaked_fd = fd;
                            return Err(io::Error::new(io::ErrorKind::Other, "Found leak"));
                        }
                        break;
                    }
                }
            }
            Ok(())
        });
    }

    if context.leaked_fd != -1 {
        Some(context.leaked_fd)
    } else {
        None
    }
}

fn o_cloexec_works() -> bool {
    let path = Path::new("/");
    match fs::File::open(path) {
        Ok(file) => fd_is_cloexec(file.as_raw_fd()),
        Err(_) => false,
    }
}

pub fn open_cloexec(path: &Path, flags: i32, mode: mode_t) -> io::Result<fs::File> {
    static CLOEXEC_WORKS: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    let cloexec_works = *CLOEXEC_WORKS.get_or_init(o_cloexec_works);

    let fd = unsafe {
        libc::open(
            path.as_os_str().as_ptr() as *const libc::c_char,
            flags | libc::O_CLOEXEC,
            mode,
        )
    };

    if fd == -1 {
        return Err(io::Error::last_os_error());
    }

    if !cloexec_works {
        let _ = fcntl(fd, FcntlArg::F_SETFD(libc::FD_CLOEXEC));
    }

    Ok(unsafe { fs::File::from_raw_fd(fd) })
}

pub fn fd_leak_check_is_enabled() -> bool {
    env::var_os("GNU_FINDUTILS_FD_LEAK_CHECK").is_some()
}

pub fn complain_about_leaky_fds(prev_fds: &[c_int]) {
    if let Some(leaking_fd) = find_first_leaked_fd(prev_fds) {
        eprintln!(
            "File descriptor {} will leak; please report this as a bug, \
             remembering to include a detailed description of the simplest way \
             to reproduce this problem.",
            leaking_fd
        );
    }
}
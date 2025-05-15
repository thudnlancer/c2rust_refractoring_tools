use std::ffi::{CString, OsStr};
use std::fs::{File, OpenOptions};
use std::io::{self, Error, ErrorKind};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use nix::fcntl::{open, OFlag};
use nix::sys::signal::{self, Signal};
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::{chdir, close, dup2, execvp, fork, ForkResult, Pid};
use libc::{O_RDONLY, O_CLOEXEC};

struct SavedCwd {
    name: Option<PathBuf>,
    desc: RawFd,
}

impl SavedCwd {
    fn new() -> io::Result<Self> {
        let cwd = std::env::current_dir()?;
        let fd = open(&cwd, OFlag::O_RDONLY | OFlag::O_CLOEXEC, nix::sys::stat::Mode::empty())?;
        Ok(Self {
            name: Some(cwd),
            desc: fd,
        })
    }

    fn restore(&self) -> io::Result<()> {
        if self.desc >= 0 {
            chdir(self.desc).map_err(|e| Error::from_raw_os_error(e as i32))?;
        } else if let Some(ref name) = self.name {
            std::env::set_current_dir(name)?;
        }
        Ok(())
    }
}

impl Drop for SavedCwd {
    fn drop(&mut self) {
        if self.desc >= 0 {
            let _ = close(self.desc);
        }
    }
}

struct ExecVal {
    wd_for_exec: Option<SavedCwd>,
    close_stdin: bool,
    last_child_status: Option<ExitStatus>,
}

impl ExecVal {
    fn initialize_wd_for_exec(&mut self, cwd_fd: RawFd, dir: &Path) -> io::Result<()> {
        let fd = openat(cwd_fd, dir, OFlag::O_RDONLY | OFlag::O_CLOEXEC, nix::sys::stat::Mode::empty())?;
        self.wd_for_exec = Some(SavedCwd {
            name: None,
            desc: fd,
        });
        Ok(())
    }

    fn record_exec_dir(&mut self, cwd_fd: RawFd, rel_pathname: &Path) -> io::Result<()> {
        if let Some(parent) = rel_pathname.parent() {
            if !parent.as_os_str().is_empty() {
                self.initialize_wd_for_exec(cwd_fd, parent)
            } else {
                self.initialize_wd_for_exec(cwd_fd, Path::new("."))
            }
        } else {
            self.initialize_wd_for_exec(cwd_fd, Path::new("."))
        }
    }
}

fn prep_child_for_exec(close_stdin: bool, wd: &SavedCwd) -> io::Result<()> {
    if close_stdin {
        let null = File::open("/dev/null")?;
        dup2(null.as_raw_fd(), 0)?;
    }
    wd.restore()
}

fn launch(execp: &mut ExecVal, argv: &[CString]) -> io::Result<bool> {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            let status = waitpid(child, None)?;
            match status {
                WaitStatus::Exited(_, code) => {
                    execp.last_child_status = Some(ExitStatus::from_raw(code));
                    Ok(code == 0)
                }
                WaitStatus::Signaled(_, sig, _) => {
                    eprintln!("Command terminated by signal {}", sig);
                    Ok(false)
                }
                _ => Ok(false),
            }
        }
        Ok(ForkResult::Child) => {
            if let Err(e) = prep_child_for_exec(execp.close_stdin, execp.wd_for_exec.as_ref().unwrap()) {
                std::process::exit(1);
            }
            
            let args: Vec<&CStr> = argv.iter().map(|s| s.as_c_str()).collect();
            if let Err(e) = execvp(&args[0], &args) {
                eprintln!("Failed to execute command: {}", e);
                std::process::exit(1);
            }
            unreachable!();
        }
        Err(e) => Err(Error::from(e)),
    }
}

fn impl_pred_exec(
    pathname: &Path,
    execp: &mut ExecVal,
    is_local: bool,
    cwd_fd: RawFd,
    rel_pathname: &Path,
) -> io::Result<bool> {
    if is_local {
        execp.record_exec_dir(cwd_fd, rel_pathname)?;
    }

    let target = if is_local {
        rel_pathname.file_name().unwrap_or(OsStr::new("."))
    } else {
        pathname.as_os_str()
    };

    // Simplified execution logic - actual implementation would need to handle
    // command building and multiple arguments properly
    let cmd = CString::new("ls").unwrap();
    let arg = CString::new(target.as_bytes()).unwrap();
    let argv = vec![cmd, arg];

    launch(execp, &argv)
}
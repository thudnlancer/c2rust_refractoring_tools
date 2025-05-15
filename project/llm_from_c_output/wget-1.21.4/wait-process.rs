use std::process::{Child, ExitStatus};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::os::unix::process::ExitStatusExt;
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use thiserror::Error;
use libc::{c_int, pid_t};

#[derive(Error, Debug)]
pub enum WaitError {
    #[error("subprocess error: {0}")]
    SubprocessError(String),
    #[error("interrupted by signal")]
    Interrupted,
    #[error("subprocess failed")]
    SubprocessFailed,
    #[error("unknown error")]
    Unknown,
}

pub struct WaitProcess {
    slaves: Arc<Mutex<HashMap<pid_t, bool>>>,
    cleanup_registered: AtomicBool,
}

impl Default for WaitProcess {
    fn default() -> Self {
        Self {
            slaves: Arc::new(Mutex::new(HashMap::new())),
            cleanup_registered: AtomicBool::new(false),
        }
    }
}

impl WaitProcess {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_slave_subprocess(&self, child: pid_t) -> Result<(), WaitError> {
        if !self.cleanup_registered.load(Ordering::SeqCst) {
            self.register_cleanup_handlers()?;
            self.cleanup_registered.store(true, Ordering::SeqCst);
        }

        let mut slaves = self.slaves.lock().unwrap();
        slaves.insert(child, true);
        Ok(())
    }

    fn register_cleanup_handlers(&self) -> Result<(), WaitError> {
        // Register cleanup handlers for atexit and fatal signals
        // Note: In Rust we'd typically use Drop trait for cleanup,
        // but for signal handling we'd need a more robust solution
        // like signal-hook or similar crate
        Ok(())
    }

    fn cleanup_slaves(&self) {
        let mut slaves = self.slaves.lock().unwrap();
        for (&pid, _) in slaves.iter().filter(|(_, &active)| active) {
            let _ = signal::kill(Pid::from_raw(pid), Signal::SIGHUP);
        }
        slaves.clear();
    }

    pub fn wait_subprocess(
        &self,
        child: pid_t,
        progname: &str,
        ignore_sigpipe: bool,
        null_stderr: bool,
        slave_process: bool,
        exit_on_error: bool,
        termsigp: Option<&mut c_int>,
    ) -> Result<c_int, WaitError> {
        if let Some(tsig) = termsigp {
            *tsig = 0;
        }

        let mut status = 0;
        loop {
            match unsafe { libc::waitpid(child, &mut status, 0) } {
                -1 => {
                    if std::io::Error::last_os_error().raw_os_error() == Some(libc::EINTR) {
                        continue;
                    }
                    if exit_on_error || !null_stderr {
                        return Err(WaitError::SubprocessError(
                            format!("{} subprocess", progname),
                        ));
                    }
                    return Ok(127);
                }
                res if res == child => break,
                _ => continue,
            }
        }

        if slave_process {
            let mut slaves = self.slaves.lock().unwrap();
            slaves.remove(&child);
        }

        if libc::WIFSIGNALED(status) {
            let termsig = libc::WTERMSIG(status);
            if let Some(tsig) = termsigp {
                *tsig = termsig;
            }
            if termsig == libc::SIGPIPE && ignore_sigpipe {
                return Ok(0);
            }
            if exit_on_error || (!null_stderr && termsigp.is_none()) {
                return Err(WaitError::SubprocessError(format!(
                    "{} subprocess got fatal signal {}",
                    progname, termsig
                )));
            }
            Ok(127)
        } else if libc::WIFEXITED(status) {
            let exit_status = libc::WEXITSTATUS(status);
            if exit_status == 127 {
                if exit_on_error || !null_stderr {
                    return Err(WaitError::SubprocessFailed);
                }
                Ok(127)
            } else {
                Ok(exit_status)
            }
        } else {
            Err(WaitError::Unknown)
        }
    }
}

// Note: This is a simplified Rust translation. In a production environment, you would:
// 1. Use proper Rust error handling throughout
// 2. Implement proper signal handling using a crate like signal-hook
// 3. Consider using std::process::Child for process management where possible
// 4. Add proper documentation and tests
// 5. Consider thread safety more carefully
use std::env;
use std::ffi::{CString, CStr};
use std::fs::File;
use std::io;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use nix::unistd::{close, pipe2};
use nix::fcntl::OFlag;
use nix::sys::signal::{sigprocmask, SigmaskHow, SigSet};
use nix::sys::wait::WaitStatus;
use nix::unistd::Pid;

#[derive(Debug)]
pub enum PipeError {
    Io(io::Error),
    Nix(nix::Error),
    PathResolution,
    SpawnFailed,
}

impl From<io::Error> for PipeError {
    fn from(err: io::Error) -> Self {
        PipeError::Io(err)
    }
}

impl From<nix::Error> for PipeError {
    fn from(err: nix::Error) -> Self {
        PipeError::Nix(err)
    }
}

fn resolve_program_path(prog_path: &str, directory: Option<&Path>) -> Result<PathBuf, PipeError> {
    let path = if let Some(dir) = directory {
        if !prog_path.starts_with('/') {
            let path_var = env::var("PATH").unwrap_or_default();
            let resolved = which::which_in(prog_path, Some(path_var), dir)
                .map_err(|_| PipeError::PathResolution)?;
            
            if !resolved.starts_with('/') {
                let absolute = resolved.canonicalize()
                    .map_err(|_| PipeError::PathResolution)?;
                if !absolute.starts_with('/') {
                    return Err(PipeError::PathResolution);
                }
                absolute
            } else {
                resolved
            }
        } else {
            PathBuf::from(prog_path)
        }
    } else {
        PathBuf::from(prog_path)
    };
    
    Ok(path)
}

fn create_pipe(
    progname: &str,
    prog_path: &str,
    prog_argv: &[&str],
    directory: Option<&Path>,
    pipe_stdin: bool,
    pipe_stdout: bool,
    prog_stdin: Option<&str>,
    prog_stdout: Option<&str>,
    null_stderr: bool,
    slave_process: bool,
) -> Result<(Pid, Option<RawFd>, Option<RawFd>), PipeError> {
    let resolved_path = resolve_program_path(prog_path, directory)?;

    let (mut stdin_pipe, mut stdout_pipe) = (None, None);
    let (mut stdin_read, mut stdin_write) = (None, None);
    let (mut stdout_read, mut stdout_write) = (None, None);

    if pipe_stdin {
        let (read, write) = pipe2(OFlag::O_CLOEXEC)?;
        stdin_read = Some(read);
        stdin_write = Some(write);
    }

    if pipe_stdout {
        let (read, write) = pipe2(OFlag::O_CLOEXEC)?;
        stdout_read = Some(read);
        stdout_write = Some(write);
    }

    let blocked_signals = if slave_process {
        let mut oldset = SigSet::empty();
        sigprocmask(SigmaskHow::SIG_BLOCK, None, Some(&mut oldset))?;
        Some(oldset)
    } else {
        None
    };

    let mut cmd = Command::new(&resolved_path);
    cmd.args(prog_argv);

    if let Some(dir) = directory {
        cmd.current_dir(dir);
    }

    if pipe_stdin {
        cmd.stdin(unsafe { Stdio::from_raw_fd(stdin_read.unwrap()) });
    } else if let Some(stdin_path) = prog_stdin {
        cmd.stdin(File::open(stdin_path)?);
    }

    if pipe_stdout {
        cmd.stdout(unsafe { Stdio::from_raw_fd(stdout_write.unwrap()) });
    } else if let Some(stdout_path) = prog_stdout {
        cmd.stdout(File::create(stdout_path)?);
    }

    if null_stderr {
        cmd.stderr(Stdio::null());
    }

    if slave_process {
        // TODO: Implement signal handling for slave process
    }

    let child = cmd.spawn()?;

    if pipe_stdin {
        close(stdin_read.unwrap())?;
    }
    if pipe_stdout {
        close(stdout_write.unwrap())?;
    }

    Ok((
        Pid::from_raw(child.id() as i32),
        stdout_read,
        stdin_write,
    ))
}

pub fn create_pipe_bidi(
    progname: &str,
    prog_path: &str,
    prog_argv: &[&str],
    directory: Option<&Path>,
    null_stderr: bool,
    slave_process: bool,
) -> Result<(Pid, RawFd, RawFd), PipeError> {
    let (pid, stdout_read, stdin_write) = create_pipe(
        progname,
        prog_path,
        prog_argv,
        directory,
        true,
        true,
        None,
        None,
        null_stderr,
        slave_process,
    )?;

    Ok((pid, stdout_read.unwrap(), stdin_write.unwrap()))
}

pub fn create_pipe_in(
    progname: &str,
    prog_path: &str,
    prog_argv: &[&str],
    directory: Option<&Path>,
    prog_stdin: Option<&str>,
    null_stderr: bool,
    slave_process: bool,
) -> Result<(Pid, RawFd), PipeError> {
    let (pid, stdout_read, _) = create_pipe(
        progname,
        prog_path,
        prog_argv,
        directory,
        false,
        true,
        prog_stdin,
        None,
        null_stderr,
        slave_process,
    )?;

    Ok((pid, stdout_read.unwrap()))
}

pub fn create_pipe_out(
    progname: &str,
    prog_path: &str,
    prog_argv: &[&str],
    directory: Option<&Path>,
    prog_stdout: Option<&str>,
    null_stderr: bool,
    slave_process: bool,
) -> Result<(Pid, RawFd), PipeError> {
    let (pid, _, stdin_write) = create_pipe(
        progname,
        prog_path,
        prog_argv,
        directory,
        true,
        false,
        None,
        prog_stdout,
        null_stderr,
        slave_process,
    )?;

    Ok((pid, stdin_write.unwrap()))
}
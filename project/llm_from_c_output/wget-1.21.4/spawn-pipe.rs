use std::{
    ffi::{CString, OsStr},
    fs::{File, OpenOptions},
    io,
    os::unix::io::{AsRawFd, FromRawFd, RawFd},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[derive(Debug)]
pub enum SpawnError {
    Io(io::Error),
    InvalidPath,
    InvalidArgs,
    SpawnFailed,
}

pub type Pid = i32;

pub fn create_pipe_out(
    progname: &str,
    prog_path: &str,
    prog_argv: &[&str],
    directory: Option<&Path>,
    prog_stdout: Option<&str>,
    null_stderr: bool,
    slave_process: bool,
    exit_on_error: bool,
) -> Result<(Pid, File), SpawnError> {
    let stdin_pipe = true;
    let stdout_pipe = false;

    let (child, pipes) = create_pipe_internal(
        progname,
        prog_path,
        prog_argv,
        directory,
        stdin_pipe,
        stdout_pipe,
        None,
        prog_stdout,
        null_stderr,
        slave_process,
        exit_on_error,
    )?;

    Ok((child, pipes.1.unwrap()))
}

pub fn create_pipe_in(
    progname: &str,
    prog_path: &str,
    prog_argv: &[&str],
    directory: Option<&Path>,
    prog_stdin: Option<&str>,
    null_stderr: bool,
    slave_process: bool,
    exit_on_error: bool,
) -> Result<(Pid, File), SpawnError> {
    let stdin_pipe = false;
    let stdout_pipe = true;

    let (child, pipes) = create_pipe_internal(
        progname,
        prog_path,
        prog_argv,
        directory,
        stdin_pipe,
        stdout_pipe,
        prog_stdin,
        None,
        null_stderr,
        slave_process,
        exit_on_error,
    )?;

    Ok((child, pipes.0.unwrap()))
}

pub fn create_pipe_bidi(
    progname: &str,
    prog_path: &str,
    prog_argv: &[&str],
    directory: Option<&Path>,
    null_stderr: bool,
    slave_process: bool,
    exit_on_error: bool,
) -> Result<(Pid, File, File), SpawnError> {
    let stdin_pipe = true;
    let stdout_pipe = true;

    let (child, pipes) = create_pipe_internal(
        progname,
        prog_path,
        prog_argv,
        directory,
        stdin_pipe,
        stdout_pipe,
        None,
        None,
        null_stderr,
        slave_process,
        exit_on_error,
    )?;

    Ok((child, pipes.0.unwrap(), pipes.1.unwrap()))
}

fn create_pipe_internal(
    progname: &str,
    prog_path: &str,
    prog_argv: &[&str],
    directory: Option<&Path>,
    pipe_stdin: bool,
    pipe_stdout: bool,
    prog_stdin: Option<&str>,
    prog_stdout: Option<&str>,
    null_stderr: bool,
    _slave_process: bool,
    _exit_on_error: bool,
) -> Result<(Pid, (Option<File>, Option<File>)), SpawnError> {
    let mut command = Command::new(prog_path);

    // Set up stdin
    let stdin_pipe = if pipe_stdin {
        let (reader, writer) = os_pipe::pipe()?;
        command.stdin(unsafe { Stdio::from_raw_fd(writer.as_raw_fd()) });
        Some(reader)
    } else {
        if let Some(stdin_path) = prog_stdin {
            command.stdin(File::open(stdin_path)?);
        } else {
            command.stdin(Stdio::inherit());
        }
        None
    };

    // Set up stdout
    let stdout_pipe = if pipe_stdout {
        let (reader, writer) = os_pipe::pipe()?;
        command.stdout(unsafe { Stdio::from_raw_fd(reader.as_raw_fd()) });
        Some(writer)
    } else {
        if let Some(stdout_path) = prog_stdout {
            command.stdout(File::create(stdout_path)?);
        } else {
            command.stdout(Stdio::inherit());
        }
        None
    };

    // Set up stderr
    if null_stderr {
        command.stderr(Stdio::null());
    } else {
        command.stderr(Stdio::inherit());
    }

    // Set working directory if specified
    if let Some(dir) = directory {
        command.current_dir(dir);
    }

    // Set arguments
    command.args(prog_argv);

    // Spawn the child process
    let child = command.spawn()?;

    Ok((child.id() as Pid, (stdin_pipe, stdout_pipe)))
}

// Helper trait to convert io::Error to SpawnError
trait ToSpawnError<T> {
    fn map_spawn_err(self) -> Result<T, SpawnError>;
}

impl<T> ToSpawnError<T> for Result<T, io::Error> {
    fn map_spawn_err(self) -> Result<T, SpawnError> {
        self.map_err(SpawnError::Io)
    }
}

impl From<os_pipe::Error> for SpawnError {
    fn from(err: os_pipe::Error) -> Self {
        SpawnError::Io(err.into())
    }
}
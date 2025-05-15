use std::ffi::CString;
use std::io::{self, Write};
use std::process::{Command, Stdio};

pub struct Printer {
    child: std::process::Child,
}

impl Printer {
    pub fn open(
        cmd: &str,
        options: Option<&str>,
        queue_param: Option<&str>,
        printer_name: Option<&str>,
    ) -> io::Result<Self> {
        let mut command = Command::new(cmd);

        if let Some(opts) = options {
            command.arg(opts);
        }

        if let (Some(param), Some(name)) = (queue_param, printer_name) {
            command.arg(param).arg(name);
        }

        let child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        Ok(Printer { child })
    }

    pub fn stdin(&mut self) -> Option<&mut std::process::ChildStdin> {
        self.child.stdin.as_mut()
    }
}

impl Drop for Printer {
    fn drop(&mut self) {
        let _ = self.child.wait();
    }
}

pub fn printer_open(
    cmd: &str,
    options: Option<&str>,
    queue_param: Option<&str>,
    printer_name: Option<&str>,
) -> io::Result<Printer> {
    Printer::open(cmd, options, queue_param, printer_name)
}

pub fn printer_close(_printer: Printer) {
    // Drop is handled automatically
}
use std::ffi::{CString, CStr};
use std::io::{self, Read, Write};
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
use std::process::{Command, Stdio};
use std::ptr;

fn safe_popen_out(command: &[&CStr], output: &mut [u8]) -> io::Result<usize> {
    let mut child = Command::new(command[0].to_str()?)
        .args(&command[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;

    let mut stdout = child.stdout.take().unwrap();
    let bytes_read = stdout.read(output)?;

    child.kill()?;
    child.wait()?;

    Ok(bytes_read)
}

fn expand(input: &CStr, ans: &mut [u8]) -> io::Result<&CStr> {
    const MAX_SIZE: usize = 2048 - 1;
    if input.is_empty() {
        return Ok(CStr::from_bytes_with_nul(b"\0").unwrap());
    }

    let input_str = input.to_str()?;
    if !input_str.chars().any(|c| "$*(){}[]\\?`~".contains(c)) {
        let len = input_str.len().min(MAX_SIZE);
        ans[..len].copy_from_slice(&input.to_bytes()[..len]);
        ans[len] = 0;
        return unsafe { Ok(CStr::from_ptr(ans.as_ptr() as *const i8)) };
    }

    let echo_cmd = CString::new(format!("echo {}", input_str))?;
    let command = [
        CString::new("/bin/sh")?.as_c_str(),
        CString::new("sh")?.as_c_str(),
        CString::new("-c")?.as_c_str(),
        echo_cmd.as_c_str(),
    ];

    let bytes_read = safe_popen_out(&command, ans)?;
    if bytes_read > 0 {
        ans[bytes_read - 1] = 0;
    } else {
        let len = input_str.len().min(MAX_SIZE);
        ans[..len].copy_from_slice(&input.to_bytes()[..len]);
        ans[len] = 0;
    }

    unsafe { Ok(CStr::from_ptr(ans.as_ptr() as *const i8)) }
}
use std::process::{Command, Stdio};
use std::io::Read;
use std::ffi::OsStr;
use std::os::unix::process::CommandExt;
use std::borrow::Cow;

const EXPAND_BUF: usize = 256;

fn safe_popen_out(command: &[&str], output: &mut [u8]) -> Result<usize, std::io::Error> {
    let mut child = Command::new(command[0])
        .args(&command[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;

    let mut stdout = child.stdout.take().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to capture stdout")
    })?;

    let bytes_read = stdout.read(output)?;
    
    child.kill()?;
    child.wait()?;
    
    Ok(bytes_read)
}

pub fn expand(input: &str, ans: &mut [u8]) -> Cow<str> {
    if input.is_empty() {
        return Cow::Borrowed("");
    }

    // Check if there's anything to expand
    if !input.contains(|c: char| 
        matches!(c, '$' | '*' | '(' | ')' | '{' | '}' | '[' | ']' | '\\' | '?' | '`' | '~')) 
    {
        let len = input.len().min(EXPAND_BUF - 1);
        ans[..len].copy_from_slice(&input.as_bytes()[..len]);
        ans[len] = b'\0';
        return Cow::Borrowed(input);
    }

    let command = ["/bin/sh", "sh", "-c", &format!("echo {}", input)];
    
    match safe_popen_out(&command, ans) {
        Ok(last) => {
            if last > 0 {
                ans[last - 1] = b'\0';
                Cow::Borrowed(std::str::from_utf8(&ans[..last - 1]).unwrap_or("")
            } else {
                let len = input.len().min(EXPAND_BUF - 1);
                ans[..len].copy_from_slice(&input.as_bytes()[..len]);
                ans[len] = b'\0';
                Cow::Borrowed(input)
            }
        }
        Err(_) => {
            let len = input.len().min(EXPAND_BUF - 1);
            ans[..len].copy_from_slice(&input.as_bytes()[..len]);
            ans[len] = b'\0';
            Cow::Borrowed(input)
        }
    }
}

#[cfg(target_os = "windows")]
pub fn expand(input: &str, ans: &mut [u8]) -> Cow<str> {
    let len = input.len().min(EXPAND_BUF - 1);
    ans[..len].copy_from_slice(&input.as_bytes()[..len]);
    ans[len] = b'\0';
    Cow::Borrowed(input)
}
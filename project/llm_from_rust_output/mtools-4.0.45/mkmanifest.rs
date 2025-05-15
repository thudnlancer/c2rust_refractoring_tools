use std::env;
use std::ffi::{CString, CStr, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::io::{self, Write};

const DEVICES: [&str; 9] = ["con", "aux", "com1", "com2", "lpt1", "prn", "lpt2", "lpt3", "nul"];
const INVALID_CHARS: &str = "^+=/[]:',?*\\<>|\". ";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() >= 2 && args[1] == "-V" {
        println!("Mtools version {}, dated {}", 
            unsafe { CStr::from_ptr(mversion).to_string_lossy() },
            unsafe { CStr::from_ptr(mdate).to_string_lossy() });
        return Ok(());
    }

    if args.len() == 1 {
        writeln!(io::stderr(), "Usage: mkmanifest [-V] <list-of-files>")?;
        return Ok(());
    }

    for arg in args.iter().skip(1) {
        let path = Path::new(arg);
        let name = path.file_name()
            .unwrap_or_else(|| OsStr::new(""))
            .to_string_lossy();
        
        let new_name = dos_name2(&name);
        if new_name.to_lowercase() != name.to_lowercase() {
            println!("mv {} {}", new_name, name);
        }
    }

    Ok(())
}

fn dos_name2(name: &str) -> String {
    let (mut base, ext) = match name.rsplit_once('.') {
        Some((b, e)) => (b.to_string(), Some(e)),
        None => (name.to_string(), None),
    };

    // Convert to lowercase
    base = base.to_lowercase();
    
    // Check for reserved device names
    if DEVICES.contains(&base.as_str()) {
        base = "x".to_string();
    }

    // Truncate base to 8 characters
    if base.len() > 8 {
        base.truncate(8);
    }

    // Replace invalid characters
    base = base.chars()
        .map(|c| if INVALID_CHARS.contains(c) { 'x' } else { c })
        .collect();

    // Process extension if exists
    let mut result = base;
    if let Some(ext) = ext {
        let mut ext = ext.to_lowercase();
        
        // Truncate extension to 3 characters
        if ext.len() > 3 {
            ext.truncate(3);
        }

        // Replace invalid characters in extension
        ext = ext.chars()
            .map(|c| if INVALID_CHARS.contains(c) { 'x' } else { c })
            .collect();

        if !ext.is_empty() {
            result.push('.');
            result.push_str(&ext);
        }
    }

    if result.is_empty() {
        result = "x".to_string();
    }

    result
}

// These would normally come from external C libraries
static mversion: *const libc::c_char = b"unknown\0".as_ptr() as *const _;
static mdate: *const libc::c_char = b"unknown\0".as_ptr() as *const _;
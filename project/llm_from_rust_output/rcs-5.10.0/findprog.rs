use std::env;
use std::ffi::{CString, OsStr, OsString};
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Stat {
    pub st_mode: u32,
    // Other fields omitted for brevity as they're not used in the logic
}

pub fn find_in_path(progname: &str) -> PathBuf {
    if progname.contains('/') {
        return PathBuf::from(progname);
    }

    let path = match env::var_os("PATH") {
        Some(p) => p,
        None => return PathBuf::from(progname),
    };

    if path.is_empty() {
        return PathBuf::from(progname);
    }

    for dir in env::split_paths(&path) {
        let dir = if dir.as_os_str().is_empty() {
            Path::new(".")
        } else {
            &dir
        };

        let progpath = dir.join(progname);
        if let Ok(metadata) = fs::metadata(&progpath) {
            if !metadata.is_dir() && metadata.permissions().readonly() {
                if progpath == Path::new(progname) {
                    return Path::new("./").join(progname);
                }
                return progpath;
            }
        }
    }

    PathBuf::from(progname)
}

fn main() {
    // Example usage
    let progname = "ls";
    let path = find_in_path(progname);
    println!("Found at: {:?}", path);
}
use std::ffi::{CString, CStr};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::ptr;

static SUFFIXES: &[&str] = &[""];

pub fn find_in_given_path(
    progname: &str,
    path: Option<&str>,
    directory: Option<&str>,
    optimize_for_exec: bool,
) -> Option<PathBuf> {
    if progname.contains('/') {
        if optimize_for_exec {
            return Some(PathBuf::from(progname));
        } else {
            let directory_as_prefix = if directory.is_some() && !progname.starts_with('/') {
                directory
            } else {
                None
            };

            let mut failure_errno = 2;
            for suffix in SUFFIXES {
                let progpathname = concatenated_filename(directory_as_prefix, progname, suffix)?;
                
                if fs::metadata(&progpathname).map(|m| m.mode() & 0o170000 != 0o40000).unwrap_or(false) {
                    if progpathname == PathBuf::from(progname) {
                        return Some(PathBuf::from(progname));
                    } else {
                        return Some(progpathname);
                    }
                } else {
                    let err = std::io::Error::last_os_error();
                    if err.raw_os_error().unwrap_or(0) != 2 {
                        failure_errno = err.raw_os_error().unwrap_or(2);
                    }
                }
            }
            std::io::Error::from_raw_os_error(failure_errno).ok()?;
            return None;
        }
    }

    let path = path.unwrap_or("");
    let path_copy = path.to_string();
    let mut failure_errno = 2;
    let mut path_rest = path_copy.as_str();

    while !path_rest.is_empty() {
        let (dir, remaining) = match path_rest.find(':') {
            Some(pos) => (&path_rest[..pos], &path_rest[pos+1..]),
            None => (path_rest, ""),
        };

        let dir = if dir.is_empty() { "." } else { dir };
        let dir_as_prefix = if directory.is_some() && !dir.starts_with('/') {
            concatenated_filename(directory, dir, "")?
        } else {
            PathBuf::from(dir)
        };

        for suffix in SUFFIXES {
            let progpathname = concatenated_filename(Some(dir_as_prefix.as_path()), progname, suffix)?;
            
            if fs::metadata(&progpathname).map(|m| m.mode() & 0o170000 != 0o40000).unwrap_or(false) {
                if progpathname == PathBuf::from(progname) {
                    let mut new_path = PathBuf::new();
                    new_path.push(".");
                    new_path.push("/");
                    new_path.push(progname);
                    return Some(new_path);
                } else {
                    return Some(progpathname);
                }
            } else {
                let err = std::io::Error::last_os_error();
                if err.raw_os_error().unwrap_or(0) != 2 {
                    failure_errno = err.raw_os_error().unwrap_or(2);
                }
            }
        }

        path_rest = remaining;
    }

    std::io::Error::from_raw_os_error(failure_errno).ok()?;
    None
}

fn concatenated_filename(
    directory: Option<&Path>,
    filename: &str,
    suffix: &str,
) -> Option<PathBuf> {
    let mut path = if let Some(dir) = directory {
        dir.to_path_buf()
    } else {
        PathBuf::new()
    };
    path.push(filename);
    path.set_extension(suffix);
    Some(path)
}
use std::ffi::{CStr, CString};
use std::fs;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr;

bitflags::bitflags! {
    pub struct CanonicalizeMode: u32 {
        const EXISTING = 0;
        const ALL_BUT_LAST = 1;
        const MISSING = 2;
        const NOLINKS = 4;
    }
}

const CAN_MODE_MASK: u32 = CanonicalizeMode::EXISTING.bits | CanonicalizeMode::ALL_BUT_LAST.bits | CanonicalizeMode::MISSING.bits;

pub fn canonicalize_filename_mode(name: &str, can_mode: CanonicalizeMode) -> io::Result<PathBuf> {
    if name.is_empty() {
        return Err(io::Error::from_raw_os_error(libc::ENOENT));
    }

    let path = Path::new(name);
    if path.is_absolute() {
        canonicalize_absolute(path, can_mode)
    } else {
        let cwd = std::env::current_dir()?;
        canonicalize_relative(&cwd, path, can_mode)
    }
}

fn canonicalize_absolute(path: &Path, can_mode: CanonicalizeMode) -> io::Result<PathBuf> {
    let mut components = path.components();
    let mut result = PathBuf::new();
    
    for component in components {
        match component {
            std::path::Component::Prefix(prefix) => {
                result.push(prefix.as_os_str());
            }
            std::path::Component::RootDir => {
                result.push("/");
            }
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                if !result.pop() {
                    return Err(io::Error::from_raw_os_error(libc::ELOOP));
                }
            }
            std::path::Component::Normal(name) => {
                result.push(name);
                if !can_mode.contains(CanonicalizeMode::NOLINKS) {
                    if let Ok(link) = fs::read_link(&result) {
                        result.pop();
                        if link.is_absolute() {
                            result = canonicalize_absolute(&link, can_mode)?;
                        } else {
                            result = canonicalize_relative(&result, &link, can_mode)?;
                        }
                    }
                }
                
                let exists = result.exists();
                if !exists && !can_mode.contains(CanonicalizeMode::MISSING) {
                    return Err(io::Error::from_raw_os_error(libc::ENOENT));
                }
            }
        }
    }
    
    Ok(result)
}

fn canonicalize_relative(base: &Path, path: &Path, can_mode: CanonicalizeMode) -> io::Result<PathBuf> {
    let mut result = base.to_path_buf();
    
    for component in path.components() {
        match component {
            std::path::Component::Prefix(_) => unreachable!("Relative path shouldn't have prefix"),
            std::path::Component::RootDir => unreachable!("Relative path shouldn't have root"),
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                if !result.pop() {
                    return Err(io::Error::from_raw_os_error(libc::ELOOP));
                }
            }
            std::path::Component::Normal(name) => {
                result.push(name);
                if !can_mode.contains(CanonicalizeMode::NOLINKS) {
                    if let Ok(link) = fs::read_link(&result) {
                        result.pop();
                        if link.is_absolute() {
                            result = canonicalize_absolute(&link, can_mode)?;
                        } else {
                            result = canonicalize_relative(&result, &link, can_mode)?;
                        }
                    }
                }
                
                let exists = result.exists();
                if !exists && !can_mode.contains(CanonicalizeMode::MISSING) {
                    return Err(io::Error::from_raw_os_error(libc::ENOENT));
                }
            }
        }
    }
    
    Ok(result)
}

pub fn canonicalize_file_name(name: &str) -> io::Result<PathBuf> {
    canonicalize_filename_mode(name, CanonicalizeMode::EXISTING)
}
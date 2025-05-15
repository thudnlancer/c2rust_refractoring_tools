use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::os::unix::fs::MetadataExt;

const NATIVE_SLASH: char = if cfg!(windows) { '\\' } else { '/' };
const PATH_SEPARATOR: char = if cfg!(windows) { ';' } else { ':' };

#[cfg(windows)]
const SUFFIXES: &[&str] = &["", ".com", ".exe", ".bat", ".cmd"];
#[cfg(target_os = "cygwin")]
const SUFFIXES: &[&str] = &["", ".exe", ".com"];
#[cfg(target_os = "emx")]
const SUFFIXES: &[&str] = &["", ".exe"];
#[cfg(target_os = "dragonfly")]
const SUFFIXES: &[&str] = &["", ".com", ".exe", ".bat"];
#[cfg(not(any(windows, target_os = "cygwin", target_os = "emx", target_os = "dragonfly")))]
const SUFFIXES: &[&str] = &[""];

pub fn find_in_given_path(
    progname: &str,
    path: Option<&str>,
    directory: Option<&str>,
    optimize_for_exec: bool,
) -> io::Result<PathBuf> {
    if progname.contains(NATIVE_SLASH) {
        if optimize_for_exec {
            return Ok(PathBuf::from(progname));
        } else {
            let directory_as_prefix = if let Some(dir) = directory {
                if is_relative_path(progname) {
                    dir
                } else {
                    ""
                }
            } else {
                ""
            };

            #[cfg(windows)]
            let progname_has_dot = progname.contains('.');

            let mut failure_errno = io::ErrorKind::NotFound;
            for suffix in SUFFIXES {
                #[cfg(windows)]
                if (!suffix.is_empty()) != progname_has_dot {
                    continue;
                }

                let progpathname = concatenated_filename(directory_as_prefix, progname, suffix)?;
                match fs::metadata(&progpathname) {
                    Ok(metadata) => {
                        if metadata.is_file() && is_executable(&progpathname)? {
                            if progpathname == PathBuf::from(progname) {
                                return Ok(PathBuf::from(progname));
                            } else {
                                return Ok(progpathname);
                            }
                        } else {
                            failure_errno = io::ErrorKind::PermissionDenied;
                        }
                    }
                    Err(e) => {
                        if e.kind() != io::ErrorKind::NotFound {
                            failure_errno = e.kind();
                        }
                    }
                }
            }

            #[cfg(windows)]
            if failure_errno == io::ErrorKind::NotFound && !progname_has_dot {
                let progpathname = concatenated_filename(directory_as_prefix, progname, "")?;
                match fs::metadata(&progpathname) {
                    Ok(metadata) => {
                        if metadata.is_file() {
                            failure_errno = io::ErrorKind::InvalidInput;
                        } else {
                            failure_errno = io::ErrorKind::PermissionDenied;
                        }
                    }
                    Err(e) => {
                        failure_errno = e.kind();
                    }
                }
            }

            return Err(io::Error::new(failure_errno, "Program not found"));
        }
    }

    let path = path.unwrap_or("");
    let path_copy = path.to_string();
    let mut failure_errno = io::ErrorKind::NotFound;

    #[cfg(windows)]
    let progname_has_dot = progname.contains('.');

    for dir in path_copy.split(PATH_SEPARATOR) {
        let dir = if dir.is_empty() { "." } else { dir };
        let dir_as_prefix = if let Some(directory) = directory {
            if is_relative_path(dir) {
                concatenated_filename(directory, dir, None)?
            } else {
                PathBuf::from(dir)
            }
        } else {
            PathBuf::from(dir)
        };

        for suffix in SUFFIXES {
            #[cfg(windows)]
            if (!suffix.is_empty()) != progname_has_dot {
                continue;
            }

            let progpathname = concatenated_filename(&dir_as_prefix, progname, suffix)?;
            match fs::metadata(&progpathname) {
                Ok(metadata) => {
                    if metadata.is_file() && is_executable(&progpathname)? {
                        if progpathname == PathBuf::from(progname) {
                            let mut new_path = PathBuf::new();
                            new_path.push(".");
                            new_path.push(NATIVE_SLASH.to_string());
                            new_path.push(progname);
                            return Ok(new_path);
                        } else {
                            return Ok(progpathname);
                        }
                    } else {
                        failure_errno = io::ErrorKind::PermissionDenied;
                    }
                }
                Err(e) => {
                    if e.kind() != io::ErrorKind::NotFound {
                        failure_errno = e.kind();
                    }
                }
            }
        }

        #[cfg(windows)]
        if failure_errno == io::ErrorKind::NotFound && !progname_has_dot {
            let progpathname = concatenated_filename(&dir_as_prefix, progname, "")?;
            match fs::metadata(&progpathname) {
                Ok(metadata) => {
                    if metadata.is_file() {
                        failure_errno = io::ErrorKind::InvalidInput;
                    } else {
                        failure_errno = io::ErrorKind::PermissionDenied;
                    }
                }
                Err(e) => {
                    failure_errno = e.kind();
                }
            }
        }
    }

    Err(io::Error::new(failure_errno, "Program not found in PATH"))
}

fn is_relative_path(path: &str) -> bool {
    !Path::new(path).is_absolute()
}

fn concatenated_filename(
    prefix: impl AsRef<Path>,
    name: &str,
    suffix: Option<&str>,
) -> io::Result<PathBuf> {
    let mut path = PathBuf::from(prefix.as_ref());
    path.push(name);
    if let Some(suffix) = suffix {
        path.set_extension(suffix.trim_start_matches('.'));
    }
    Ok(path)
}

#[cfg(unix)]
fn is_executable(path: &Path) -> io::Result<bool> {
    use std::os::unix::fs::PermissionsExt;
    let metadata = fs::metadata(path)?;
    Ok(metadata.permissions().mode() & 0o111 != 0)
}

#[cfg(windows)]
fn is_executable(path: &Path) -> io::Result<bool> {
    let metadata = fs::metadata(path)?;
    Ok(!metadata.is_dir())
}
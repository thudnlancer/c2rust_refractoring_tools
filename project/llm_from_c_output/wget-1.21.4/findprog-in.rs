use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::os::unix::fs::PermissionsExt;

const NATIVE_SLASH: char = '/';
const PATH_SEPARATOR: char = ':';

#[cfg(target_os = "windows")]
const SUFFIXES: &[&str] = &["", ".com", ".exe", ".bat", ".cmd"];
#[cfg(target_os = "cygwin")]
const SUFFIXES: &[&str] = &["", ".exe", ".com"];
#[cfg(target_os = "emscripten")]
const SUFFIXES: &[&str] = &["", ".exe"];
#[cfg(target_os = "dragonfly")]
const SUFFIXES: &[&str] = &["", ".com", ".exe", ".bat"];
#[cfg(not(any(
    target_os = "windows",
    target_os = "cygwin",
    target_os = "emscripten",
    target_os = "dragonfly"
)))]
const SUFFIXES: &[&str] = &[""];

fn has_slash(progname: &str) -> bool {
    progname.contains(NATIVE_SLASH)
}

fn is_relative_file_name(path: &str) -> bool {
    !Path::new(path).is_absolute()
}

fn concatenated_filename(prefix: &str, name: &str, suffix: &str) -> io::Result<PathBuf> {
    let mut path = PathBuf::from(prefix);
    path.push(name);
    path.set_extension(suffix.trim_start_matches('.'));
    Ok(path)
}

fn is_executable(path: &Path) -> io::Result<bool> {
    let metadata = fs::metadata(path)?;
    if metadata.is_dir() {
        return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Is a directory"));
    }
    Ok(metadata.permissions().mode() & 0o111 != 0)
}

fn find_in_given_path(
    progname: &str,
    path: Option<&str>,
    directory: Option<&str>,
    optimize_for_exec: bool,
) -> io::Result<PathBuf> {
    if has_slash(progname) {
        if optimize_for_exec {
            return Ok(PathBuf::from(progname));
        } else {
            let directory_as_prefix = if directory.is_some() && is_relative_file_name(progname) {
                directory.unwrap()
            } else {
                ""
            };

            #[cfg(target_os = "windows")]
            let progname_has_dot = progname.contains('.');

            let mut failure_errno = io::ErrorKind::NotFound;
            for suffix in SUFFIXES {
                #[cfg(target_os = "windows")]
                if (!suffix.is_empty()) != progname_has_dot {
                    continue;
                }

                let progpathname = concatenated_filename(directory_as_prefix, progname, suffix)?;
                match is_executable(&progpathname) {
                    Ok(true) => {
                        if progpathname == PathBuf::from(progname) {
                            return Ok(PathBuf::from(progname));
                        } else {
                            return Ok(progpathname);
                        }
                    }
                    Err(e) if e.kind() != io::ErrorKind::NotFound => {
                        failure_errno = e.kind();
                    }
                    _ => {}
                }
            }

            #[cfg(target_os = "windows")]
            if failure_errno == io::ErrorKind::NotFound && !progname_has_dot {
                let progpathname = concatenated_filename(directory_as_prefix, progname, "")?;
                match is_executable(&progpathname) {
                    Ok(true) => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "Exec format error",
                        ));
                    }
                    Err(e) => {
                        failure_errno = e.kind();
                    }
                    _ => {}
                }
            }

            return Err(io::Error::new(failure_errno, ""));
        }
    }

    let path = path.unwrap_or("");
    let path_copy = env::split_paths(path).collect::<Vec<_>>();

    #[cfg(target_os = "windows")]
    let progname_has_dot = progname.contains('.');

    let mut failure_errno = io::ErrorKind::NotFound;
    for dir in path_copy {
        let dir = if dir.as_os_str().is_empty() {
            PathBuf::from(".")
        } else {
            dir
        };

        let dir_as_prefix = if let Some(directory) = directory {
            if is_relative_file_name(dir.to_str().unwrap()) {
                concatenated_filename(directory, dir.to_str().unwrap(), "")?
            } else {
                dir
            }
        } else {
            dir
        };

        for suffix in SUFFIXES {
            #[cfg(target_os = "windows")]
            if (!suffix.is_empty()) != progname_has_dot {
                continue;
            }

            let progpathname = concatenated_filename(
                dir_as_prefix.to_str().unwrap(),
                progname,
                suffix,
            )?;
            match is_executable(&progpathname) {
                Ok(true) => {
                    if progpathname == PathBuf::from(progname) {
                        let mut new_path = PathBuf::new();
                        new_path.push(".");
                        new_path.push(NATIVE_SLASH.to_string());
                        new_path.push(progname);
                        return Ok(new_path);
                    } else {
                        return Ok(progpathname);
                    }
                }
                Err(e) if e.kind() != io::ErrorKind::NotFound => {
                    failure_errno = e.kind();
                }
                _ => {}
            }
        }

        #[cfg(target_os = "windows")]
        if failure_errno == io::ErrorKind::NotFound && !progname_has_dot {
            let progpathname = concatenated_filename(
                dir_as_prefix.to_str().unwrap(),
                progname,
                "",
            )?;
            match is_executable(&progpathname) {
                Ok(true) => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Exec format error",
                    ));
                }
                Err(e) => {
                    failure_errno = e.kind();
                }
                _ => {}
            }
        }
    }

    Err(io::Error::new(failure_errno, ""))
}
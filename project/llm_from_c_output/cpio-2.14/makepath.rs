use std::fs::{self, DirBuilder, Metadata};
use std::os::unix::fs::{DirBuilderExt, MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::io::{Error, ErrorKind};
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;

/// Ensure that the directory `path` exists.
/// Remove any trailing slashes from `path` before calling this function.
///
/// Make all directory components that don't already exist with
/// permissions 700.
/// If `owner` and `group` are non-negative, make them the UID and GID of
/// created directories.
/// If `verbose_fmt_string` is Some, use it as a format string for printing
/// a message after successfully making a directory.
///
/// Return Ok(()) if `path` exists as a directory with the proper
/// ownership and permissions when done, otherwise Err.
pub fn make_path(
    path: &str,
    owner: Option<u32>,
    group: Option<u32>,
    verbose_fmt_string: Option<&str>,
) -> Result<(), Error> {
    let path = Path::new(path);
    let mut dirpath = PathBuf::from(path);
    let we_are_root = unsafe { libc::getuid() == 0 };
    let tmpmode = 0o700 & !newdir_umask();
    let invert_permissions = if we_are_root { 0 } else { 0o700 & !tmpmode };

    match fs::metadata(path) {
        Ok(metadata) => {
            if !metadata.is_dir() {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("'{}' exists but is not a directory", path.display()),
                ));
            }
            Ok(())
        }
        Err(_) => {
            let mut components = path.components();
            let mut current_path = PathBuf::new();

            // Skip root component if present
            if path.has_root() {
                if let Some(comp) = components.next() {
                    current_path.push(comp);
                }
            }

            for component in components {
                current_path.push(component);
                if let Err(e) = fs::metadata(&current_path) {
                    if e.kind() == ErrorKind::NotFound {
                        let mut builder = DirBuilder::new();
                        builder.mode(tmpmode ^ invert_permissions);
                        builder.create(&current_path).map_err(|e| {
                            Error::new(
                                e.kind(),
                                format!("cannot make directory '{}'", current_path.display()),
                            )
                        })?;

                        if let Some(fmt) = verbose_fmt_string {
                            println!(fmt, current_path.display());
                        }

                        let mut metadata = fs::metadata(&current_path)?;
                        if let Some(uid) = owner {
                            metadata.st_uid = uid;
                        }
                        if let Some(gid) = group {
                            metadata.st_gid = gid;
                        }

                        // TODO: Implement delay_set_stat equivalent
                        // This would require maintaining state for later chmod/chown calls
                    } else {
                        return Err(e);
                    }
                } else {
                    let metadata = fs::metadata(&current_path)?;
                    if !metadata.is_dir() {
                        return Err(Error::new(
                            ErrorKind::Other,
                            format!(
                                "'{}' exists but is not a directory",
                                current_path.display()
                            ),
                        ));
                    }
                }
            }

            // Create final component
            let mut builder = DirBuilder::new();
            builder.mode(tmpmode ^ invert_permissions);
            match builder.create(path) {
                Ok(_) => {
                    if let Some(fmt) = verbose_fmt_string {
                        println!(fmt, path.display());
                    }

                    let mut metadata = fs::metadata(path)?;
                    if let Some(uid) = owner {
                        metadata.st_uid = uid;
                    }
                    if let Some(gid) = group {
                        metadata.st_gid = gid;
                    }

                    // TODO: Implement delay_set_stat equivalent
                    Ok(())
                }
                Err(e) if e.kind() == ErrorKind::AlreadyExists => {
                    let metadata = fs::metadata(path)?;
                    if !metadata.is_dir() {
                        Err(Error::new(
                            ErrorKind::Other,
                            format!("'{}' exists but is not a directory", path.display()),
                        ))
                    } else {
                        Ok(())
                    }
                }
                Err(e) => Err(Error::new(
                    e.kind(),
                    format!("cannot make directory '{}'", path.display()),
                )),
            }
        }
    }
}

fn newdir_umask() -> u32 {
    unsafe { libc::umask(0) as u32 }
}

// TODO: Implement delay_set_stat functionality
// This would require maintaining state for later chmod/chown calls
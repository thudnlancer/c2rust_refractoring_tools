use std::ffi::{CStr, CString, OsStr};
use std::fs;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalizeMode {
    Existing = 0,
    AllButLast = 1,
    Missing = 2,
    NoLinks = 4,
}

const CAN_MODE_MASK: u32 = 0b111;

pub fn canonicalize_filename_mode(
    name: &str,
    can_mode: CanonicalizeMode,
) -> io::Result<PathBuf> {
    let mut path = PathBuf::new();
    let mut components = Path::new(name).components();
    let mut num_links = 0;
    let mut seen_links = std::collections::HashSet::new();

    let can_exist = (can_mode as u32) & CAN_MODE_MASK;
    if can_exist.count_ones() > 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid canonicalize mode",
        ));
    }

    if name.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Empty path"));
    }

    if !Path::new(name).is_absolute() {
        path = std::env::current_dir()?;
    }

    for component in components {
        match component {
            Component::Prefix(_) => {}
            Component::RootDir => {
                path.push("/");
            }
            Component::CurDir => {}
            Component::ParentDir => {
                if !path.pop() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Path climbs above root",
                    ));
                }
            }
            Component::Normal(part) => {
                let part_str = part.to_str().ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Invalid UTF-8 in path",
                    )
                })?;

                path.push(part_str);

                if (can_mode as u32 & CanonicalizeMode::NoLinks as u32) == 0 {
                    if num_links >= 20 {
                        let parent_metadata = path.parent().map_or_else(
                            || std::fs::metadata("."),
                            |p| std::fs::metadata(p),
                        )?;

                        let key = (
                            part_str.to_owned(),
                            parent_metadata.dev(),
                            parent_metadata.ino(),
                        );

                        if !seen_links.insert(key) {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidInput,
                                "Symlink loop detected",
                            ));
                        }
                    }

                    if let Ok(link_target) = fs::read_link(&path) {
                        num_links += 1;
                        path.pop();

                        if link_target.is_absolute() {
                            path = link_target;
                        } else {
                            path.push(link_target);
                        }
                        continue;
                    }
                }

                match can_mode {
                    CanonicalizeMode::Existing => {
                        if !path.exists() {
                            return Err(io::Error::new(
                                io::ErrorKind::NotFound,
                                "Path component does not exist",
                            ));
                        }
                    }
                    CanonicalizeMode::AllButLast => {
                        if components.as_path().as_os_str().is_empty() {
                            break;
                        }
                        if !path.exists() {
                            return Err(io::Error::new(
                                io::ErrorKind::NotFound,
                                "Path component does not exist",
                            ));
                        }
                    }
                    CanonicalizeMode::Missing => {}
                    _ => {}
                }
            }
        }
    }

    Ok(path)
}

pub fn canonicalize_file_name(name: &str) -> io::Result<PathBuf> {
    canonicalize_filename_mode(name, CanonicalizeMode::Existing)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_canonicalize_existing() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");
        fs::write(&path, "test").unwrap();

        let canonical = canonicalize_filename_mode(
            path.to_str().unwrap(),
            CanonicalizeMode::Existing,
        )
        .unwrap();
        assert_eq!(canonical, fs::canonicalize(path).unwrap());
    }

    #[test]
    fn test_canonicalize_missing() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nonexistent.txt");

        let result = canonicalize_filename_mode(
            path.to_str().unwrap(),
            CanonicalizeMode::Missing,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_canonicalize_symlink() {
        let dir = tempdir().unwrap();
        let target_path = dir.path().join("target.txt");
        fs::write(&target_path, "test").unwrap();

        let link_path = dir.path().join("link.txt");
        std::os::unix::fs::symlink(&target_path, &link_path).unwrap();

        let canonical = canonicalize_filename_mode(
            link_path.to_str().unwrap(),
            CanonicalizeMode::Existing,
        )
        .unwrap();
        assert_eq!(canonical, fs::canonicalize(target_path).unwrap());
    }
}
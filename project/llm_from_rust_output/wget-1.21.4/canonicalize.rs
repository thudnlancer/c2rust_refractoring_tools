use std::ffi::{CStr, CString};
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalizeMode {
    Existing,
    AllButLast,
    Missing,
    NoLinks,
}

impl CanonicalizeMode {
    fn allows_missing(&self) -> bool {
        matches!(self, CanonicalizeMode::Missing)
    }

    fn allows_all_but_last(&self) -> bool {
        matches!(self, CanonicalizeMode::AllButLast)
    }

    fn no_links(&self) -> bool {
        matches!(self, CanonicalizeMode::NoLinks)
    }
}

pub fn canonicalize_filename_mode(
    name: &Path,
    can_mode: CanonicalizeMode,
) -> Result<PathBuf, std::io::Error> {
    if name.as_os_str().is_empty() {
        return Err(std::io::Error::from_raw_os_error(libc::ENOENT));
    }

    let mut result = if name.is_absolute() {
        PathBuf::from("/")
    } else {
        std::env::current_dir()?
    };

    let components = name.components();
    let mut num_links = 0;
    let mut seen = std::collections::HashMap::new();

    for component in components {
        let component_str = component.as_os_str().as_bytes();
        if component_str == b"." {
            continue;
        }

        if component_str == b".." {
            if !result.pop() && result != PathBuf::from("/") {
                result.push("..");
            }
            continue;
        }

        result.push(component);
        let metadata = match fs::symlink_metadata(&result) {
            Ok(m) => m,
            Err(e) if can_mode.allows_missing() && e.kind() == std::io::ErrorKind::NotFound => {
                continue;
            }
            Err(e) => return Err(e),
        };

        if metadata.file_type().is_symlink() && !can_mode.no_links() {
            if num_links >= 20 {
                return Err(std::io::Error::from_raw_os_error(libc::ELOOP));
            }
            num_links += 1;

            let link_target = fs::read_link(&result)?;
            result.pop();

            if link_target.is_absolute() {
                result = PathBuf::from("/");
            }
            result.push(link_target);
        } else {
            let ino = metadata.ino();
            if seen.contains_key(&ino) {
                if can_mode.allows_missing() {
                    continue;
                } else {
                    return Err(std::io::Error::from_raw_os_error(libc::ELOOP));
                }
            }
            seen.insert(ino, ());
        }
    }

    if result.as_os_str().is_empty() {
        result.push(".");
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_canonicalize_basic() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");
        fs::write(&path, "test").unwrap();

        let canonical = canonicalize_filename_mode(&path, CanonicalizeMode::Existing).unwrap();
        assert!(canonical.is_absolute());
        assert!(canonical.ends_with("test.txt"));
    }

    #[test]
    fn test_canonicalize_missing() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nonexistent.txt");

        let result = canonicalize_filename_mode(&path, CanonicalizeMode::Missing);
        assert!(result.is_ok());
    }

    #[test]
    fn test_canonicalize_symlink() {
        let dir = tempdir().unwrap();
        let target = dir.path().join("target.txt");
        fs::write(&target, "test").unwrap();

        let link = dir.path().join("link");
        std::os::unix::fs::symlink(&target, &link).unwrap();

        let canonical = canonicalize_filename_mode(&link, CanonicalizeMode::Existing).unwrap();
        assert!(canonical.is_absolute());
        assert!(canonical.ends_with("target.txt"));
    }
}
use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug)]
pub enum PathSearchError {
    NoValidDirectory,
    InsufficientBuffer,
    IoError(std::io::Error),
}

impl From<std::io::Error> for PathSearchError {
    fn from(err: std::io::Error) -> Self {
        PathSearchError::IoError(err)
    }
}

/// Determine if a directory exists and is accessible
fn dir_exists(dir: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(dir) {
        metadata.is_dir()
    } else {
        false
    }
}

/// Path search algorithm for temporary files
/// 
/// If `dir` is Some and exists, uses it; otherwise uses the first of:
/// - $TMPDIR
/// - P_tmpdir (/tmp on Unix)
/// - /tmp
/// 
/// Returns a template suitable for use with tempfile creation functions.
pub fn path_search(
    dir: Option<&Path>,
    prefix: Option<&str>,
    try_tmpdir: bool,
) -> Result<PathBuf, PathSearchError> {
    let prefix = prefix.unwrap_or("file");
    let prefix = if prefix.len() > 5 {
        &prefix[..5]
    } else {
        prefix
    };

    let temp_dir = if try_tmpdir {
        if let Some(d) = env::var_os("TMPDIR") {
            let tmpdir = PathBuf::from(d);
            if dir_exists(&tmpdir) {
                Some(tmpdir)
            } else {
                dir.and_then(|d| if dir_exists(d) { Some(d.to_path_buf()) } else { None })
            }
        } else {
            dir.and_then(|d| if dir_exists(d) { Some(d.to_path_buf()) } else { None })
        }
    } else {
        dir.and_then(|d| if dir_exists(d) { Some(d.to_path_buf()) } else { None })
    };

    let temp_dir = match temp_dir {
        Some(d) => d,
        None => {
            // Try system-specific temp directories
            #[cfg(windows)]
            {
                if let Ok(temp) = env::var("TEMP") {
                    let temp_path = PathBuf::from(temp);
                    if dir_exists(&temp_path) {
                        temp_path
                    } else {
                        PathBuf::from("/tmp")
                    }
                } else {
                    PathBuf::from("/tmp")
                }
            }
            #[cfg(not(windows))]
            {
                let p_tmpdir = Path::new("/tmp");
                if dir_exists(p_tmpdir) {
                    p_tmpdir.to_path_buf()
                } else {
                    return Err(PathSearchError::NoValidDirectory);
                }
            }
        }
    };

    let mut template = temp_dir;
    if !template.as_os_str().is_empty() {
        let last_char = template.as_os_str().to_string_lossy().chars().last();
        if last_char != Some('/') && last_char != Some('\\') {
            template.push(std::path::MAIN_SEPARATOR.to_string());
        }
    }

    template.push(prefix);
    template.push("XXXXXX");

    Ok(template)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_path_search_with_dir() {
        let temp_dir = env::temp_dir();
        let result = path_search(Some(&temp_dir), Some("test"), true);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.to_string_lossy().contains("testXXXXXX"));
    }

    #[test]
    fn test_path_search_without_dir() {
        let result = path_search(None, Some("test"), true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_path_search_with_invalid_dir() {
        let result = path_search(Some(Path::new("/nonexistent/dir")), Some("test"), false);
        assert!(matches!(result, Err(PathSearchError::NoValidDirectory)));
    }
}
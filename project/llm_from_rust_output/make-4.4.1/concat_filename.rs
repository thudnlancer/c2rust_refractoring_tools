use std::ffi::CString;
use std::path::{Path, PathBuf};

pub fn concatenated_filename(
    directory: &str,
    filename: &str,
    suffix: Option<&str>,
) -> Option<CString> {
    let mut path = if directory == "." {
        PathBuf::new()
    } else {
        let mut path = PathBuf::from(directory);
        // Ensure directory ends with a separator if not empty
        if !directory.is_empty() && !directory.ends_with('/') {
            path.push("");
        }
        path
    };

    path.push(filename);
    if let Some(suffix) = suffix {
        path.set_extension(suffix.trim_start_matches('.'));
    }

    CString::new(path.to_string_lossy().into_owned()).ok()
}
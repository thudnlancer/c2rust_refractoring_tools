use std::env;
use std::ffi::CString;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Stat {
    st_mode: u32,
}

fn direxists(dir: &Path) -> bool {
    match fs::metadata(dir) {
        Ok(metadata) => {
            let mode = metadata.permissions().mode();
            mode & 0o170000 == 0o40000
        }
        Err(_) => false,
    }
}

pub fn path_search(
    tmpl: &mut [u8],
    dir: Option<&str>,
    pfx: Option<&str>,
    try_tmpdir: bool,
) -> Result<(), std::io::Error> {
    let pfx = pfx.unwrap_or("file");
    let pfx = if pfx.len() > 5 { &pfx[..5] } else { pfx };

    let dir = if try_tmpdir {
        env::var_os("TMPDIR")
            .and_then(|d| {
                let path = Path::new(&d);
                if direxists(path) {
                    Some(path)
                } else {
                    None
                }
            })
            .or_else(|| dir.and_then(|d| Some(Path::new(d)).filter(|p| direxists(p))))
    } else {
        dir.and_then(|d| Some(Path::new(d)).filter(|p| direxists(p)))
    };

    let dir = match dir {
        Some(d) => d,
        None => {
            let default_tmp = Path::new("/tmp");
            if direxists(default_tmp) {
                default_tmp
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No suitable directory found",
                ));
            }
        }
    };

    let dir_str = dir.as_os_str().as_bytes();
    let needs_slash = !dir_str.ends_with(b"/");

    let required_len = dir_str.len()
        + if needs_slash { 1 } else { 0 }
        + pfx.len()
        + 6 // "XXXXXX"
        + 1; // null terminator

    if tmpl.len() < required_len {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Template buffer too small",
        ));
    }

    let mut offset = 0;
    tmpl[offset..offset + dir_str.len()].copy_from_slice(dir_str);
    offset += dir_str.len();

    if needs_slash {
        tmpl[offset] = b'/';
        offset += 1;
    }

    tmpl[offset..offset + pfx.len()].copy_from_slice(pfx.as_bytes());
    offset += pfx.len();

    tmpl[offset..offset + 6].copy_from_slice(b"XXXXXX");
    offset += 6;
    tmpl[offset] = 0;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_search() {
        let mut buffer = [0u8; 1024];
        let result = path_search(&mut buffer, Some("/tmp"), Some("test"), true);
        assert!(result.is_ok());
    }
}
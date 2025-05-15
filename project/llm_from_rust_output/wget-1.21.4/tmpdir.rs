use std::env;
use std::ffi::{CString, CStr};
use std::fs;
use std::io;
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
) -> io::Result<()> {
    let pfx = pfx.unwrap_or("file");
    let pfx = if pfx.len() > 5 { &pfx[..5] } else { pfx };

    let dir = if try_tmpdir {
        env::var_os("TMPDIR")
            .map(PathBuf::from)
            .filter(|p| direxists(p))
            .or_else(|| dir.map(PathBuf::from).filter(|p| direxists(p))
    } else {
        dir.map(PathBuf::from).filter(|p| direxists(p))
    };

    let dir = match dir {
        Some(d) => d,
        None => {
            let default_tmp = Path::new("/tmp");
            if direxists(default_tmp) {
                default_tmp.to_path_buf()
            } else {
                return Err(io::Error::new(io::ErrorKind::NotFound, "No suitable directory found"));
            }
        }
    };

    let dir_str = dir.as_os_str().as_bytes();
    let add_slash = !dir_str.is_empty() && dir_str.last() != Some(&b'/');

    let required_len = dir_str.len()
        + if add_slash { 1 } else { 0 }
        + pfx.len()
        + 6 // for "XXXXXX"
        + 1; // for null terminator

    if tmpl.len() < required_len {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Template buffer too small",
        ));
    }

    let mut offset = 0;
    tmpl[offset..offset + dir_str.len()].copy_from_slice(dir_str);
    offset += dir_str.len();

    if add_slash {
        tmpl[offset] = b'/';
        offset += 1;
    }

    let format = format!("%.*sXXXXXX", pfx.len(), pfx);
    tmpl[offset..offset + format.len()].copy_from_slice(format.as_bytes());
    offset += format.len();
    tmpl[offset] = 0;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_search() {
        let mut buffer = [0u8; 256];
        let result = path_search(&mut buffer, Some("/tmp"), Some("test"), true);
        assert!(result.is_ok());
        let c_str = CStr::from_bytes_until_nul(&buffer).unwrap();
        assert!(c_str.to_str().unwrap().contains("/tmp/testXXXXXX"));
    }
}
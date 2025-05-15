use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::os::unix::ffi::OsStrExt;

/// Determine if the given path exists and is a directory.
fn direxists(dir: &Path) -> bool {
    match fs::metadata(dir) {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false,
    }
}

/// Path search algorithm, for tmpnam, tmpfile, etc. If `dir` is
/// Some and exists, uses it; otherwise uses the first of $TMPDIR,
/// P_tmpdir, /tmp that exists. Returns a PathBuf containing a template
/// suitable for use with mk[s]temp. Returns an error if:
/// - dir is Some but doesn't exist
/// - none of the searched dirs exists
/// - the resulting path would be too long
pub fn path_search(
    dir: Option<&Path>,
    pfx: Option<&str>,
    try_tmpdir: bool,
) -> io::Result<PathBuf> {
    let pfx = pfx.unwrap_or("file");
    let pfx = if pfx.len() > 5 { &pfx[..5] } else { pfx };

    let dir = if try_tmpdir {
        if let Some(d) = env::var_os("TMPDIR") {
            let tmpdir = Path::new(&d);
            if direxists(tmpdir) {
                Some(tmpdir)
            } else {
                dir.and_then(|d| if direxists(d) { Some(d) } else { None })
            }
        } else {
            dir.and_then(|d| if direxists(d) { Some(d) } else { None })
        }
    } else {
        dir.and_then(|d| if direxists(d) { Some(d) } else { None })
    };

    let dir = match dir {
        Some(d) => d,
        None => {
            #[cfg(windows)]
            {
                if let Some(temp_path) = get_windows_temp_path()? {
                    temp_path
                } else if direxists(Path::new(P_tmpdir)) {
                    Path::new(P_tmpdir)
                } else if P_tmpdir != "/tmp" && direxists(Path::new("/tmp")) {
                    Path::new("/tmp")
                } else {
                    return Err(io::Error::new(io::ErrorKind::NotFound, "No temporary directory found"));
                }
            }
            #[cfg(not(windows))]
            {
                if direxists(Path::new(P_tmpdir)) {
                    Path::new(P_tmpdir)
                } else if P_tmpdir != "/tmp" && direxists(Path::new("/tmp")) {
                    Path::new("/tmp")
                } else {
                    return Err(io::Error::new(io::ErrorKind::NotFound, "No temporary directory found"));
                }
            }
        }
    };

    let mut path = dir.to_path_buf();
    if !dir.as_os_str().as_bytes().ends_with(&[b'/']) && !dir.as_os_str().as_bytes().is_empty() {
        path.push("/");
    }
    path.push(format!("{}XXXXXX", pfx));

    Ok(path)
}

#[cfg(windows)]
fn get_windows_temp_path() -> io::Result<Option<PathBuf>> {
    use std::ptr;
    use winapi::um::fileapi::GetTempPathW;
    use winapi::um::winbase;

    unsafe {
        let mut buffer: [u16; winbase::MAX_PATH as usize] = [0; winbase::MAX_PATH as usize];
        let len = GetTempPathW(buffer.len() as u32, buffer.as_mut_ptr());
        if len == 0 || len >= buffer.len() as u32 {
            return Ok(None);
        }

        let path = PathBuf::from(OsStr::from_wide(&buffer[..len as usize]));
        if direxists(&path) {
            Ok(Some(path))
        } else {
            Ok(None)
        }
    }
}

#[cfg(windows)]
const P_tmpdir: &str = "\\";
#[cfg(not(windows))]
const P_tmpdir: &str = "/tmp";
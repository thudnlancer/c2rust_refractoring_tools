use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::io;

/// Looks up a program in the PATH.
/// Attempts to determine the pathname that would be called by execlp/execvp
/// of PROGNAME. If successful, it returns a pathname containing a slash
/// (either absolute or relative to the current directory). Otherwise, it
/// returns PROGNAME unmodified.
pub fn find_in_path(progname: &str) -> PathBuf {
    #[cfg(any(target_os = "windows", target_os = "cygwin", target_os = "emscripten", target_os = "dos"))]
    {
        // Native Windows, Cygwin, OS/2, DOS - leave it to the OS
        return PathBuf::from(progname);
    }

    #[cfg(not(any(target_os = "windows", target_os = "cygwin", target_os = "emscripten", target_os = "dos")))]
    {
        // Unix-like systems
        if progname.contains('/') {
            return PathBuf::from(progname);
        }

        let path = match env::var_os("PATH") {
            Some(p) => p,
            None => return PathBuf::from(progname),
        };

        for dir in env::split_paths(&path) {
            let dir = if dir.as_os_str().is_empty() {
                Path::new(".")
            } else {
                &dir
            };

            let progpath = dir.join(progname);
            
            if let Ok(metadata) = fs::metadata(&progpath) {
                if !metadata.is_dir() && is_executable(&progpath) {
                    if progpath == Path::new(progname) {
                        // Add ./ prefix
                        let mut path = PathBuf::new();
                        path.push(".");
                        path.push(progname);
                        return path;
                    }
                    return progpath;
                }
            }
        }

        PathBuf::from(progname)
    }
}

/// Checks if a file is executable
fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::metadata(path)
            .map(|m| m.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }
    
    #[cfg(not(unix))]
    {
        // On non-Unix systems, assume file is executable if it exists
        fs::metadata(path).is_ok()
    }
}

/// Looks up a program in the given PATH-like string.
pub fn find_in_given_path(progname: &str, path: Option<&str>, optimize_for_exec: bool) -> io::Result<PathBuf> {
    #[cfg(any(target_os = "windows", target_os = "cygwin", target_os = "emscripten", target_os = "dos"))]
    {
        // Native Windows, Cygwin, OS/2, DOS - leave it to the OS
        return Ok(PathBuf::from(progname));
    }

    #[cfg(not(any(target_os = "windows", target_os = "cygwin", target_os = "emscripten", target_os = "dos")))]
    {
        if progname.contains('/') {
            return Ok(PathBuf::from(progname));
        }

        let path = path.unwrap_or("");
        let separator = if cfg!(windows) { ';' } else { ':' };

        for dir in path.split(separator) {
            let dir = if dir.is_empty() {
                Path::new(".")
            } else {
                Path::new(dir)
            };

            let progpath = dir.join(progname);
            
            if let Ok(metadata) = fs::metadata(&progpath) {
                if !metadata.is_dir() && (optimize_for_exec || is_executable(&progpath)) {
                    if progpath == Path::new(progname) {
                        // Add ./ prefix
                        let mut path = PathBuf::new();
                        path.push(".");
                        path.push(progname);
                        return Ok(path);
                    }
                    return Ok(progpath);
                }
            }
        }

        Err(io::Error::new(io::ErrorKind::NotFound, "Program not found in PATH"))
    }
}
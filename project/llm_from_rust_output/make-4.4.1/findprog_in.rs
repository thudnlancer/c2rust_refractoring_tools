use std::ffi::{CString, CStr};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::fs;
use std::io;

#[derive(Debug)]
struct Stat {
    st_dev: u64,
    st_ino: u64,
    st_nlink: u64,
    st_mode: u32,
    st_uid: u32,
    st_gid: u32,
    st_rdev: u64,
    st_size: i64,
    st_blksize: i64,
    st_blocks: i64,
    st_atime: i64,
    st_mtime: i64,
    st_ctime: i64,
}

fn stat(path: &Path) -> io::Result<Stat> {
    let metadata = fs::metadata(path)?;
    Ok(Stat {
        st_dev: metadata.dev(),
        st_ino: metadata.ino(),
        st_nlink: metadata.nlink(),
        st_mode: metadata.mode(),
        st_uid: metadata.uid(),
        st_gid: metadata.gid(),
        st_rdev: metadata.rdev(),
        st_size: metadata.len() as i64,
        st_blksize: metadata.blksize().unwrap_or(0) as i64,
        st_blocks: metadata.blocks().unwrap_or(0) as i64,
        st_atime: metadata.accessed()?.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
        st_mtime: metadata.modified()?.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
        st_ctime: metadata.created()?.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
    })
}

fn concatenated_filename(directory: Option<&str>, filename: &str, suffix: Option<&str>) -> PathBuf {
    let mut path = PathBuf::new();
    if let Some(dir) = directory {
        path.push(dir);
    }
    path.push(filename);
    if let Some(suf) = suffix {
        path.set_extension(suf);
    }
    path
}

fn find_in_given_path(
    progname: &str,
    path: Option<&str>,
    directory: Option<&str>,
    optimize_for_exec: bool,
) -> io::Result<PathBuf> {
    let has_slash = progname.contains('/');
    
    if has_slash {
        if optimize_for_exec {
            return Ok(PathBuf::from(progname));
        } else {
            let mut failure_errno = io::Error::from_raw_os_error(2);
            let suffixes = [""];
            
            for suffix in &suffixes {
                let directory_as_prefix = if directory.is_some() && !progname.starts_with('/') {
                    directory
                } else {
                    None
                };
                
                let progpathname = concatenated_filename(directory_as_prefix, progname, Some(suffix));
                
                match fs::metadata(&progpathname) {
                    Ok(metadata) => {
                        if !metadata.is_dir() {
                            if progpathname == PathBuf::from(progname) {
                                return Ok(PathBuf::from(progname));
                            } else {
                                return Ok(progpathname);
                            }
                        } else {
                            failure_errno = io::Error::from_raw_os_error(13);
                        }
                    }
                    Err(e) => {
                        if e.raw_os_error() != Some(2) {
                            failure_errno = e;
                        }
                    }
                }
            }
            return Err(failure_errno);
        }
    }
    
    let path = path.unwrap_or("");
    let path_components = path.split(':');
    let mut failure_errno = io::Error::from_raw_os_error(2);
    
    for dir in path_components {
        let dir = if dir.is_empty() { "." } else { dir };
        let dir_as_prefix = if directory.is_some() && !dir.starts_with('/') {
            concatenated_filename(directory, dir, None)
        } else {
            PathBuf::from(dir)
        };
        
        let suffixes = [""];
        for suffix in &suffixes {
            let progpathname = concatenated_filename(Some(dir_as_prefix.to_str().unwrap()), progname, Some(suffix));
            
            match fs::metadata(&progpathname) {
                Ok(metadata) => {
                    if !metadata.is_dir() {
                        if progpathname == PathBuf::from(progname) {
                            let mut new_path = PathBuf::new();
                            new_path.push("./");
                            new_path.push(progname);
                            return Ok(new_path);
                        } else {
                            return Ok(progpathname);
                        }
                    } else {
                        failure_errno = io::Error::from_raw_os_error(13);
                    }
                }
                Err(e) => {
                    if e.raw_os_error() != Some(2) {
                        failure_errno = e;
                    }
                }
            }
        }
    }
    
    Err(failure_errno)
}
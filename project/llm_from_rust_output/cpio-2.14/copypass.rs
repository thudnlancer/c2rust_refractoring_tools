use std::fs::{self, File, OpenOptions, Metadata, Permissions};
use std::io::{self, Read, Write};
use std::os::unix::fs::{symlink, MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use nix::sys::stat::{mknod, Mode, SFlag};
use nix::unistd::{chown, link};
use libc::{uid_t, gid_t, mode_t, dev_t};
use std::ffi::{CString, CStr};
use std::os::unix::ffi::OsStrExt;
use std::sync::atomic::{AtomicBool, Ordering};

static VERBOSE: AtomicBool = AtomicBool::new(false);
static DOT_FLAG: AtomicBool = AtomicBool::new(false);
static LINK_FLAG: AtomicBool = AtomicBool::new(false);
static CREATE_DIR_FLAG: AtomicBool = AtomicBool::new(false);
static UNCONDITIONAL_FLAG: AtomicBool = AtomicBool::new(false);
static QUIET_FLAG: AtomicBool = AtomicBool::new(false);
static RESET_TIME_FLAG: AtomicBool = AtomicBool::new(false);
static NO_CHOWN_FLAG: AtomicBool = AtomicBool::new(false);
static SET_OWNER_FLAG: AtomicBool = AtomicBool::new(false);
static SET_GROUP_FLAG: AtomicBool = AtomicBool::new(false);

struct FileStat {
    st_mode: mode_t,
    st_uid: uid_t,
    st_gid: gid_t,
    st_size: u64,
    st_dev: dev_t,
    st_ino: u64,
    st_nlink: u64,
    st_atime: i64,
    st_mtime: i64,
    st_rdev: dev_t,
}

fn set_copypass_perms(path: &Path, st: &FileStat) -> io::Result<()> {
    let perms = Permissions::from_mode(st.st_mode);
    fs::set_permissions(path, perms)?;
    
    if !NO_CHOWN_FLAG.load(Ordering::Relaxed) {
        let uid = if SET_OWNER_FLAG.load(Ordering::Relaxed) {
            SET_OWNER.load(Ordering::Relaxed)
        } else {
            st.st_uid
        };
        
        let gid = if SET_GROUP_FLAG.load(Ordering::Relaxed) {
            SET_GROUP.load(Ordering::Relaxed)
        } else {
            st.st_gid
        };
        
        chown(path, Some(uid), Some(gid))?;
    }
    Ok(())
}

fn process_copy_pass() -> io::Result<()> {
    let stdin = io::stdin();
    let mut input = String::new();
    let mut output_bytes = 0;
    let mut output_path = PathBuf::new();
    
    while stdin.read_line(&mut input)? > 0 {
        let input_path = Path::new(input.trim());
        if input_path.as_os_str().is_empty() {
            eprintln!("blank line ignored");
            continue;
        }
        
        if input_path.starts_with(".") && 
           (input_path == Path::new(".") || input_path == Path::new("./")) {
            continue;
        }
        
        let in_meta = fs::metadata(input_path)?;
        let in_stat = FileStat {
            st_mode: in_meta.mode(),
            st_uid: in_meta.uid(),
            st_gid: in_meta.gid(),
            st_size: in_meta.size(),
            st_dev: in_meta.dev(),
            st_ino: in_meta.ino(),
            st_nlink: in_meta.nlink(),
            st_atime: in_meta.atime(),
            st_mtime: in_meta.mtime(),
            st_rdev: in_meta.rdev(),
        };
        
        let output_path = output_path.join(input_path);
        let out_meta = fs::symlink_metadata(&output_path);
        
        let existing_dir = if let Ok(meta) = out_meta {
            if meta.is_dir() && in_meta.is_dir() {
                true
            } else if !UNCONDITIONAL_FLAG.load(Ordering::Relaxed) && 
                      in_meta.modified()? <= meta.modified()? {
                eprintln!("{} not created: newer or same age version exists", 
                         output_path.display());
                continue;
            } else {
                if meta.is_dir() {
                    fs::remove_dir(&output_path)?;
                } else {
                    fs::remove_file(&output_path)?;
                }
                false
            }
        } else {
            false
        };
        
        if in_meta.is_file() {
            if LINK_FLAG.load(Ordering::Relaxed) {
                if let Err(e) = link(input_path, &output_path) {
                    if in_stat.st_nlink > 1 {
                        if let Err(_) = link_to_maj_min_ino(&output_path, 
                                                          in_stat.st_dev, 
                                                          in_stat.st_ino) {
                            copy_file(input_path, &output_path, &in_stat)?;
                        }
                    } else {
                        copy_file(input_path, &output_path, &in_stat)?;
                    }
                }
            } else {
                copy_file(input_path, &output_path, &in_stat)?;
            }
            output_bytes += in_stat.st_size;
        } else if in_meta.is_dir() {
            fs::create_dir(&output_path)?;
            set_copypass_perms(&output_path, &in_stat)?;
        } else if in_meta.file_type().is_symlink() {
            let link_target = fs::read_link(input_path)?;
            symlink(&link_target, &output_path)?;
            set_copypass_perms(&output_path, &in_stat)?;
        } else {
            // Handle special files
            mknod(&output_path, 
                  SFlag::from_bits_truncate(in_stat.st_mode), 
                  Mode::from_bits_truncate(in_stat.st_mode), 
                  in_stat.st_rdev)?;
            set_copypass_perms(&output_path, &in_stat)?;
        }
        
        if VERBOSE.load(Ordering::Relaxed) {
            println!("{}", output_path.display());
        }
        
        if DOT_FLAG.load(Ordering::Relaxed) {
            print!(".");
        }
        
        input.clear();
    }
    
    if DOT_FLAG.load(Ordering::Relaxed) {
        println!();
    }
    
    if !QUIET_FLAG.load(Ordering::Relaxed) {
        println!("{} blocks", output_bytes / 512);
    }
    
    Ok(())
}

fn copy_file(src: &Path, dst: &Path, stat: &FileStat) -> io::Result<()> {
    let mut src_file = File::open(src)?;
    let mut dst_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(dst)?;
    
    io::copy(&mut src_file, &mut dst_file)?;
    set_copypass_perms(dst, stat)?;
    
    if RESET_TIME_FLAG.load(Ordering::Relaxed) {
        let atime = SystemTime::UNIX_EPOCH + Duration::from_secs(stat.st_atime as u64);
        let mtime = SystemTime::UNIX_EPOCH + Duration::from_secs(stat.st_mtime as u64);
        filetime::set_file_times(dst, atime, mtime)?;
    }
    
    Ok(())
}

fn link_to_maj_min_ino(path: &Path, dev_maj: dev_t, dev_min: dev_t, ino: u64) -> io::Result<()> {
    // Implementation would need to track inodes like original C code
    // This is a simplified version
    Err(io::Error::new(io::ErrorKind::Other, "Not implemented"))
}
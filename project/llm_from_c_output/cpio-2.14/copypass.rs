use std::fs::{self, File, OpenOptions, Metadata, Permissions};
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::os::unix::fs::{PermissionsExt, MetadataExt, symlink, chown};
use std::path::{Path, PathBuf};
use std::os::unix::fs::FileTypeExt;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use nix::sys::stat::{major, minor};
use nix::unistd::{chown as lchown};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref INODE_MAP: Mutex<HashMap<(u64, u64, u64), PathBuf>> = Mutex::new(HashMap::new());
}

struct CpioFileStat {
    c_name: PathBuf,
    st_mode: u32,
    st_uid: u32,
    st_gid: u32,
    st_size: u64,
    st_dev: u64,
    st_ino: u64,
    st_rdev: u64,
    st_mtime: i64,
    st_atime: i64,
}

fn stat_to_cpio(header: &mut CpioFileStat, st: &Metadata) {
    header.st_mode = st.mode();
    header.st_uid = st.uid();
    header.st_gid = st.gid();
    header.st_size = st.size();
    header.st_dev = st.dev();
    header.st_ino = st.ino();
    header.st_rdev = st.rdev();
    header.st_mtime = st.mtime();
    header.st_atime = st.atime();
}

fn set_perms(fd: Option<&File>, header: &CpioFileStat) -> io::Result<()> {
    let path = &header.c_name;
    let metadata = fs::metadata(path)?;
    
    if !no_chown_flag {
        let uid = if set_owner_flag { set_owner } else { header.st_uid };
        let gid = if set_group_flag { set_group } else { header.st_gid };
        lchown(path, Some(uid), Some(gid))?;
    }
    
    fs::set_permissions(path, Permissions::from_mode(header.st_mode))?;
    
    if let Some(file) = fd {
        file.sync_all()?;
    }
    
    Ok(())
}

fn set_copypass_perms(fd: Option<&File>, name: &Path, st: &Metadata) -> io::Result<()> {
    let mut header = CpioFileStat {
        c_name: name.to_path_buf(),
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        st_size: 0,
        st_dev: 0,
        st_ino: 0,
        st_rdev: 0,
        st_mtime: 0,
        st_atime: 0,
    };
    stat_to_cpio(&mut header, st);
    set_perms(fd, &header)
}

fn process_copy_pass() -> io::Result<()> {
    let mut input_name = String::new();
    let mut output_name = PathBuf::from(directory_name);
    let dirname_len = output_name.as_os_str().len();
    
    let newdir_umask = unsafe { libc::umask(0) };
    
    if change_directory_option && !directory_name.starts_with('/') {
        let pwd = std::env::current_dir()?;
        output_name = pwd.join(&output_name);
    }
    
    output_name.push("/");
    let dirname_len = output_name.as_os_str().len();
    
    std::env::set_current_dir(&output_name)?;
    
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    
    while reader.read_line(&mut input_name)? > 0 {
        let input_name = input_name.trim();
        
        if input_name.is_empty() {
            eprintln!("blank line ignored");
            continue;
        }
        
        if input_name == "." || input_name == "./" {
            continue;
        }
        
        let in_file_stat = fs::metadata(input_name)?;
        
        let mut output_path = output_name.clone();
        output_path.push(input_name.trim_start_matches('/'));
        
        let mut existing_dir = false;
        if let Ok(out_file_stat) = fs::symlink_metadata(&output_path) {
            if out_file_stat.file_type().is_dir() && in_file_stat.file_type().is_dir() {
                existing_dir = true;
            } else if !unconditional_flag && 
                      in_file_stat.modified()? <= out_file_stat.modified()? {
                eprintln!("{} not created: newer or same age version exists", 
                         output_path.display());
                continue;
            } else {
                if out_file_stat.file_type().is_dir() {
                    fs::remove_dir(&output_path)?;
                } else {
                    fs::remove_file(&output_path)?;
                }
            }
        }
        
        if in_file_stat.file_type().is_file() {
            let mut link_res = -1;
            
            if link_flag {
                link_res = link_to_name(&output_path, input_name)?;
            }
            
            if link_res < 0 && in_file_stat.nlink() > 1 {
                link_res = link_to_maj_min_ino(
                    &output_path,
                    major(in_file_stat.dev()),
                    minor(in_file_stat.dev()),
                    in_file_stat.ino(),
                )?;
            }
            
            if link_res < 0 {
                let in_file = File::open(input_name)?;
                let mut out_file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .mode(0o600)
                    .open(&output_path);
                
                if out_file.is_err() && create_dir_flag {
                    create_all_directories(&output_path)?;
                    out_file = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .mode(0o600)
                        .open(&output_path);
                }
                
                let mut out_file = out_file?;
                io::copy(&mut BufReader::new(in_file), &mut out_file)?;
                out_file.sync_all()?;
                
                set_copypass_perms(Some(&out_file), &output_path, &in_file_stat)?;
                
                if reset_time_flag {
                    set_file_times(
                        Some(&out_file),
                        &output_path,
                        in_file_stat.accessed()?,
                        in_file_stat.modified()?,
                    )?;
                }
            }
        } else if in_file_stat.file_type().is_dir() {
            let mut header = CpioFileStat {
                c_name: output_path.clone(),
                st_mode: in_file_stat.mode(),
                st_uid: in_file_stat.uid(),
                st_gid: in_file_stat.gid(),
                st_size: in_file_stat.size(),
                st_dev: in_file_stat.dev(),
                st_ino: in_file_stat.ino(),
                st_rdev: in_file_stat.rdev(),
                st_mtime: in_file_stat.mtime(),
                st_atime: in_file_stat.atime(),
            };
            cpio_create_dir(&header, existing_dir)?;
        } else if in_file_stat.file_type().is_char_device() ||
                  in_file_stat.file_type().is_block_device() ||
                  in_file_stat.file_type().is_fifo() ||
                  in_file_stat.file_type().is_socket() {
            let mut link_res = -1;
            
            if link_flag {
                link_res = link_to_name(&output_path, input_name)?;
            }
            
            if link_res < 0 && in_file_stat.nlink() > 1 {
                link_res = link_to_maj_min_ino(
                    &output_path,
                    major(in_file_stat.dev()),
                    minor(in_file_stat.dev()),
                    in_file_stat.ino(),
                )?;
            }
            
            if link_res < 0 {
                let res = unsafe {
                    libc::mknod(
                        CString::new(output_path.as_os_str().as_bytes())?.as_ptr(),
                        in_file_stat.mode(),
                        in_file_stat.rdev(),
                    )
                };
                
                if res < 0 && create_dir_flag {
                    create_all_directories(&output_path)?;
                    let res = unsafe {
                        libc::mknod(
                            CString::new(output_path.as_os_str().as_bytes())?.as_ptr(),
                            in_file_stat.mode(),
                            in_file_stat.rdev(),
                        )
                    };
                }
                
                if res < 0 {
                    return Err(io::Error::last_os_error());
                }
                
                set_copypass_perms(None, &output_path, &in_file_stat)?;
            }
        } else if in_file_stat.file_type().is_symlink() {
            let link_name = fs::read_link(input_name)?;
            
            let res = symlink(&link_name, &output_path);
            if res.is_err() && create_dir_flag {
                create_all_directories(&output_path)?;
                symlink(&link_name, &output_path)?;
            }
            
            if !no_chown_flag {
                let uid = if set_owner_flag { set_owner } else { in_file_stat.uid() };
                let gid = if set_group_flag { set_group } else { in_file_stat.gid() };
                chown(&output_path, Some(uid), Some(gid))?;
            }
        } else {
            eprintln!("{}: unknown file type", input_name);
        }
        
        if verbose_flag {
            println!("{}", output_path.display());
        }
        
        if dot_flag {
            print!(".");
        }
        
        input_name.clear();
    }
    
    if dot_flag {
        println!();
    }
    
    apply_delayed_set_stat()?;
    
    if !quiet_flag {
        let blocks = (output_bytes + io_block_size - 1) / io_block_size;
        println!("{} block{}", blocks, if blocks == 1 { "" } else { "s" });
    }
    
    Ok(())
}

fn link_to_maj_min_ino(
    file_name: &Path,
    st_dev_maj: u64,
    st_dev_min: u64,
    st_ino: u64,
) -> io::Result<i32> {
    let mut map = INODE_MAP.lock().unwrap();
    if let Some(link_name) = map.get(&(st_ino, st_dev_maj, st_dev_min)) {
        link_to_name(file_name, link_name)
    } else {
        map.insert((st_ino, st_dev_maj, st_dev_min), file_name.to_path_buf());
        Ok(-1)
    }
}

fn link_to_name(link_name: &Path, link_target: &Path) -> io::Result<i32> {
    match fs::hard_link(link_target, link_name) {
        Ok(_) => {
            if verbose_flag {
                eprintln!("{} linked to {}", link_target.display(), link_name.display());
            }
            Ok(0)
        }
        Err(e) => {
            if create_dir_flag {
                create_all_directories(link_name)?;
                match fs::hard_link(link_target, link_name) {
                    Ok(_) => Ok(0),
                    Err(e) => {
                        if link_flag {
                            eprintln!("cannot link {} to {}: {}", 
                                     link_target.display(), link_name.display(), e);
                        }
                        Err(e)
                    }
                }
            } else {
                if link_flag {
                    eprintln!("cannot link {} to {}: {}", 
                             link_target.display(), link_name.display(), e);
                }
                Err(e)
            }
        }
    }
}
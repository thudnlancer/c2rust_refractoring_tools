use std::ffi::CStr;
use std::fs::File;
use std::io::{self, Write};
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use libc::{mode_t, uid_t, gid_t, dev_t, time_t};

static mut INODE_NUMBER_WIDTH: i32 = 9;
static mut BLOCK_SIZE_WIDTH: i32 = 6;
static mut NLINK_WIDTH: i32 = 3;
static mut OWNER_WIDTH: i32 = 8;
static mut GROUP_WIDTH: i32 = 8;
static mut MAJOR_DEVICE_NUMBER_WIDTH: i32 = 3;
static mut MINOR_DEVICE_NUMBER_WIDTH: i32 = 3;
static mut FILE_SIZE_WIDTH: i32 = 8;

struct FileInfo {
    mode: mode_t,
    ino: u64,
    nlink: u64,
    uid: uid_t,
    gid: gid_t,
    rdev: dev_t,
    size: i64,
    blocks: i64,
    atime: time_t,
    mtime: time_t,
    ctime: time_t,
}

fn print_num(stream: &mut File, num: u64, width: &mut i32) -> io::Result<()> {
    let s = format!("{:width$}", num, width = *width as usize);
    write!(stream, "{}", s)?;
    *width = (*width).max(s.len() as i32);
    Ok(())
}

fn list_file(
    name: &Path,
    dir_fd: i32,
    relname: &Path,
    statp: &FileInfo,
    current_time: time_t,
    output_block_size: i32,
    literal_control_chars: bool,
    stream: &mut File,
) -> io::Result<()> {
    let modebuf = format_mode(statp.mode);
    let when_local = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(statp.mtime as u64);
    
    let user_name = get_user(statp.uid).unwrap_or_else(|| statp.uid.to_string());
    let group_name = get_group(statp.gid).unwrap_or_else(|| statp.gid.to_string());
    
    let mut output_good = true;
    let mut failed_at = 0;

    // Print inode number
    let inode_str = human_readable(statp.ino, 1, 1);
    write!(stream, "{:width$}", inode_str, width = unsafe { INODE_NUMBER_WIDTH } as usize)?;
    unsafe { INODE_NUMBER_WIDTH = INODE_NUMBER_WIDTH.max(inode_str.len() as i32); }

    write!(stream, " ")?;

    // Print block size
    let block_str = human_readable(statp.blocks as u64, 512, output_block_size as u64);
    write!(stream, "{:width$}", block_str, width = unsafe { BLOCK_SIZE_WIDTH } as usize)?;
    unsafe { BLOCK_SIZE_WIDTH = BLOCK_SIZE_WIDTH.max(block_str.len() as i32); }

    write!(stream, " ")?;
    write!(stream, "{}", modebuf)?;

    // Print number of links
    write!(stream, "{:width$}", statp.nlink, width = unsafe { NLINK_WIDTH } as usize)?;
    unsafe { NLINK_WIDTH = NLINK_WIDTH.max(statp.nlink.to_string().len() as i32); }

    write!(stream, " ")?;

    // Print owner
    let user_width = unsafe { OWNER_WIDTH };
    write!(stream, "{:<width$} ", user_name, width = user_width as usize)?;
    unsafe { OWNER_WIDTH = OWNER_WIDTH.max(user_name.len() as i32); }

    // Print group
    let group_width = unsafe { GROUP_WIDTH };
    write!(stream, "{:<width$} ", group_name, width = group_width as usize)?;
    unsafe { GROUP_WIDTH = GROUP_WIDTH.max(group_name.len() as i32); }

    // Print device numbers or file size
    if statp.mode & 0o170000 == 0o20000 || statp.mode & 0o170000 == 0o60000 {
        let major = unsafe { MAJOR_DEVICE_NUMBER_WIDTH };
        let minor = unsafe { MINOR_DEVICE_NUMBER_WIDTH };
        write!(stream, "{:width$}, {:width$}", 
               dev_major(statp.rdev), dev_minor(statp.rdev),
               width = major as usize, width2 = minor as usize)?;
        unsafe { 
            MAJOR_DEVICE_NUMBER_WIDTH = MAJOR_DEVICE_NUMBER_WIDTH.max(dev_major(statp.rdev).to_string().len() as i32);
            MINOR_DEVICE_NUMBER_WIDTH = MINOR_DEVICE_NUMBER_WIDTH.max(dev_minor(statp.rdev).to_string().len() as i32);
        }
    } else {
        let blocksize = if output_block_size < 0 { output_block_size } else { 1 };
        let size_str = human_readable(statp.size as u64, 1, blocksize as u64);
        write!(stream, "{:width$}", size_str, width = unsafe { FILE_SIZE_WIDTH } as usize)?;
        unsafe { FILE_SIZE_WIDTH = FILE_SIZE_WIDTH.max(size_str.len() as i32); }
    }

    write!(stream, " ")?;

    // Print time
    let time_str = if (current_time - 6 * 30 * 24 * 60 * 60) <= statp.mtime 
        && statp.mtime <= (current_time + 60 * 60) {
        format_time(when_local, "%b %e %H:%M")?
    } else {
        format_time(when_local, "%b %e  %Y")?
    };
    write!(stream, "{} ", time_str)?;

    // Print name
    print_name(name, stream, literal_control_chars)?;

    // Handle symlinks
    if statp.mode & 0o170000 == 0o120000 {
        if let Ok(linkname) = read_link_at(dir_fd, relname) {
            write!(stream, " -> ")?;
            print_name(&linkname, stream, literal_control_chars)?;
        } else {
            eprintln!("{}: {}", name.display(), io::Error::last_os_error());
        }
    }

    writeln!(stream)?;
    Ok(())
}

// Helper functions would need to be implemented:
// - format_mode()
// - human_readable()
// - get_user()
// - get_group()
// - dev_major()
// - dev_minor()
// - format_time()
// - read_link_at()
// - print_name()
// - print_name_without_quoting()
// - print_name_with_quoting()
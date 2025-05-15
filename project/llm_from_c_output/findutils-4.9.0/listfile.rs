/* listfile.rs -- display a long listing of a file
   Copyright (C) 1991-2022 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::{
    ffi::CStr,
    fmt::Write,
    fs::{File, Metadata},
    io::{self, Write as IoWrite},
    os::unix::fs::MetadataExt,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use libc::{gid_t, uid_t};

static mut INODE_NUMBER_WIDTH: usize = 9;
static mut BLOCK_SIZE_WIDTH: usize = 6;
static mut NLINK_WIDTH: usize = 3;
static mut OWNER_WIDTH: usize = 8;
static mut GROUP_WIDTH: usize = 8;
static mut MAJOR_DEVICE_NUMBER_WIDTH: usize = 3;
static mut MINOR_DEVICE_NUMBER_WIDTH: usize = 3;
static mut FILE_SIZE_WIDTH: usize = 8;

fn print_num(stream: &mut impl IoWrite, num: u64, width: &mut usize) -> io::Result<()> {
    let s = format!("{:width$}", num, width = *width);
    let chars_out = s.len();
    stream.write_all(s.as_bytes())?;
    if *width < chars_out {
        *width = chars_out;
    }
    Ok(())
}

fn get_user_name(uid: uid_t) -> Option<String> {
    unsafe {
        let passwd = libc::getpwuid(uid);
        if passwd.is_null() {
            None
        } else {
            Some(CStr::from_ptr((*passwd).pw_name).ok()?.to_str().ok()?.to_string()
        }
    }
}

fn get_group_name(gid: gid_t) -> Option<String> {
    unsafe {
        let group = libc::getgrgid(gid);
        if group.is_null() {
            None
        } else {
            Some(CStr::from_ptr((*group).gr_name).ok()?.to_str().ok()?.to_string()
        }
    }
}

fn print_name_without_quoting(p: &str, stream: &mut impl IoWrite) -> io::Result<()> {
    write!(stream, "{}", p)?;
    Ok(())
}

fn print_name_with_quoting(p: &str, stream: &mut impl IoWrite) -> io::Result<()> {
    for c in p.chars() {
        match c {
            '\\' => write!(stream, "\\\\")?,
            '\n' => write!(stream, "\\n")?,
            '\b' => write!(stream, "\\b")?,
            '\r' => write!(stream, "\\r")?,
            '\t' => write!(stream, "\\t")?,
            '\f' => write!(stream, "\\f")?,
            ' ' => write!(stream, "\\ ")?,
            '"' => write!(stream, "\\\"")?,
            _ if c.is_ascii() && (c as u8) < 32 => write!(stream, "\\{:03o}", c as u8)?,
            _ => write!(stream, "{}", c)?,
        }
    }
    Ok(())
}

fn print_name(p: &str, stream: &mut impl IoWrite, literal_control_chars: bool) -> io::Result<()> {
    if literal_control_chars {
        print_name_without_quoting(p, stream)
    } else {
        print_name_with_quoting(p, stream)
    }
}

pub fn list_file(
    name: &str,
    dir_fd: i32,
    relname: &str,
    statp: &Metadata,
    current_time: i64,
    output_block_size: i32,
    literal_control_chars: bool,
    stream: &mut impl IoWrite,
) -> io::Result<()> {
    let mut modebuf = [0; 12];
    unsafe {
        libc::strmode(statp.mode() as libc::mode_t, modebuf.as_mut_ptr());
    }
    let mode_str = unsafe { CStr::from_ptr(modebuf.as_ptr()).to_str().unwrap() };

    // Print inode number
    let inode_str = format!("{}", statp.ino());
    let inode_width = unsafe { INODE_NUMBER_WIDTH };
    write!(stream, "{:width$}", inode_str, width = inode_width)?;
    if inode_str.len() > inode_width {
        unsafe { INODE_NUMBER_WIDTH = inode_str.len() };
    }
    write!(stream, " ")?;

    // Print block size
    let blocks = statp.blocks();
    let block_str = format!("{}", blocks);
    let block_width = unsafe { BLOCK_SIZE_WIDTH };
    write!(stream, "{:width$}", block_str, width = block_width)?;
    if block_str.len() > block_width {
        unsafe { BLOCK_SIZE_WIDTH = block_str.len() };
    }
    write!(stream, " ")?;

    // Print mode and links
    write!(stream, "{}", mode_str)?;
    let nlink_str = format!("{}", statp.nlink());
    let nlink_width = unsafe { NLINK_WIDTH };
    write!(stream, "{:width$}", nlink_str, width = nlink_width)?;
    if nlink_str.len() > nlink_width {
        unsafe { NLINK_WIDTH = nlink_str.len() };
    }
    write!(stream, " ")?;

    // Print owner
    let user_name = get_user_name(statp.uid());
    let owner_width = unsafe { OWNER_WIDTH };
    if let Some(user) = user_name {
        let user_len = user.chars().count();
        if user_len > owner_width {
            unsafe { OWNER_WIDTH = user_len };
        }
        write!(stream, "{:<width$} ", user, width = owner_width)?;
    } else {
        let uid_str = format!("{}", statp.uid());
        write!(stream, "{:<width$} ", uid_str, width = owner_width)?;
        if uid_str.len() > owner_width {
            unsafe { OWNER_WIDTH = uid_str.len() };
        }
    }

    // Print group
    let group_name = get_group_name(statp.gid());
    let group_width = unsafe { GROUP_WIDTH };
    if let Some(group) = group_name {
        let group_len = group.chars().count();
        if group_len > group_width {
            unsafe { GROUP_WIDTH = group_len };
        }
        write!(stream, "{:<width$} ", group, width = group_width)?;
    } else {
        let gid_str = format!("{}", statp.gid());
        write!(stream, "{:<width$} ", gid_str, width = group_width)?;
        if gid_str.len() > group_width {
            unsafe { GROUP_WIDTH = gid_str.len() };
        }
    }

    // Print size or device numbers
    if statp.file_type().is_char_device() || statp.file_type().is_block_device() {
        let rdev = statp.rdev();
        let major = unsafe { libc::major(rdev) };
        let minor = unsafe { libc::minor(rdev) };
        
        let major_width = unsafe { MAJOR_DEVICE_NUMBER_WIDTH };
        write!(stream, "{:width$}", major, width = major_width)?;
        if major.to_string().len() > major_width {
            unsafe { MAJOR_DEVICE_NUMBER_WIDTH = major.to_string().len() };
        }
        write!(stream, ", ")?;
        
        let minor_width = unsafe { MINOR_DEVICE_NUMBER_WIDTH };
        write!(stream, "{:width$}", minor, width = minor_width)?;
        if minor.to_string().len() > minor_width {
            unsafe { MINOR_DEVICE_NUMBER_WIDTH = minor.to_string().len() };
        }
    } else {
        let size_str = format!("{}", statp.size());
        let size_width = unsafe { FILE_SIZE_WIDTH };
        write!(stream, "{:width$}", size_str, width = size_width)?;
        if size_str.len() > size_width {
            unsafe { FILE_SIZE_WIDTH = size_str.len() };
        }
    }
    write!(stream, " ")?;

    // Print time
    let mtime = statp.mtime();
    let time_str = if let Ok(tm) = time::at(time::Timespec::new(mtime, 0)).to_local() {
        let fmt = if current_time - 6 * 30 * 24 * 60 * 60 <= mtime && mtime <= current_time + 60 * 60 {
            "%b %e %H:%M"
        } else {
            "%b %e  %Y"
        };
        tm.strftime(fmt).unwrap().to_string()
    } else {
        format!("{}", mtime)
    };
    write!(stream, "{} ", time_str)?;

    // Print name
    print_name(name, stream, literal_control_chars)?;

    // Handle symlinks
    if statp.file_type().is_symlink() {
        let linkname = std::fs::read_link(Path::new(relname))?;
        write!(stream, " -> ")?;
        print_name(linkname.to_str().unwrap(), stream, literal_control_chars)?;
    }

    writeln!(stream)?;
    Ok(())
}
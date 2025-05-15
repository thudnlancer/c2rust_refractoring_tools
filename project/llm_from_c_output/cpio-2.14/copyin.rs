use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    os::unix::fs::{symlink, PermissionsExt},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use libc::{dev_t, gid_t, major, minor, mode_t, makedev, uid_t};
use nix::{
    sys::stat::{mknod, Mode, SFlag},
    unistd::{chown, lchown},
};
use thiserror::Error;

#[derive(Debug, Error)]
enum CpioError {
    #[error("I/O error")]
    Io(#[from] io::Error),
    #[error("Nix error")]
    Nix(#[from] nix::Error),
    #[error("Invalid header")]
    InvalidHeader,
    #[error("Invalid file type")]
    InvalidFileType,
    #[error("Path error")]
    PathError,
}

struct CpioFileStat {
    c_magic: u32,
    c_ino: u64,
    c_mode: mode_t,
    c_uid: uid_t,
    c_gid: gid_t,
    c_nlink: u32,
    c_mtime: i64,
    c_filesize: u64,
    c_dev_maj: u32,
    c_dev_min: u32,
    c_rdev_maj: u32,
    c_rdev_min: u32,
    c_namesize: u32,
    c_chksum: u32,
    c_name: String,
    c_tar_linkname: Option<String>,
}

impl CpioFileStat {
    fn new() -> Self {
        Self {
            c_magic: 0,
            c_ino: 0,
            c_mode: 0,
            c_uid: 0,
            c_gid: 0,
            c_nlink: 0,
            c_mtime: 0,
            c_filesize: 0,
            c_dev_maj: 0,
            c_dev_min: 0,
            c_rdev_maj: 0,
            c_rdev_min: 0,
            c_namesize: 0,
            c_chksum: 0,
            c_name: String::new(),
            c_tar_linkname: None,
        }
    }

    fn set_name(&mut self, name: &str) {
        self.c_name = name.to_string();
        self.c_namesize = name.len() as u32;
    }
}

fn warn_junk_bytes(bytes_skipped: u64) {
    eprintln!(
        "warning: skipped {} byte{} of junk",
        bytes_skipped,
        if bytes_skipped == 1 { "" } else { "s" }
    );
}

fn tape_skip_padding(in_file: &mut File, offset: u64) -> Result<(), CpioError> {
    let pad = match archive_format {
        ArchiveFormat::CrcAscii | ArchiveFormat::NewAscii => (4 - (offset % 4)) % 4,
        ArchiveFormat::Binary | ArchiveFormat::HpBinary => (2 - (offset % 2)) % 2,
        ArchiveFormat::Tar | ArchiveFormat::Ustar => (512 - (offset % 512)) % 512,
        _ => 0,
    };

    if pad != 0 {
        tape_toss_input(in_file, pad)?;
    }
    Ok(())
}

fn get_link_name(file_hdr: &CpioFileStat, in_file: &mut File) -> Result<String, CpioError> {
    if file_hdr.c_filesize > usize::MAX as u64 {
        return Err(CpioError::InvalidHeader);
    }

    let mut link_name = vec![0; file_hdr.c_filesize as usize];
    in_file.read_exact(&mut link_name)?;
    tape_skip_padding(in_file, file_hdr.c_filesize)?;

    String::from_utf8(link_name).map_err(|_| CpioError::InvalidHeader)
}

fn copyin_regular_file(file_hdr: &CpioFileStat, in_file: &mut File) -> Result<(), CpioError> {
    let out_file = if to_stdout_option {
        None
    } else {
        let mut options = OpenOptions::new();
        options.write(true).create(true).truncate(true);
        Some(options.open(&file_hdr.c_name)?)
    };

    if let Some(mut out_file) = out_file {
        let mut buffer = vec![0; 4096];
        let mut remaining = file_hdr.c_filesize;

        while remaining > 0 {
            let to_read = std::cmp::min(buffer.len(), remaining as usize);
            in_file.read_exact(&mut buffer[..to_read])?;
            out_file.write_all(&buffer[..to_read])?;
            remaining -= to_read as u64;
        }

        set_perms(&out_file, file_hdr)?;
    }

    tape_skip_padding(in_file, file_hdr.c_filesize)?;
    Ok(())
}

fn copyin_device(file_hdr: &CpioFileStat) -> Result<(), CpioError> {
    if to_stdout_option {
        return Ok(());
    }

    let dev = makedev(file_hdr.c_rdev_maj as u32, file_hdr.c_rdev_min as u32);
    mknod(
        &file_hdr.c_name,
        SFlag::from_bits_truncate(file_hdr.c_mode),
        Mode::from_bits_truncate(file_hdr.c_mode),
        dev,
    )?;

    if !no_chown_flag {
        let uid = if set_owner_flag { set_owner } else { file_hdr.c_uid };
        let gid = if set_group_flag { set_group } else { file_hdr.c_gid };
        chown(&file_hdr.c_name, Some(uid), Some(gid))?;
    }

    std::fs::set_permissions(
        &file_hdr.c_name,
        std::fs::Permissions::from_mode(file_hdr.c_mode),
    )?;

    if retain_time_flag {
        set_file_times(&file_hdr.c_name, file_hdr.c_mtime, file_hdr.c_mtime)?;
    }

    Ok(())
}

fn copyin_link(file_hdr: &CpioFileStat, in_file: &mut File) -> Result<(), CpioError> {
    let link_name = if archive_format != ArchiveFormat::Tar && archive_format != ArchiveFormat::Ustar {
        if to_stdout_option {
            tape_toss_input(in_file, file_hdr.c_filesize)?;
            tape_skip_padding(in_file, file_hdr.c_filesize)?;
            return Ok(());
        }
        get_link_name(file_hdr, in_file)?
    } else {
        if to_stdout_option {
            return Ok(());
        }
        file_hdr.c_tar_linkname.as_ref().unwrap().clone()
    };

    if no_abs_paths_flag {
        symlink_placeholder(&link_name, &file_hdr.c_name, file_hdr)?;
    } else {
        symlink(&link_name, &file_hdr.c_name)?;
        if !no_chown_flag {
            let uid = if set_owner_flag { set_owner } else { file_hdr.c_uid };
            let gid = if set_group_flag { set_group } else { file_hdr.c_gid };
            lchown(&file_hdr.c_name, Some(uid), Some(gid))?;
        }
    }

    Ok(())
}

fn process_copy_in() -> Result<(), CpioError> {
    let mut in_file = File::open(archive_path)?;
    let mut file_hdr = CpioFileStat::new();

    loop {
        read_in_header(&mut file_hdr, &mut in_file)?;

        if file_hdr.c_namesize == 0 {
            continue;
        }

        if file_hdr.c_name == CPIO_TRAILER_NAME {
            break;
        }

        let skip_file = should_skip_file(&file_hdr);

        if skip_file {
            handle_skipped_file(&file_hdr, &mut in_file)?;
        } else if table_flag {
            list_file(&file_hdr, &mut in_file)?;
        } else if append_flag {
            tape_toss_input(&mut in_file, file_hdr.c_filesize)?;
            tape_skip_padding(&mut in_file, file_hdr.c_filesize)?;
        } else {
            copyin_file(&file_hdr, &mut in_file)?;
        }
    }

    replace_symlink_placeholders()?;
    apply_delayed_set_stat()?;

    if !quiet_flag {
        print_block_count(input_bytes);
    }

    Ok(())
}

#[derive(Debug)]
enum ArchiveFormat {
    Unknown,
    NewAscii,
    CrcAscii,
    OldAscii,
    HpOldAscii,
    Binary,
    HpBinary,
    Tar,
    Ustar,
}

static mut archive_format: ArchiveFormat = ArchiveFormat::Unknown;
static mut input_bytes: u64 = 0;
static mut to_stdout_option: bool = false;
static mut no_chown_flag: bool = false;
static mut set_owner_flag: bool = false;
static mut set_owner: uid_t = 0;
static mut set_group_flag: bool = false;
static mut set_group: gid_t = 0;
static mut no_abs_paths_flag: bool = false;
static mut retain_time_flag: bool = false;
static mut table_flag: bool = false;
static mut append_flag: bool = false;
static mut quiet_flag: bool = false;
static mut create_dir_flag: bool = false;
static mut unconditional_flag: bool = false;
static mut only_verify_crc_flag: bool = false;
static mut verbose_flag: bool = false;
static mut dot_flag: bool = false;
static mut rename_flag: bool = false;
static mut swap_halfwords_flag: bool = false;
static mut swap_bytes_flag: bool = false;

const CPIO_TRAILER_NAME: &str = "TRAILER!!!";
const CP_IFMT: mode_t = 0o170000;
const CP_IFDIR: mode_t = 0o040000;
const CP_IFCHR: mode_t = 0o020000;
const CP_IFBLK: mode_t = 0o060000;
const CP_IFREG: mode_t = 0o100000;
const CP_IFLNK: mode_t = 0o120000;

fn main() -> Result<(), CpioError> {
    process_copy_in()
}
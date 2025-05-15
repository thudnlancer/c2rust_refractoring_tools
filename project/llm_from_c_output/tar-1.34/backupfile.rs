use std::ffi::{CStr, CString};
use std::fs::{DirBuilder, File, OpenOptions};
use std::io::{Error, ErrorKind, Result};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::ptr;
use std::str;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupType {
    NoBackups,
    SimpleBackups,
    NumberedExistingBackups,
    NumberedBackups,
}

impl BackupType {
    pub fn is_valid(self) -> bool {
        matches!(
            self,
            BackupType::NoBackups
                | BackupType::SimpleBackups
                | BackupType::NumberedExistingBackups
                | BackupType::NumberedBackups
        )
    }
}

static mut SIMPLE_BACKUP_SUFFIX: Option<CString> = None;

pub fn set_simple_backup_suffix(s: Option<&str>) {
    let suffix = match s {
        Some(s) if !s.is_empty() => s,
        _ => match std::env::var("SIMPLE_BACKUP_SUFFIX") {
            Ok(env_suffix) if !env_suffix.is_empty() => &env_suffix,
            _ => "~",
        },
    };

    unsafe {
        SIMPLE_BACKUP_SUFFIX = Some(CString::new(suffix).unwrap());
    }
}

pub fn get_simple_backup_suffix() -> &'static CStr {
    unsafe {
        SIMPLE_BACKUP_SUFFIX
            .as_ref()
            .map(|s| s.as_c_str())
            .unwrap_or_else(|| CStr::from_bytes_with_nul(b"~\0").unwrap())
    }
}

fn check_extension(
    file: &mut PathBuf,
    filelen: usize,
    e: char,
    dir_fd: Option<&File>,
    base_max: &mut usize,
) -> Result<()> {
    let base = file.file_name().unwrap().to_str().unwrap();
    let baselen = base.len();
    let mut baselen_max = if cfg!(feature = "long-file-names") {
        255
    } else {
        14 // POSIX_NAME_MAX
    };

    if cfg!(windows) || baselen > baselen_max {
        if *base_max == 0 {
            let name_max = match dir_fd {
                Some(dir_fd) => {
                    let fd = dir_fd.as_raw_fd();
                    unsafe { libc::fpathconf(fd, libc::_PC_NAME_MAX) }
                }
                None => {
                    let parent = file.parent().unwrap_or_else(|| Path::new("."));
                    let parent_cstr = CString::new(parent.to_str().unwrap()).unwrap();
                    unsafe { libc::pathconf(parent_cstr.as_ptr(), libc::_PC_NAME_MAX) }
                }
            };

            *base_max = if name_max >= 0 && name_max <= isize::MAX as libc::c_long {
                name_max as usize
            } else if name_max < -1 {
                14 // NAME_MAX_MINIMUM
            } else {
                usize::MAX
            };
        }

        baselen_max = *base_max;
    }

    if cfg!(windows) && baselen_max <= 12 {
        if let Some(dot) = base.find('.') {
            if let Some(second_dot) = base[dot + 1..].find('.') {
                baselen_max = dot + 1 + second_dot;
            } else {
                baselen_max = dot + 1 + 3;
            }
        } else {
            baselen_max = 8;
        }
    }

    if baselen_max < baselen {
        let mut new_path = file.to_str().unwrap().to_string();
        if baselen_max <= new_path.len() {
            new_path.truncate(baselen_max - 1);
        }
        new_path.push(e);
        *file = PathBuf::from(new_path);
    }

    Ok(())
}

#[derive(Debug)]
enum NumberedBackupResult {
    SameLength,
    Longer,
    New,
    Nomem,
}

fn numbered_backup(
    dir_fd: Option<&File>,
    buffer: &mut PathBuf,
    filelen: usize,
    base_offset: usize,
    dirp: &mut Option<std::fs::ReadDir>,
) -> Result<NumberedBackupResult> {
    let mut result = NumberedBackupResult::New;
    let mut versionlenmax = 1;
    let base = buffer.to_str().unwrap()[base_offset..].to_string();

    if let Some(dir) = dirp {
        *dir = Some(std::fs::read_dir(buffer.parent().unwrap())?);
    } else {
        let parent = buffer.parent().unwrap_or_else(|| Path::new("."));
        *dirp = Some(std::fs::read_dir(parent)?);
    }

    let dir = dirp.as_mut().unwrap();
    for entry in dir {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_str().unwrap();

        if name.len() < base.len() + 4 {
            continue;
        }

        if !name.starts_with(&base) || !name[base.len()..].starts_with(".~") {
            continue;
        }

        let p = &name[base.len() + 2..];
        if !p.starts_with(|c: char| c.is_ascii_digit() && c != '0') {
            continue;
        }

        let mut all_9s = p.starts_with('9');
        let versionlen = p
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .count();
        all_9s &= p[..versionlen].chars().all(|c| c == '9');

        if !(name.ends_with('~') || versionlenmax >= versionlen {
            continue;
        }

        versionlenmax = if all_9s { versionlen + 1 } else { versionlen };
        result = if all_9s {
            NumberedBackupResult::Longer
        } else {
            NumberedBackupResult::SameLength
        };

        let mut new_name = format!(
            "{}.~{}~",
            &buffer.to_str().unwrap()[..filelen],
            if all_9s { "0" } else { "" }
        );
        new_name.push_str(&p[..versionlen]);
        new_name.push('~');

        let mut num = p[..versionlen].parse::<u64>().unwrap() + 1;
        let mut pos = new_name.len() - 1;
        while num > 0 {
            let digit = (num % 10) as u8;
            new_name.as_mut_vec()[pos] = b'0' + digit;
            num /= 10;
            pos -= 1;
        }

        *buffer = PathBuf::from(new_name);
    }

    Ok(result)
}

pub fn backup_file_rename(
    dir_fd: Option<&File>,
    file: &str,
    backup_type: BackupType,
) -> Result<PathBuf> {
    if backup_type == BackupType::NoBackups {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "backup_type cannot be NoBackups",
        ));
    }

    let file_path = Path::new(file);
    let base_offset = file_path.file_name().unwrap().to_str().unwrap().len();
    let filelen = file.len();

    if unsafe { SIMPLE_BACKUP_SUFFIX.is_none() } {
        set_simple_backup_suffix(None);
    }

    let simple_backup_suffix = get_simple_backup_suffix().to_str().unwrap();
    let simple_backup_suffix_size = simple_backup_suffix.len() + 1;
    let mut backup_suffix_size_guess = simple_backup_suffix_size.max(8); // sizeof ".~12345~"

    let mut buffer = PathBuf::with_capacity(filelen + backup_suffix_size_guess + 1);
    buffer.push(file);

    let mut dirp = None;
    let mut sdir = None;
    let mut base_max = 0;

    loop {
        if backup_type == BackupType::SimpleBackups {
            buffer.set_extension(simple_backup_suffix);
        } else {
            match numbered_backup(dir_fd, &mut buffer, filelen, base_offset, &mut dirp)? {
                NumberedBackupResult::SameLength => {}
                NumberedBackupResult::New => {
                    if backup_type == BackupType::NumberedExistingBackups {
                        buffer.set_extension(simple_backup_suffix);
                    }
                }
                NumberedBackupResult::Longer => {
                    check_extension(&mut buffer, filelen, '~', dir_fd, &mut base_max)?;
                }
                NumberedBackupResult::Nomem => {
                    return Err(Error::new(ErrorKind::OutOfMemory, "Allocation failed"));
                }
            }
        }

        if dir_fd.is_none() {
            break;
        }

        let flags = if backup_type == BackupType::SimpleBackups {
            0
        } else {
            libc::RENAME_NOREPLACE
        };

        match unsafe {
            libc::renameat(
                libc::AT_FDCWD,
                file.as_ptr() as *const libc::c_char,
                sdir.as_ref().map_or(libc::AT_FDCWD, |f| f.as_raw_fd()),
                buffer.to_str().unwrap().as_ptr() as *const libc::c_char,
            )
        } {
            0 => break,
            _ => {
                let e = Error::last_os_error();
                if e.kind() != ErrorKind::AlreadyExists {
                    return Err(e);
                }
            }
        }
    }

    Ok(buffer)
}

pub fn find_backup_file_name(
    dir_fd: Option<&File>,
    file: &str,
    backup_type: BackupType,
) -> Result<PathBuf> {
    backup_file_rename(dir_fd, file, backup_type)
}

pub fn get_version(context: &str, arg: &str) -> BackupType {
    match arg {
        "none" => BackupType::NoBackups,
        "simple" => BackupType::SimpleBackups,
        "existing" => BackupType::NumberedExistingBackups,
        "numbered" => BackupType::NumberedBackups,
        _ => BackupType::SimpleBackups,
    }
}

pub fn xget_version(context: &str, arg: &str) -> BackupType {
    get_version(context, arg)
}
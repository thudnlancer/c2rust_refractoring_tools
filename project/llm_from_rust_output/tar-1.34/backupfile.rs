use std::env;
use std::ffi::{CString, OsStr, OsString};
use std::fs::{DirBuilder, File, OpenOptions};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupType {
    NoBackups,
    Simple,
    NumberedExisting,
    Numbered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NumberedBackupResult {
    SameLength,
    Longer,
    New,
    NoMem,
}

static mut SIMPLE_BACKUP_SUFFIX: Option<CString> = None;

pub fn set_simple_backup_suffix(s: Option<&str>) {
    let suffix = match s {
        Some(s) => CString::new(s).unwrap(),
        None => match env::var("SIMPLE_BACKUP_SUFFIX") {
            Ok(val) => CString::new(val).unwrap(),
            Err(_) => CString::new("~").unwrap(),
        },
    };

    unsafe {
        SIMPLE_BACKUP_SUFFIX = Some(suffix);
    }
}

fn get_simple_backup_suffix() -> CString {
    unsafe {
        if SIMPLE_BACKUP_SUFFIX.is_none() {
            set_simple_backup_suffix(None);
        }
        SIMPLE_BACKUP_SUFFIX.as_ref().unwrap().clone()
    }
}

fn check_extension(
    file: &mut PathBuf,
    filelen: usize,
    e: char,
    dir_fd: Option<&File>,
    base_max: &mut usize,
) -> io::Result<()> {
    let base = file.file_name().unwrap().to_os_string();
    let baselen = base.len();
    let mut baselen_max = 255;

    if *base_max == 0 {
        let name_max = if let Some(dir_fd) = dir_fd {
            dir_fd.pathconf(libc::_PC_NAME_MAX)?
        } else {
            Path::new(".").pathconf(libc::_PC_NAME_MAX)?
        };

        *base_max = match name_max {
            Some(n) if n > 0 => n as usize,
            _ => 255,
        };
    }

    baselen_max = *base_max;

    if baselen_max < baselen {
        let new_len = if baselen_max <= filelen {
            baselen_max - 1
        } else {
            filelen
        };

        let mut new_name = file.file_name().unwrap().to_os_string();
        new_name.push(e.to_string());
        file.set_file_name(new_name);
    }

    Ok(())
}

fn numbered_backup(
    dir_fd: Option<&File>,
    buffer: &mut PathBuf,
    filelen: usize,
    base_offset: isize,
    dirp: &mut Option<DirBuilder>,
) -> io::Result<NumberedBackupResult> {
    let mut result = NumberedBackupResult::New;
    let mut versionlenmax = 1;
    let base = buffer.file_name().unwrap();
    let baselen = base.len();

    if dirp.is_none() {
        let mut tmp = PathBuf::new();
        tmp.push(".");
        *dirp = Some(DirBuilder::new());
        dirp.as_mut().unwrap().create(&tmp)?;
    }

    let dir = dirp.as_ref().unwrap().open(&buffer)?;
    for entry in dir.read_dir()? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_str().unwrap();

        if name.len() < baselen + 4 {
            continue;
        }

        if !name.starts_with(&base.to_str().unwrap()) {
            continue;
        }

        let p = &name[baselen + 2..];
        if !p.chars().next().map_or(false, |c| c.is_digit(10)) {
            continue;
        }

        let all_9s = p.chars().all(|c| c == '9');
        let versionlen = p.chars().take_while(|c| c.is_digit(10)).count();

        if !(p.ends_with('~') && (versionlenmax < versionlen || (versionlenmax == versionlen && p <= &buffer.to_str().unwrap()[filelen + 2..]))) {
            continue;
        }

        versionlenmax = if all_9s { versionlen + 1 } else { versionlen };
        result = if all_9s {
            NumberedBackupResult::Longer
        } else {
            NumberedBackupResult::SameLength
        };

        let mut new_name = buffer.to_str().unwrap()[..filelen].to_string();
        new_name.push_str(".~");
        new_name.push_str(&"0".repeat(all_9s as usize));
        new_name.push_str(p);
        new_name.push('~');

        let mut q = new_name.len() - 1;
        while q > filelen + 2 && new_name.chars().nth(q).unwrap() == '9' {
            new_name.replace_range(q..q+1, "0");
            q -= 1;
        }

        if q > filelen + 2 {
            let c = new_name.chars().nth(q).unwrap();
            new_name.replace_range(q..q+1, &(c as u8 + 1).to_string());
        }

        *buffer = PathBuf::from(new_name);
    }

    Ok(result)
}

pub fn backupfile_internal(
    dir_fd: Option<&File>,
    file: &Path,
    backup_type: BackupType,
    rename: bool,
) -> io::Result<PathBuf> {
    let base_offset = file.file_name().unwrap().len() as isize;
    let filelen = file.to_str().unwrap().len();

    let simple_backup_suffix = get_simple_backup_suffix();
    let simple_backup_suffix_size = simple_backup_suffix.as_bytes().len() + 1;
    let mut backup_suffix_size_guess = simple_backup_suffix_size.max(9);

    let mut s = PathBuf::from(file);
    let mut dirp = None;
    let mut sdir = None;
    let mut base_max = 0;

    loop {
        if backup_type == BackupType::Simple {
            s.set_extension(simple_backup_suffix.to_str().unwrap());
        } else {
            match numbered_backup(dir_fd, &mut s, filelen, base_offset, &mut dirp)? {
                NumberedBackupResult::New => {
                    if backup_type == BackupType::NumberedExisting {
                        s.set_extension(simple_backup_suffix.to_str().unwrap());
                    }
                }
                NumberedBackupResult::NoMem => {
                    return Err(io::Error::new(io::ErrorKind::OutOfMemory, "Out of memory"));
                }
                _ => {}
            }

            check_extension(&mut s, filelen, '~', sdir.as_ref(), &mut base_max)?;
        }

        if !rename {
            break;
        }

        if sdir.is_none() {
            sdir = Some(File::open(".")?;
            base_offset = 0;
        }

        let flags = if backup_type == BackupType::Simple {
            0
        } else {
            libc::RENAME_NOREPLACE
        };

        match nix::fcntl::renameat(
            None,
            file,
            sdir.as_ref(),
            &s,
            nix::fcntl::RenameFlags::from_bits(flags).unwrap(),
        ) {
            Ok(_) => break,
            Err(e) if e != nix::Error::EEXIST => return Err(e.into()),
            _ => continue,
        }
    }

    Ok(s)
}
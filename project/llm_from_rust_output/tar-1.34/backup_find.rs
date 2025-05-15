use std::ffi::{CStr, CString};
use std::os::unix::io::RawFd;
use std::ptr;
use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupType {
    NoBackups,
    SimpleBackups,
    NumberedExistingBackups,
    NumberedBackups,
}

impl From<BackupType> for u32 {
    fn from(bt: BackupType) -> u32 {
        match bt {
            BackupType::NoBackups => 0,
            BackupType::SimpleBackups => 1,
            BackupType::NumberedExistingBackups => 2,
            BackupType::NumberedBackups => 3,
        }
    }
}

impl From<u32> for BackupType {
    fn from(val: u32) -> Self {
        match val {
            0 => BackupType::NoBackups,
            1 => BackupType::SimpleBackups,
            2 => BackupType::NumberedExistingBackups,
            3 => BackupType::NumberedBackups,
            _ => panic!("Invalid backup type value"),
        }
    }
}

extern "C" {
    fn backupfile_internal(
        dir_fd: RawFd,
        file: *const libc::c_char,
        backup_type: u32,
        _: bool,
    ) -> *mut libc::c_char;
}

pub fn find_backup_file_name(dir_fd: RawFd, file: &CStr, backup_type: BackupType) -> CString {
    unsafe {
        let result = backupfile_internal(dir_fd, file.as_ptr(), backup_type.into(), false);
        if result.is_null() {
            panic!("Failed to allocate backup file name");
        }
        CString::from_raw(result)
    }
}

const BACKUP_ARGS: &[&str] = &[
    "none", "off", "simple", "never", "existing", "nil", "numbered", "t",
];

const BACKUP_TYPES: [BackupType; 8] = [
    BackupType::NoBackups,
    BackupType::NoBackups,
    BackupType::SimpleBackups,
    BackupType::SimpleBackups,
    BackupType::NumberedExistingBackups,
    BackupType::NumberedExistingBackups,
    BackupType::NumberedBackups,
    BackupType::NumberedBackups,
];

pub fn get_version(context: &str, version: Option<&str>) -> BackupType {
    let version = match version {
        Some(v) if !v.is_empty() => v,
        _ => return BackupType::NumberedExistingBackups,
    };

    let index = BACKUP_ARGS
        .iter()
        .position(|&arg| arg == version)
        .unwrap_or_else(|| panic!("Invalid backup version '{}' in context '{}'", version, context));

    BACKUP_TYPES[index]
}

pub fn xget_version(context: &str, version: Option<&str>) -> BackupType {
    if let Some(v) = version {
        if !v.is_empty() {
            return get_version(context, Some(v));
        }
    }

    let env_var = env::var("VERSION_CONTROL").unwrap_or_default();
    get_version("$VERSION_CONTROL", Some(&env_var))
}
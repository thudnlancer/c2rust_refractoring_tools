use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::io;

#[derive(Clone, Copy)]
pub struct PermissionContext {
    pub mode: u32,
}

pub fn chmod_or_fchmod(name: Option<&std::ffi::CStr>, desc: Option<std::os::unix::io::RawFd>, mode: u32) -> io::Result<()> {
    if let Some(fd) = desc {
        fs::set_permissions(fd, fs::Permissions::from_mode(mode))
    } else if let Some(path) = name {
        fs::set_permissions(path.to_str().ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Invalid path"))?, fs::Permissions::from_mode(mode))
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Neither file descriptor nor path provided"))
    }
}

pub fn set_permissions(ctx: &PermissionContext, name: Option<&std::ffi::CStr>, desc: Option<std::os::unix::io::RawFd>) -> io::Result<()> {
    let acls_set = false;
    let early_chmod = true;
    let must_chmod = false;

    if early_chmod {
        chmod_or_fchmod(name, desc, ctx.mode)?;
    }

    if must_chmod && !early_chmod {
        let saved_errno = io::Error::last_os_error();
        chmod_or_fchmod(name, desc, ctx.mode)?;
        if saved_errno.raw_os_error().unwrap_or(0) != 0 {
            return Err(saved_errno);
        }
    }

    Ok(())
}
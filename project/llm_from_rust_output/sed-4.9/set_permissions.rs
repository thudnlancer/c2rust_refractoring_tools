use std::ffi::CString;
use std::os::unix::fs::PermissionsExt;
use std::fs::{File, set_permissions, Permissions};
use std::io;

#[derive(Clone, Copy)]
pub struct PermissionContext {
    pub mode: u32,
}

pub fn chmod_or_fchmod(name: Option<&CString>, desc: Option<&File>, mode: u32) -> io::Result<()> {
    if let Some(file) = desc {
        file.set_permissions(Permissions::from_mode(mode))
    } else if let Some(path) = name {
        set_permissions(path.as_c_str(), Permissions::from_mode(mode))
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Neither file descriptor nor path provided"))
    }
}

pub fn set_permissions(ctx: &PermissionContext, name: Option<&CString>, desc: Option<&File>) -> io::Result<()> {
    let acls_set = false;
    let early_chmod = true;
    let must_chmod = false;

    if early_chmod {
        chmod_or_fchmod(name, desc, ctx.mode)?;
    }

    if must_chmod && !early_chmod {
        chmod_or_fchmod(name, desc, ctx.mode)?;
    }

    Ok(())
}
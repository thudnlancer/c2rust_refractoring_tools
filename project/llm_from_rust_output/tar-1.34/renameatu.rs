use std::ffi::CString;
use std::fs::{metadata, symlink_metadata};
use std::io;
use std::os::unix::fs::{MetadataExt, FileTypeExt};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum RenameError {
    InvalidFlags,
    DestinationExists,
    SourceNotFound,
    DestinationNotFound,
    NotDirectory,
    IsDirectory,
    Other(io::Error),
}

impl From<io::Error> for RenameError {
    fn from(err: io::Error) -> Self {
        RenameError::Other(err)
    }
}

pub fn renameatu(
    fd1: Option<&Path>,
    src: &Path,
    fd2: Option<&Path>,
    dst: &Path,
    flags: u32,
) -> Result<(), RenameError> {
    if flags & !0x1 != 0 {
        return Err(RenameError::InvalidFlags);
    }

    let no_replace = flags & 0x1 != 0;

    // Check destination existence if NO_REPLACE flag is set
    if no_replace {
        match fd2 {
            Some(base) => {
                let full_dst = base.join(dst);
                if symlink_metadata(&full_dst).is_ok() {
                    return Err(RenameError::DestinationExists);
                }
            }
            None => {
                if symlink_metadata(dst).is_ok() {
                    return Err(RenameError::DestinationExists);
                }
            }
        }
    }

    // Get source metadata
    let src_metadata = match fd1 {
        Some(base) => symlink_metadata(base.join(src))?,
        None => symlink_metadata(src)?,
    };

    // Check if destination exists (when NO_REPLACE not set)
    if !no_replace {
        match fd2 {
            Some(base) => {
                let full_dst = base.join(dst);
                if let Ok(dst_metadata) = symlink_metadata(&full_dst) {
                    if !dst_metadata.file_type().is_dir() {
                        return Err(RenameError::NotDirectory);
                    }
                    if !src_metadata.file_type().is_dir() {
                        return Err(RenameError::IsDirectory);
                    }
                }
            }
            None => {
                if let Ok(dst_metadata) = symlink_metadata(dst) {
                    if !dst_metadata.file_type().is_dir() {
                        return Err(RenameError::NotDirectory);
                    }
                    if !src_metadata.file_type().is_dir() {
                        return Err(RenameError::IsDirectory);
                    }
                }
            }
        }
    }

    // Handle trailing slashes
    let src_str = src.to_string_lossy().into_owned();
    let dst_str = dst.to_string_lossy().into_owned();

    let src_has_slash = src_str.ends_with('/');
    let dst_has_slash = dst_str.ends_with('/');

    if src_has_slash || dst_has_slash {
        if !src_metadata.file_type().is_dir() {
            return Err(RenameError::NotDirectory);
        }
    }

    // Perform the actual rename
    let src_path = if let Some(base) = fd1 {
        base.join(src)
    } else {
        src.to_path_buf()
    };

    let dst_path = if let Some(base) = fd2 {
        base.join(dst)
    } else {
        dst.to_path_buf()
    };

    std::fs::rename(&src_path, &dst_path)?;

    Ok(())
}
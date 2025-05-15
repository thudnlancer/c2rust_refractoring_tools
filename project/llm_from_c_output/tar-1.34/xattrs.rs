//! Support for extended attributes in Rust

use std::ffi::{CStr, CString};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr;
use std::slice;
use std::sync::Once;

use libc::{c_char, c_int, c_void, size_t};
use nix::errno::Errno;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::close;

#[cfg(feature = "acl")]
use acl::Acl;

#[cfg(feature = "selinux")]
use selinux::{getfilecon, setfilecon};

mod error;
use error::{TarError, TarResult};

/// Structure to hold xattr mask patterns
struct XattrsMaskMap {
    masks: Vec<String>,
}

impl XattrsMaskMap {
    fn new() -> Self {
        XattrsMaskMap { masks: Vec::new() }
    }

    fn add(&mut self, mask: &str) {
        self.masks.push(mask.to_string());
    }

    fn clear(&mut self) {
        self.masks.clear();
    }

    fn matches(&self, keyword: &str) -> bool {
        self.masks.iter().any(|m| {
            fnmatch::fnmatch(m, keyword).unwrap_or(false)
        })
    }
}

/// Global setup for xattrs
struct XattrsSetup {
    include: XattrsMaskMap,
    exclude: XattrsMaskMap,
}

impl XattrsSetup {
    fn new() -> Self {
        XattrsSetup {
            include: XattrsMaskMap::new(),
            exclude: XattrsMaskMap::new(),
        }
    }

    fn clear(&mut self) {
        self.include.clear();
        self.exclude.clear();
    }
}

lazy_static::lazy_static! {
    static ref XATTRS_SETUP: std::sync::Mutex<XattrsSetup> = 
        std::sync::Mutex::new(XattrsSetup::new());
}

/// Add include/exclude fnmatch pattern for xattr key domain
pub fn xattrs_mask_add(mask: &str, include: bool) {
    let mut setup = XATTRS_SETUP.lock().unwrap();
    if include {
        setup.include.add(mask);
    } else {
        setup.exclude.add(mask);
    }
}

/// Clear xattr setup when tar finishes
pub fn xattrs_clear_setup() {
    XATTRS_SETUP.lock().unwrap().clear();
}

/// Check if xattr should be masked out
fn xattrs_masked_out(keyword: &str, archiving: bool) -> bool {
    let setup = XATTRS_SETUP.lock().unwrap();
    
    let included = if setup.include.masks.is_empty() {
        archiving
    } else {
        setup.include.matches(keyword)
    };
    
    let excluded = if setup.exclude.masks.is_empty() {
        false
    } else {
        setup.exclude.matches(keyword)
    };
    
    if included {
        excluded
    } else {
        true
    }
}

/// Get ACLs for file
#[cfg(feature = "acl")]
pub fn xattrs_acls_get(
    parentfd: Option<i32>,
    file_name: &Path,
    st: &mut TarStatInfo,
    fd: Option<i32>,
    is_file: bool,
) -> TarResult<()> {
    // Implementation using acl crate
    Ok(())
}

/// Set ACLs for file
#[cfg(feature = "acl")]
pub fn xattrs_acls_set(
    st: &TarStatInfo,
    file_name: &Path,
    typeflag: char,
) -> TarResult<()> {
    // Implementation using acl crate
    Ok(())
}

/// Get SELinux context for file
#[cfg(feature = "selinux")]
pub fn xattrs_selinux_get(
    parentfd: Option<i32>,
    file_name: &Path,
    st: &mut TarStatInfo,
    fd: Option<i32>,
) -> TarResult<()> {
    let context = if let Some(fd) = fd {
        getfilecon(fd)?
    } else {
        let path = CString::new(file_name.as_os_str().as_bytes())?;
        unsafe { getfilecon(path.as_ptr())? }
    };
    
    st.cntx_name = Some(context.to_string_lossy().into_owned());
    Ok(())
}

/// Set SELinux context for file
#[cfg(feature = "selinux")]
pub fn xattrs_selinux_set(
    st: &TarStatInfo,
    file_name: &Path,
    typeflag: char,
) -> TarResult<()> {
    if let Some(ref context) = st.cntx_name {
        let c_context = CString::new(context.as_bytes())?;
        let path = CString::new(file_name.as_os_str().as_bytes())?;
        
        if typeflag != 'l' {
            unsafe { setfilecon(path.as_ptr(), c_context.as_ptr())? };
        } else {
            unsafe { lsetfilecon(path.as_ptr(), c_context.as_ptr())? };
        }
    }
    Ok(())
}

/// Get extended attributes for file
pub fn xattrs_xattrs_get(
    parentfd: Option<i32>,
    file_name: &Path,
    st: &mut TarStatInfo,
    fd: Option<i32>,
) -> TarResult<()> {
    // Implementation using xattr crate
    Ok(())
}

/// Set extended attributes for file
pub fn xattrs_xattrs_set(
    st: &TarStatInfo,
    file_name: &Path,
    typeflag: char,
    later_run: bool,
) -> TarResult<()> {
    // Implementation using xattr crate
    Ok(())
}

/// Print xattr information
pub fn xattrs_print(st: &TarStatInfo) {
    // Implementation
}

/// Print single character xattr indicator
pub fn xattrs_print_char(st: &TarStatInfo) -> char {
    // Implementation
    ' '
}

/// Tar file stat information
pub struct TarStatInfo {
    pub cntx_name: Option<String>,
    pub acls_a_ptr: Option<String>,
    pub acls_a_len: usize,
    pub acls_d_ptr: Option<String>,
    pub acls_d_len: usize,
    pub xattr_map: Vec<XattrEntry>,
    pub xattr_map_size: usize,
}

impl Default for TarStatInfo {
    fn default() -> Self {
        TarStatInfo {
            cntx_name: None,
            acls_a_ptr: None,
            acls_a_len: 0,
            acls_d_ptr: None,
            acls_d_len: 0,
            xattr_map: Vec::new(),
            xattr_map_size: 0,
        }
    }
}

/// Extended attribute entry
pub struct XattrEntry {
    pub xkey: String,
    pub xval_ptr: Vec<u8>,
    pub xval_len: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mask_add() {
        xattrs_mask_add("user.*", true);
        xattrs_mask_add("security.*", false);
        
        let setup = XATTRS_SETUP.lock().unwrap();
        assert!(setup.include.matches("user.test"));
        assert!(setup.exclude.matches("security.test"));
    }
}